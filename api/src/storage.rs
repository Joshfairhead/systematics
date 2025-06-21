use crate::error::SystematicsError;
use crate::SystematicStructure;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use uuid::Uuid;

use surrealdb::sql::{Datetime, Thing};
use serde_json;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoredUserInstance {
    pub id: Thing,
    pub name: String,
    pub structure_type: String,
    pub grammar_id: String,
    pub instances: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub description: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoredCommunityGrammar {
    pub id: Thing,
    pub structure_type: String,
    pub name: String,
    pub term_characters: Vec<String>,
    pub author: String,
    pub mapping_notes: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub description: Option<String>,
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
        db.use_ns("systematics").use_db("user_instances").await?;
        
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
            DEFINE TABLE user_instances SCHEMALESS;
            DEFINE INDEX idx_name ON user_instances COLUMNS name;
            DEFINE INDEX idx_type ON user_instances COLUMNS structure_type;
            DEFINE INDEX idx_created ON user_instances COLUMNS created_at;
        ").await?;

        db.query("
            DEFINE TABLE community_grammars SCHEMALESS;
            DEFINE INDEX idx_grammar_name ON community_grammars COLUMNS name;
            DEFINE INDEX idx_grammar_type ON community_grammars COLUMNS structure_type;
            DEFINE INDEX idx_grammar_author ON community_grammars COLUMNS author;
            DEFINE INDEX idx_grammar_created ON community_grammars COLUMNS created_at;
        ").await?;

        // Migrate existing records from old table names and field names
        // First, check if migration is needed from old 'definitions' table
        let check_definitions_sql = "SELECT * FROM definitions LIMIT 1";
        if let Ok(mut result) = db.query(check_definitions_sql).await {
            if let Ok(definitions) = result.take::<Vec<serde_json::Value>>(0) {
                if !definitions.is_empty() {
                    eprintln!("🔄 Migrating from 'definitions' table to 'user_instances' table...");
                    let migration_sql = "
                        INSERT INTO user_instances SELECT * FROM definitions;
                        DELETE FROM definitions;
                    ";
                    db.query(migration_sql).await?;
                    eprintln!("✅ Table migration from 'definitions' to 'user_instances' completed");
                }
            }
        }

        // Check for field migrations in user_instances table
        let check_sql = "SELECT * FROM user_instances LIMIT 1";
        if let Ok(mut result) = db.query(check_sql).await {
            if let Ok(user_instances) = result.take::<Vec<serde_json::Value>>(0) {
                if !user_instances.is_empty() {
                    // Check if first record has old field names
                    if let Some(first_instance) = user_instances.first() {
                        if first_instance.get("terms").is_some() && first_instance.get("instances").is_none() {
                            eprintln!("🔄 Migrating database from 'terms' to 'instances'...");
                            let migration_sql = "
                                UPDATE user_instances SET instances = terms, grammar_id = 'core' WHERE terms IS NOT NULL;
                                UPDATE user_instances UNSET terms;
                            ";
                            db.query(migration_sql).await?;
                            eprintln!("✅ Database migration from 'terms' completed");
                        } else if first_instance.get("user_instance_index").is_some() && first_instance.get("instances").is_none() {
                            eprintln!("🔄 Migrating database from 'user_instance_index' to 'instances'...");
                            let migration_sql = "
                                UPDATE user_instances SET instances = user_instance_index, grammar_id = 'core' WHERE user_instance_index IS NOT NULL;
                                UPDATE user_instances UNSET user_instance_index;
                            ";
                            db.query(migration_sql).await?;
                            eprintln!("✅ Database migration from 'user_instance_index' completed");
                        }
                    }
                }
            }
        }

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

    pub async fn store_definition<T: SystematicStructure>(
        &self,
        definition: &T,
        name: &str,
        description: Option<&str>,
    ) -> Result<String, SystematicsError> {
        let id_string = Uuid::new_v4().to_string();
        let now = Datetime::default();
        
        // Convert connectives from (usize, usize) keys to string keys for storage
        let connectives: HashMap<String, String> = definition.connectives_traits()
            .iter()
            .map(|((from, to), relationship)| {
                (format!("{}:{}", from, to), relationship.clone())
            })
            .collect();
        
        let stored_user_instance = StoredUserInstance {
            id: Thing::from(("user_instances", id_string.as_str())),
            name: name.to_string(),
            structure_type: definition.structure_type().to_string(),
            grammar_id: "core".to_string(),
            instances: definition.user_instance_index().to_vec(),
            connectives,
            created_at: now.clone(),
            updated_at: now,
            description: description.map(|s| s.to_string()),
            metadata: HashMap::new(),
        };

        // Store the user instance
        let _: Option<StoredUserInstance> = self.db
            .create(("user_instances", id_string.as_str()))
            .content(stored_user_instance)
            .await?;

        // Store nodes and create graph representation
        let nodes = self.create_nodes(&id_string, definition.user_instance_index()).await?;
        let _edges = self.create_edges(&nodes).await?;

        Ok(id_string)
    }

    async fn create_nodes(&self, structure_id: &str, user_instances: &[String]) -> Result<Vec<GraphNode>, SystematicsError> {
        let mut nodes = Vec::new();
        let now = Datetime::default();

        for (position, user_instance) in user_instances.iter().enumerate() {
            let node_id = format!("{}_{}", structure_id, position);
            let node = GraphNode {
                id: Thing::from(("nodes", node_id.as_str())),
                structure_id: structure_id.to_string(),
                position,
                term: user_instance.clone(),
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

    pub async fn get_user_instance(&self, id: &str) -> Result<Option<StoredUserInstance>, SystematicsError> {
        let user_instance: Option<StoredUserInstance> = self.db.select(("user_instances", id)).await?;
        Ok(user_instance)
    }

    pub async fn list_user_instances(&self) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        let user_instances: Vec<StoredUserInstance> = self.db.select("user_instances").await?;
        Ok(user_instances)
    }

    pub async fn search_user_instances(&self, query: &str) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        let sql = "
            SELECT * FROM user_instances 
            WHERE name CONTAINS $query 
            OR description CONTAINS $query 
            OR array::some(instances, |$user_instance| $user_instance CONTAINS $query)
            ORDER BY created_at DESC
        ";
        
        let query_string = query.to_string();
        let mut result = self.db.query(sql).bind(("query", query_string)).await?;
        let user_instances: Vec<StoredUserInstance> = result.take(0)?;
        
        Ok(user_instances)
    }

    pub async fn get_related_user_instances(&self, id: &str) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        // Find user instances that share instances with the given user instance
        let sql = "
            SELECT DISTINCT s2.* FROM user_instances s1, user_instances s2
            WHERE s1.id = $id 
            AND s2.id != $id
            AND array::intersect(s1.instances, s2.instances) != []
            ORDER BY array::len(array::intersect(s1.instances, s2.instances)) DESC
        ";
        
        let id_string = id.to_string();
        let mut result = self.db.query(sql).bind(("id", id_string)).await?;
        let user_instances: Vec<StoredUserInstance> = result.take(0)?;
        
        Ok(user_instances)
    }

    pub async fn get_definition_graph(&self, id: &str) -> Result<(Vec<GraphNode>, Vec<GraphEdge>), SystematicsError> {
        // Get nodes for this definition
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

    pub async fn delete_user_instance(&self, id: &str) -> Result<bool, SystematicsError> {
        // Delete associated nodes and edges first
        let nodes_sql = "DELETE FROM nodes WHERE structure_id = $id";
        let id_string = id.to_string();
        self.db.query(nodes_sql).bind(("id", id_string.clone())).await?;

        // Delete edges (they should be cleaned up by the node deletion, but let's be explicit)
        let edges_sql = "DELETE FROM edges WHERE from_node LIKE $pattern OR to_node LIKE $pattern";
        let pattern = format!("{}_%", id);
        self.db.query(edges_sql).bind(("pattern", pattern)).await?;

        // Delete the user instance itself
        let deleted: Option<StoredUserInstance> = self.db.delete(("user_instances", id)).await?;
        
        Ok(deleted.is_some())
    }

    pub async fn update_user_instance_metadata(
        &self,
        id: &str,
        metadata: HashMap<String, String>,
    ) -> Result<bool, SystematicsError> {
        let sql = "UPDATE user_instances SET metadata = $metadata, updated_at = $now WHERE id = $id";
        let now = Datetime::default();
        let id_string = id.to_string();
        
        let mut result = self.db
            .query(sql)
            .bind(("id", id_string))
            .bind(("metadata", metadata))
            .bind(("now", now))
            .await?;
            
        let updated: Option<Vec<StoredUserInstance>> = result.take(0)?;
        
        Ok(updated.is_some() && !updated.unwrap().is_empty())
    }

    pub async fn get_instance_usage(&self, instance: &str) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        let sql = "
            SELECT * FROM user_instances 
            WHERE array::some(instances, |$ui| $ui = $instance)
            ORDER BY created_at DESC
        ";
        
        let instance_string = instance.to_string();
        let mut result = self.db.query(sql).bind(("instance", instance_string)).await?;
        let user_instances: Vec<StoredUserInstance> = result.take(0)?;
        
        Ok(user_instances)
    }

    /// Store a user instance directly from API request data (legacy method)
    pub async fn store_definition_direct(
        &self,
        name: &str,
        structure_type: &str,
        user_instance_index: Vec<String>,
        connectives: HashMap<String, String>,
        description: Option<String>,
    ) -> Result<String, SystematicsError> {
        // Delegate to the new method for backward compatibility
        self.store_user_instance_direct(name, structure_type, "core", user_instance_index, connectives, description).await
    }

    /// Store a user instance directly from API request data
    pub async fn store_user_instance_direct(
        &self,
        name: &str,
        structure_type: &str,
        grammar_id: &str,
        instances: Vec<String>,
        connectives: HashMap<String, String>,
        description: Option<String>,
    ) -> Result<String, SystematicsError> {
        let id_string = Uuid::new_v4().to_string();
        let now = Datetime::default();
        
        let stored_user_instance = StoredUserInstance {
            id: Thing::from(("user_instances", id_string.as_str())),
            name: name.to_string(),
            structure_type: structure_type.to_string(),
            grammar_id: grammar_id.to_string(),
            instances: instances.clone(),
            connectives,
            created_at: now.clone(),
            updated_at: now,
            description,
            metadata: HashMap::new(),
        };

        // Store the user instance
        let _: Option<StoredUserInstance> = self.db
            .create(("user_instances", id_string.as_str()))
            .content(stored_user_instance)
            .await?;

        // Store nodes and create graph representation
        let nodes = self.create_nodes(&id_string, &instances).await?;
        let _edges = self.create_edges(&nodes).await?;

        Ok(id_string)
    }

    // CommunityGrammar storage methods
    pub async fn list_community_grammars(&self, structure_type: Option<&str>) -> Result<Vec<StoredCommunityGrammar>, SystematicsError> {
        let sql = if let Some(_st) = structure_type {
            "SELECT * FROM community_grammars WHERE structure_type = $structure_type ORDER BY created_at DESC"
        } else {
            "SELECT * FROM community_grammars ORDER BY created_at DESC"
        };
        
        let mut result = if let Some(st) = structure_type {
            self.db.query(sql).bind(("structure_type", st.to_string())).await?
        } else {
            self.db.query(sql).await?
        };
        
        let community_grammars: Vec<StoredCommunityGrammar> = result.take(0)?;
        Ok(community_grammars)
    }

    pub async fn get_community_grammar(&self, id: &str) -> Result<Option<StoredCommunityGrammar>, SystematicsError> {
        let community_grammar: Option<StoredCommunityGrammar> = self.db.select(("community_grammars", id)).await?;
        Ok(community_grammar)
    }

    pub async fn create_community_grammar(
        &self,
        structure_type: &str,
        name: &str,
        term_characters: Vec<String>,
        author: &str,
        mapping_notes: &str,
        description: Option<String>,
    ) -> Result<String, SystematicsError> {
        let id_string = Uuid::new_v4().to_string();
        let now = Datetime::default();
        
        let stored_community_grammar = StoredCommunityGrammar {
            id: Thing::from(("community_grammars", id_string.as_str())),
            structure_type: structure_type.to_string(),
            name: name.to_string(),
            term_characters,
            author: author.to_string(),
            mapping_notes: mapping_notes.to_string(),
            created_at: now.clone(),
            updated_at: now,
            description,
        };

        // Store the community grammar
        let _: Option<StoredCommunityGrammar> = self.db
            .create(("community_grammars", id_string.as_str()))
            .content(stored_community_grammar)
            .await?;

        Ok(id_string)
    }

    pub async fn update_community_grammar(
        &self,
        id: &str,
        structure_type: &str,
        name: &str,
        term_characters: Vec<String>,
        author: &str,
        mapping_notes: &str,
        description: Option<String>,
    ) -> Result<bool, SystematicsError> {
        let now = Datetime::default();
        
        let sql = "
            UPDATE community_grammars SET 
                structure_type = $structure_type,
                name = $name,
                term_characters = $term_characters,
                author = $author,
                mapping_notes = $mapping_notes,
                description = $description,
                updated_at = $now
            WHERE id = $id
        ";
        
        let mut result = self.db
            .query(sql)
            .bind(("id", id.to_string()))
            .bind(("structure_type", structure_type.to_string()))
            .bind(("name", name.to_string()))
            .bind(("term_characters", term_characters))
            .bind(("author", author.to_string()))
            .bind(("mapping_notes", mapping_notes.to_string()))
            .bind(("description", description))
            .bind(("now", now))
            .await?;
            
        let updated: Option<Vec<StoredCommunityGrammar>> = result.take(0)?;
        Ok(updated.is_some() && !updated.unwrap().is_empty())
    }

    pub async fn delete_community_grammar(&self, id: &str) -> Result<bool, SystematicsError> {
        let deleted: Option<StoredCommunityGrammar> = self.db.delete(("community_grammars", id)).await?;
        Ok(deleted.is_some())
    }

    pub async fn search_community_grammars(&self, query: &str) -> Result<Vec<StoredCommunityGrammar>, SystematicsError> {
        let sql = "
            SELECT * FROM community_grammars 
            WHERE name CONTAINS $query 
            OR author CONTAINS $query 
            OR description CONTAINS $query 
            OR mapping_notes CONTAINS $query
            OR array::some(term_characters, |$term| $term CONTAINS $query)
            ORDER BY created_at DESC
        ";
        
        let query_string = query.to_string();
        let mut result = self.db.query(sql).bind(("query", query_string)).await?;
        let community_grammars: Vec<StoredCommunityGrammar> = result.take(0)?;
        
        Ok(community_grammars)
    }
}

// Convert SurrealDB errors to our error type
impl From<surrealdb::Error> for SystematicsError {
    fn from(err: surrealdb::Error) -> Self {
        SystematicsError::Storage(format!("SurrealDB error: {}", err))
    }
} 