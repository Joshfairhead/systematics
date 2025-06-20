use crate::error::SystematicsError;
use crate::SystematicStructure;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use uuid::Uuid;

use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoredStructure {
    pub id: Thing,
    pub name: String,
    pub structure_type: String,
    pub terms: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub description: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraphNode {
    pub id: Thing,
    pub structure_id: String,
    pub position: usize,
    pub term: String,
    pub created_at: Datetime,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraphEdge {
    pub id: Thing,
    pub from_node: String,
    pub to_node: String,
    pub relationship_type: String,
    pub weight: f64,
    pub created_at: Datetime,
}

#[derive(Clone)]
pub struct SurrealStorage {
    db: Surreal<Db>,
}

impl SurrealStorage {
    pub async fn new(db_path: &str) -> Result<Self, SystematicsError> {
        // Use file-based storage for persistence
        let db = Surreal::new::<RocksDb>(db_path).await?;
        
        // Use a namespace and database
        db.use_ns("systematics").use_db("structures").await?;
        
        // Initialize schema
        Self::init_schema(&db).await?;
        
        Ok(Self { db })
    }

    /// Create a new SurrealStorage with default project data path
    pub async fn new_default() -> Result<Self, SystematicsError> {
        let default_path = std::env::var("SYSTEMATICS_DB_PATH")
            .unwrap_or_else(|_| "../data/systematics.db".to_string());
        Self::new(&default_path).await
    }

    async fn init_schema(db: &Surreal<Db>) -> Result<(), SystematicsError> {
        // Create tables and indexes - using SCHEMALESS to avoid type conflicts
        db.query("
            DEFINE TABLE structures SCHEMALESS;
            DEFINE INDEX idx_name ON structures COLUMNS name;
            DEFINE INDEX idx_type ON structures COLUMNS structure_type;
            DEFINE INDEX idx_created ON structures COLUMNS created_at;
        ").await?;

        db.query("
            DEFINE TABLE nodes SCHEMAFULL;
            DEFINE FIELD id ON nodes TYPE string;
            DEFINE FIELD structure_id ON nodes TYPE string;
            DEFINE FIELD position ON nodes TYPE int;
            DEFINE FIELD term ON nodes TYPE string;
            DEFINE FIELD created_at ON nodes TYPE datetime;
            DEFINE INDEX idx_structure ON nodes COLUMNS structure_id;
            DEFINE INDEX idx_term ON nodes COLUMNS term;
        ").await?;

        db.query("
            DEFINE TABLE edges SCHEMAFULL;
            DEFINE FIELD id ON edges TYPE string;
            DEFINE FIELD from_node ON edges TYPE string;
            DEFINE FIELD to_node ON edges TYPE string;
            DEFINE FIELD relationship_type ON edges TYPE string;
            DEFINE FIELD weight ON edges TYPE float;
            DEFINE FIELD created_at ON edges TYPE datetime;
            DEFINE INDEX idx_from ON edges COLUMNS from_node;
            DEFINE INDEX idx_to ON edges COLUMNS to_node;
            DEFINE INDEX idx_relationship ON edges COLUMNS relationship_type;
        ").await?;

        Ok(())
    }

    pub async fn store_structure<T: SystematicStructure>(
        &self,
        structure: &T,
        name: &str,
        description: Option<&str>,
    ) -> Result<String, SystematicsError> {
        let id_string = Uuid::new_v4().to_string();
        let now = Datetime::default();
        
        // Convert connectives from (usize, usize) keys to string keys for storage
        let connectives: HashMap<String, String> = structure.connectives_traits()
            .iter()
            .map(|((from, to), relationship)| {
                (format!("{}:{}", from, to), relationship.clone())
            })
            .collect();
        
        let stored_structure = StoredStructure {
            id: Thing::from(("structures", id_string.as_str())),
            name: name.to_string(),
            structure_type: structure.structure_type().to_string(),
            terms: structure.user_instance_index().to_vec(),
            connectives,
            created_at: now.clone(),
            updated_at: now,
            description: description.map(|s| s.to_string()),
            metadata: HashMap::new(),
        };

        // Store the structure
        let _: Option<StoredStructure> = self.db
            .create(("structures", id_string.as_str()))
            .content(stored_structure)
            .await?;

        // Store nodes and create graph representation
        let nodes = self.create_nodes(&id_string, structure.user_instance_index()).await?;
        let _edges = self.create_edges(&nodes).await?;

        Ok(id_string)
    }

    async fn create_nodes(&self, structure_id: &str, terms: &[String]) -> Result<Vec<GraphNode>, SystematicsError> {
        let mut nodes = Vec::new();
        let now = Datetime::default();

        for (position, term) in terms.iter().enumerate() {
            let node_id = format!("{}_{}", structure_id, position);
            let node = GraphNode {
                id: Thing::from(("nodes", node_id.as_str())),
                structure_id: structure_id.to_string(),
                position,
                term: term.clone(),
                created_at: now.clone(),
            };

            let _: Option<GraphNode> = self.db
                .create(("nodes", node_id.as_str()))
                .content(node.clone())
                .await?;

            nodes.push(node);
        }

        Ok(nodes)
    }

    async fn create_edges(&self, nodes: &[GraphNode]) -> Result<Vec<GraphEdge>, SystematicsError> {
        let mut edges = Vec::new();
        let now = Datetime::default();

        // Create sequential edges between adjacent terms
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let from_id = nodes[i].id.id.to_string();
                let to_id = nodes[j].id.id.to_string();
                let edge_id = format!("{}_{}", from_id, to_id);
                let edge = GraphEdge {
                    id: Thing::from(("edges", edge_id.as_str())),
                    from_node: from_id,
                    to_node: to_id,
                    relationship_type: "sequence".to_string(),
                    weight: 1.0 / (j - i) as f64, // Higher weight for closer terms
                    created_at: now.clone(),
                };

                let _: Option<GraphEdge> = self.db
                    .create(("edges", edge_id.as_str()))
                    .content(edge.clone())
                    .await?;

                edges.push(edge);
            }
        }

        Ok(edges)
    }

    pub async fn get_structure(&self, id: &str) -> Result<Option<StoredStructure>, SystematicsError> {
        let structure: Option<StoredStructure> = self.db.select(("structures", id)).await?;
        Ok(structure)
    }

    pub async fn list_structures(&self) -> Result<Vec<StoredStructure>, SystematicsError> {
        let structures: Vec<StoredStructure> = self.db.select("structures").await?;
        Ok(structures)
    }

    pub async fn search_structures(&self, query: &str) -> Result<Vec<StoredStructure>, SystematicsError> {
        let sql = "
            SELECT * FROM structures 
            WHERE name CONTAINS $query 
            OR description CONTAINS $query 
            OR array::some(terms, |$term| $term CONTAINS $query)
            ORDER BY created_at DESC
        ";
        
        let query_string = query.to_string();
        let mut result = self.db.query(sql).bind(("query", query_string)).await?;
        let structures: Vec<StoredStructure> = result.take(0)?;
        
        Ok(structures)
    }

    pub async fn get_related_structures(&self, id: &str) -> Result<Vec<StoredStructure>, SystematicsError> {
        // Find structures that share terms with the given structure
        let sql = "
            SELECT DISTINCT s2.* FROM structures s1, structures s2
            WHERE s1.id = $id 
            AND s2.id != $id
            AND array::intersect(s1.terms, s2.terms) != []
            ORDER BY array::len(array::intersect(s1.terms, s2.terms)) DESC
        ";
        
        let id_string = id.to_string();
        let mut result = self.db.query(sql).bind(("id", id_string)).await?;
        let structures: Vec<StoredStructure> = result.take(0)?;
        
        Ok(structures)
    }

    pub async fn get_structure_graph(&self, id: &str) -> Result<(Vec<GraphNode>, Vec<GraphEdge>), SystematicsError> {
        // Get nodes for this structure
        let nodes_sql = "SELECT * FROM nodes WHERE structure_id = $id ORDER BY position";
        let id_string = id.to_string();
        let mut nodes_result = self.db.query(nodes_sql).bind(("id", id_string.clone())).await?;
        let nodes: Vec<GraphNode> = nodes_result.take(0)?;

        // Get edges for these nodes
        let node_ids: Vec<String> = nodes.iter().map(|n| n.id.id.to_string()).collect();
        let edges_sql = "SELECT * FROM edges WHERE from_node IN $node_ids OR to_node IN $node_ids";
        let mut edges_result = self.db.query(edges_sql).bind(("node_ids", node_ids)).await?;
        let edges: Vec<GraphEdge> = edges_result.take(0)?;

        Ok((nodes, edges))
    }

    pub async fn delete_structure(&self, id: &str) -> Result<bool, SystematicsError> {
        // Delete associated nodes and edges first
        let nodes_sql = "DELETE FROM nodes WHERE structure_id = $id";
        let id_string = id.to_string();
        self.db.query(nodes_sql).bind(("id", id_string.clone())).await?;

        // Delete edges (they should be cleaned up by the node deletion, but let's be explicit)
        let edges_sql = "DELETE FROM edges WHERE from_node LIKE $pattern OR to_node LIKE $pattern";
        let pattern = format!("{}_%", id);
        self.db.query(edges_sql).bind(("pattern", pattern)).await?;

        // Delete the structure itself
        let deleted: Option<StoredStructure> = self.db.delete(("structures", id)).await?;
        
        Ok(deleted.is_some())
    }

    pub async fn update_structure_metadata(
        &self,
        id: &str,
        metadata: HashMap<String, String>,
    ) -> Result<bool, SystematicsError> {
        let sql = "UPDATE structures SET metadata = $metadata, updated_at = $now WHERE id = $id";
        let now = Datetime::default();
        let id_string = id.to_string();
        
        let mut result = self.db
            .query(sql)
            .bind(("id", id_string))
            .bind(("metadata", metadata))
            .bind(("now", now))
            .await?;
            
        let updated: Option<Vec<StoredStructure>> = result.take(0)?;
        
        Ok(updated.is_some() && !updated.unwrap().is_empty())
    }

    pub async fn get_term_usage(&self, term: &str) -> Result<Vec<StoredStructure>, SystematicsError> {
        let sql = "
            SELECT * FROM structures 
            WHERE array::some(terms, |$t| $t = $term)
            ORDER BY created_at DESC
        ";
        
        let term_string = term.to_string();
        let mut result = self.db.query(sql).bind(("term", term_string)).await?;
        let structures: Vec<StoredStructure> = result.take(0)?;
        
        Ok(structures)
    }

    /// Store a structure directly from API request data
    pub async fn store_structure_direct(
        &self,
        name: &str,
        structure_type: &str,
        terms: Vec<String>,
        connectives: HashMap<String, String>,
        description: Option<String>,
    ) -> Result<String, SystematicsError> {
        let id_string = Uuid::new_v4().to_string();
        let now = Datetime::default();
        
        let stored_structure = StoredStructure {
            id: Thing::from(("structures", id_string.as_str())),
            name: name.to_string(),
            structure_type: structure_type.to_string(),
            terms: terms.clone(),
            connectives,
            created_at: now.clone(),
            updated_at: now,
            description,
            metadata: HashMap::new(),
        };

        // Store the structure
        let _: Option<StoredStructure> = self.db
            .create(("structures", id_string.as_str()))
            .content(stored_structure)
            .await?;

        // Store nodes and create graph representation
        let nodes = self.create_nodes(&id_string, &terms).await?;
        let _edges = self.create_edges(&nodes).await?;

        Ok(id_string)
    }
}

// Convert SurrealDB errors to our error type
impl From<surrealdb::Error> for SystematicsError {
    fn from(err: surrealdb::Error) -> Self {
        SystematicsError::Storage(format!("SurrealDB error: {}", err))
    }
} 