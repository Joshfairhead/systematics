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

/// Database environment types for systematic separation
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseEnvironment {
    /// Wipeable/expendable testing database
    Testing,
    /// Protected persistent development database (includes systematic grammars)
    Development,
}

impl DatabaseEnvironment {
    /// Get the database file path for this environment
    pub fn db_path(&self) -> String {
        let base_path = std::env::var("SYSTEMATICS_DATA_PATH")
            .unwrap_or_else(|_| "../data".to_string());
        
        match self {
            DatabaseEnvironment::Testing => format!("{}/testing_systematics.db", base_path),
            DatabaseEnvironment::Development => format!("{}/development_systematics.db", base_path),
        }
    }
    
    /// Check if this environment is protected against destructive operations
    pub fn is_protected(&self) -> bool {
        match self {
            DatabaseEnvironment::Testing => false,
            DatabaseEnvironment::Development => true,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoredUserExpression {
    pub id: Thing,
    pub name: String,
    pub definition_type: String,
    pub grammar_id: String,
    pub user_expressions: Vec<String>,
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
    pub definition_type: String,
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
    environment: DatabaseEnvironment,
}

impl SurrealStorage {
    pub async fn new_with_environment(environment: DatabaseEnvironment) -> Result<Self, SystematicsError> {
        let db_path = environment.db_path();
        
        // Use file-based storage for persistence
        let db = Surreal::new::<RocksDb>(&db_path).await?;
        
        // Use a namespace and database
        db.use_ns("systematics").use_db("user_expressions").await?;
        
        // Initialize schema
        Self::init_schema(&db).await?;
        
        Ok(Self { db, environment })
    }

    pub async fn new(db_path: &str) -> Result<Self, SystematicsError> {
        // Legacy method - determine environment from path
        let environment = if db_path.contains("testing") {
            DatabaseEnvironment::Testing
        } else {
            DatabaseEnvironment::Development
        };
        
        // Use file-based storage for persistence
        let db = Surreal::new::<RocksDb>(db_path).await?;
        
        // Use a namespace and database
        db.use_ns("systematics").use_db("user_expressions").await?;
        
        // Initialize schema
        Self::init_schema(&db).await?;
        
        Ok(Self { db, environment })
    }

    /// Create a new SurrealStorage with default development database
    pub async fn new_default() -> Result<Self, SystematicsError> {
        Self::new_with_environment(DatabaseEnvironment::Development).await
    }
    
    /// Get the current database environment
    pub fn environment(&self) -> &DatabaseEnvironment {
        &self.environment
    }

    async fn init_schema(db: &Surreal<Db>) -> Result<(), SystematicsError> {
        // Create tables and indexes - using SCHEMALESS to avoid type conflicts
        db.query("
            DEFINE TABLE user_expressions SCHEMALESS;
            DEFINE INDEX idx_name ON user_expressions COLUMNS name;
            DEFINE INDEX idx_type ON user_expressions COLUMNS definition_type;
            DEFINE INDEX idx_created ON user_expressions COLUMNS created_at;
        ").await?;

        db.query("
            DEFINE TABLE community_grammars SCHEMALESS;
            DEFINE INDEX idx_grammar_name ON community_grammars COLUMNS name;
            DEFINE INDEX idx_grammar_type ON community_grammars COLUMNS definition_type;
            DEFINE INDEX idx_grammar_author ON community_grammars COLUMNS author;
            DEFINE INDEX idx_grammar_created ON community_grammars COLUMNS created_at;
        ").await?;

        // Migrate existing records from old table names and field names
        // First, check if migration is needed from old 'definitions' table
        let check_definitions_sql = "SELECT * FROM definitions LIMIT 1";
        if let Ok(mut result) = db.query(check_definitions_sql).await {
            if let Ok(definitions) = result.take::<Vec<serde_json::Value>>(0) {
                if !definitions.is_empty() {
                    eprintln!("🔄 Migrating from 'definitions' table to 'user_expressions' table...");
                    let migration_sql = "
                        INSERT INTO user_expressions SELECT * FROM definitions;
                        DELETE FROM definitions;
                    ";
                    db.query(migration_sql).await?;
                    eprintln!("✅ Table migration from 'definitions' to 'user_expressions' completed");
                }
            }
        }

        // Check if migration is needed from old 'user_expressions' table
        let check_instances_sql = "SELECT * FROM user_expressions LIMIT 1";
        if let Ok(mut result) = db.query(check_instances_sql).await {
            if let Ok(instances) = result.take::<Vec<serde_json::Value>>(0) {
                if !instances.is_empty() {
                    eprintln!("🔄 Migrating from 'user_expressions' table to 'user_expressions' table...");
                    let migration_sql = "
                        INSERT INTO user_expressions SELECT * FROM user_expressions;
                        DELETE FROM user_expressions;
                    ";
                    db.query(migration_sql).await?;
                    eprintln!("✅ Table migration from 'user_expressions' to 'user_expressions' completed");
                }
            }
        }

        // Check for field migrations in user_expressions table
        let check_sql = "SELECT * FROM user_expressions LIMIT 1";
        if let Ok(mut result) = db.query(check_sql).await {
            if let Ok(user_expressions) = result.take::<Vec<serde_json::Value>>(0) {
                if !user_expressions.is_empty() {
                    // Check if first record has old field names
                    if let Some(first_expression) = user_expressions.first() {
                        if first_expression.get("terms").is_some() && first_expression.get("user_expressions").is_none() {
                            eprintln!("🔄 Migrating database from 'terms' to 'user_expressions'...");
                            let migration_sql = "
                                UPDATE user_expressions SET user_expressions = terms, grammar_id = 'core' WHERE terms IS NOT NULL;
                                UPDATE user_expressions UNSET terms;
                            ";
                            db.query(migration_sql).await?;
                            eprintln!("✅ Database migration from 'terms' completed");
                        } else if first_expression.get("user_expressions").is_some() && first_expression.get("user_expressions").is_none() {
                            eprintln!("🔄 Migrating database from 'instances' to 'user_expressions'...");
                            let migration_sql = "
                                UPDATE user_expressions SET user_expressions = instances, grammar_id = 'core' WHERE instances IS NOT NULL;
                                UPDATE user_expressions UNSET instances;
                            ";
                            db.query(migration_sql).await?;
                            eprintln!("✅ Database migration from 'instances' completed");
                        } else if first_expression.get("expressions").is_some() && first_expression.get("user_expressions").is_none() {
                            eprintln!("🔄 Migrating database from 'expressions' to 'user_expressions'...");
                            let migration_sql = "
                                UPDATE user_expressions SET user_expressions = expressions, grammar_id = 'core' WHERE expressions IS NOT NULL;
                                UPDATE user_expressions UNSET expressions;
                            ";
                            db.query(migration_sql).await?;
                            eprintln!("✅ Database migration from 'expressions' completed");
                        } else if first_expression.get("user_instance_index").is_some() && first_expression.get("user_expressions").is_none() {
                            eprintln!("🔄 Migrating database from 'user_instance_index' to 'user_expressions'...");
                            let migration_sql = "
                                UPDATE user_expressions SET user_expressions = user_instance_index, grammar_id = 'core' WHERE user_instance_index IS NOT NULL;
                                UPDATE user_expressions UNSET user_instance_index;
                            ";
                            db.query(migration_sql).await?;
                            eprintln!("✅ Database migration from 'user_instance_index' completed");
                        }
                        
                        // Check if we need to migrate structure_type to definition_type
                        if first_expression.get("structure_type").is_some() && first_expression.get("definition_type").is_none() {
                            eprintln!("🔄 Migrating database from 'structure_type' to 'definition_type'...");
                            let migration_sql = "
                                UPDATE user_expressions SET definition_type = structure_type WHERE structure_type IS NOT NULL;
                                UPDATE user_expressions UNSET structure_type;
                            ";
                            db.query(migration_sql).await?;
                            eprintln!("✅ Database migration from 'structure_type' completed");
                        }
                    }
                }
            }
        }

        // Check for field migrations in community_grammars table
        let check_community_sql = "SELECT * FROM community_grammars LIMIT 1";
        if let Ok(mut result) = db.query(check_community_sql).await {
            if let Ok(community_grammars) = result.take::<Vec<serde_json::Value>>(0) {
                if !community_grammars.is_empty() {
                    if let Some(first_grammar) = community_grammars.first() {
                        if first_grammar.get("structure_type").is_some() && first_grammar.get("definition_type").is_none() {
                            eprintln!("🔄 Migrating community_grammars from 'structure_type' to 'definition_type'...");
                            let migration_sql = "
                                UPDATE community_grammars SET definition_type = structure_type WHERE structure_type IS NOT NULL;
                                UPDATE community_grammars UNSET structure_type;
                            ";
                            db.query(migration_sql).await?;
                            eprintln!("✅ Community grammars migration from 'structure_type' completed");
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
        
        let stored_user_instance = StoredUserExpression {
            id: Thing::from(("user_expressions", id_string.as_str())),
            name: name.to_string(),
            definition_type: definition.definition_type().to_string(),
            grammar_id: "core".to_string(),
            user_expressions: definition.user_expressions().to_vec(),
            connectives,
            created_at: now.clone(),
            updated_at: now,
            description: description.map(|s| s.to_string()),
            metadata: HashMap::new(),
        };

        // Store the user expression
        let _: Option<StoredUserExpression> = self.db
            .create(("user_expressions", id_string.as_str()))
            .content(stored_user_instance)
            .await?;

        // Store nodes and create graph representation
        let nodes = self.create_nodes(&id_string, definition.user_expressions()).await?;
        let _edges = self.create_edges(&nodes).await?;

        Ok(id_string)
    }

    async fn create_nodes(&self, structure_id: &str, user_expressions: &[String]) -> Result<Vec<GraphNode>, SystematicsError> {
        let mut nodes = Vec::new();
        let now = Datetime::default();

        for (position, user_instance) in user_expressions.iter().enumerate() {
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

    pub async fn get_user_expression(&self, id: &str) -> Result<Option<StoredUserExpression>, SystematicsError> {
        let user_instance: Option<StoredUserExpression> = self.db.select(("user_expressions", id)).await?;
        Ok(user_instance)
    }

    pub async fn list_user_expressions(&self) -> Result<Vec<StoredUserExpression>, SystematicsError> {
        let user_expressions: Vec<StoredUserExpression> = self.db.select("user_expressions").await?;
        Ok(user_expressions)
    }

    pub async fn search_user_expressions(&self, query: &str) -> Result<Vec<StoredUserExpression>, SystematicsError> {
        let sql = "
            SELECT * FROM user_expressions 
            WHERE name CONTAINS $query 
            OR description CONTAINS $query 
            OR array::some(user_expressions, |$user_expression| $user_expression CONTAINS $query)
            ORDER BY created_at DESC
        ";
        
        let query_string = query.to_string();
        let mut result = self.db.query(sql).bind(("query", query_string)).await?;
        let user_expressions: Vec<StoredUserExpression> = result.take(0)?;
        
        Ok(user_expressions)
    }

    pub async fn get_related_user_expressions(&self, id: &str) -> Result<Vec<StoredUserExpression>, SystematicsError> {
        // Find user expressions that share instances with the given user expression
        let sql = "
            SELECT DISTINCT s2.* FROM user_expressions s1, user_expressions s2
            WHERE s1.id = $id 
            AND s2.id != $id
            AND array::intersect(s1.user_expressions, s2.user_expressions) != []
            ORDER BY array::len(array::intersect(s1.user_expressions, s2.user_expressions)) DESC
        ";
        
        let id_string = id.to_string();
        let mut result = self.db.query(sql).bind(("id", id_string)).await?;
        let user_expressions: Vec<StoredUserExpression> = result.take(0)?;
        
        Ok(user_expressions)
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

    pub async fn delete_user_expression(&self, id: &str) -> Result<bool, SystematicsError> {
        // Delete associated nodes and edges first
        let nodes_sql = "DELETE FROM nodes WHERE structure_id = $id";
        let id_string = id.to_string();
        self.db.query(nodes_sql).bind(("id", id_string.clone())).await?;

        // Delete edges (they should be cleaned up by the node deletion, but let's be explicit)
        let edges_sql = "DELETE FROM edges WHERE from_node LIKE $pattern OR to_node LIKE $pattern";
        let pattern = format!("{}_%", id);
        self.db.query(edges_sql).bind(("pattern", pattern)).await?;

        // Delete the user expression itself
        let deleted: Option<StoredUserExpression> = self.db.delete(("user_expressions", id)).await?;
        
        Ok(deleted.is_some())
    }

    pub async fn update_user_expression_metadata(
        &self,
        id: &str,
        metadata: HashMap<String, String>,
    ) -> Result<bool, SystematicsError> {
        let sql = "UPDATE user_expressions SET metadata = $metadata, updated_at = $now WHERE id = $id";
        let now = Datetime::default();
        let id_string = id.to_string();
        
        let mut result = self.db
            .query(sql)
            .bind(("id", id_string))
            .bind(("metadata", metadata))
            .bind(("now", now))
            .await?;
            
        let updated: Option<Vec<StoredUserExpression>> = result.take(0)?;
        
        Ok(updated.is_some() && !updated.unwrap().is_empty())
    }

    pub async fn get_instance_usage(&self, instance: &str) -> Result<Vec<StoredUserExpression>, SystematicsError> {
        let sql = "
            SELECT * FROM user_expressions 
            WHERE array::some(user_expressions, |$ue| $ue = $instance)
            ORDER BY created_at DESC
        ";
        
        let instance_string = instance.to_string();
        let mut result = self.db.query(sql).bind(("instance", instance_string)).await?;
        let user_expressions: Vec<StoredUserExpression> = result.take(0)?;
        
        Ok(user_expressions)
    }

    /// Store a user expression directly from API request data
    pub async fn save_user_expression(
        &self,
        name: &str,
        definition_type: &str,
        user_expressions: Vec<String>,
        connectives: HashMap<String, String>,
        description: Option<String>,
    ) -> Result<String, SystematicsError> {
        let grammar_id = "core"; // Default to core grammar
        let id_string = Uuid::new_v4().to_string();
        let now = Datetime::default();
        
        let stored_user_instance = StoredUserExpression {
            id: Thing::from(("user_expressions", id_string.as_str())),
            name: name.to_string(),
            definition_type: definition_type.to_string(),
            grammar_id: grammar_id.to_string(),
            user_expressions: user_expressions.clone(),
            connectives,
            created_at: now.clone(),
            updated_at: now,
            description,
            metadata: HashMap::new(),
        };

        // Store the user expression
        let _: Option<StoredUserExpression> = self.db
            .create(("user_expressions", id_string.as_str()))
            .content(stored_user_instance)
            .await?;

        // Store nodes and create graph representation
        let nodes = self.create_nodes(&id_string, &user_expressions).await?;
        let _edges = self.create_edges(&nodes).await?;

        Ok(id_string)
    }

    // CommunityGrammar storage methods
    pub async fn list_community_grammars(&self, definition_type: Option<&str>) -> Result<Vec<StoredCommunityGrammar>, SystematicsError> {
        let sql = if let Some(_st) = definition_type {
            "SELECT * FROM community_grammars WHERE definition_type = $definition_type ORDER BY created_at DESC"
        } else {
            "SELECT * FROM community_grammars ORDER BY created_at DESC"
        };
        
        let mut result = if let Some(st) = definition_type {
            self.db.query(sql).bind(("definition_type", st.to_string())).await?
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
        definition_type: &str,
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
            definition_type: definition_type.to_string(),
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
        definition_type: &str,
        name: &str,
        term_characters: Vec<String>,
        author: &str,
        mapping_notes: &str,
        description: Option<String>,
    ) -> Result<bool, SystematicsError> {
        let now = Datetime::default();
        
        let sql = "
            UPDATE community_grammars SET 
                definition_type = $definition_type,
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
            .bind(("definition_type", definition_type.to_string()))
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