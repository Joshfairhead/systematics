use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// Ground: User Expressions (concrete personal applications)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserExpression {
    pub id: serde_json::Value,
    pub name: String,
    pub definition_type: String,
    pub grammar_id: String, // References either core or community grammar
    pub user_expressions: Vec<String>, // Concrete user data
    pub connectives: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub metadata: HashMap<String, String>,
}

// Directive: Core Grammar (Bennett's canonical terms)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoreGrammar {
    pub definition_type: String,
    pub name: String, // "Bennett's Canonical"
    pub term_characters: Vec<String>, // Bennett's canonical terms
    pub coherence_attribute: String,
    pub term_designation: String,
    pub source: String,
    pub first_order_connectives_type: String,
    pub connectives: Vec<ConnectiveInfo>,
}

// Instrumental: Community Grammar (user-contributed mappings)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommunityGrammar {
    pub id: serde_json::Value,
    pub definition_type: String,
    pub name: String, // "Landry's Metaphysics", "Agent Languages"
    pub term_characters: Vec<String>, // Alternative terms mapping to same positions
    pub author: String,
    pub mapping_notes: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
}

// Source: System Definitions (pure mathematical structures)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemDefinition {
    pub definition_type: String,
    pub term_count: usize,
    pub term_characters: Vec<String>, // Added for compatibility
    pub coherence_attribute: String,
    pub term_designation: String,
    pub source: String,
    pub first_order_connectives_type: String,
    pub connectives: Vec<ConnectiveInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStructureRequest {
    pub name: String,
    pub definition_type: String,
    pub user_expressions: Vec<String>, // Clean field name
    pub connectives: HashMap<String, String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserExpressionRequest {
    pub name: String,
    pub definition_type: String,
    pub grammar_id: String,
    pub user_expressions: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommunityGrammarRequest {
    pub definition_type: String,
    pub name: String,
    pub term_characters: Vec<String>,
    pub author: String,
    pub mapping_notes: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectiveInfo {
    pub from_index: usize,
    pub to_index: usize,
    pub relation_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: serde_json::Value,
    pub structure_id: String,
    pub position: usize,
    pub term: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: serde_json::Value,
    pub from_node: String,
    pub to_node: String,
    pub relationship_type: String,
    pub weight: f64,
    pub created_at: String,
}

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:3001".to_string(), // Default API server URL
        }
    }





    pub async fn list_user_expressions(&self) -> Result<Vec<UserExpression>, anyhow::Error> {
        let url = format!("{}/user-instances", self.base_url);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<Vec<UserExpression>> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to list user user_expressions: {}", response.status()))
        }
    }

    pub async fn create_user_instance(&self, name: &str, definition_type: &str, grammar_id: &str, user_expressions: &[String]) -> Result<String, anyhow::Error> {
        let request = CreateUserExpressionRequest {
            name: name.to_string(),
            definition_type: definition_type.to_string(),
            grammar_id: grammar_id.to_string(),
            user_expressions: user_expressions.to_vec(),
            connectives: std::collections::HashMap::new(),
            description: Some(format!("User-created {} expression", definition_type)),
        };
        
        let url = format!("{}/user-instances", self.base_url);
        let response = Request::post(&url)
            .json(&request)?
            .send()
            .await?;
        
        if response.ok() {
            let api_response: ApiResponse<String> = response.json().await?;
            if api_response.success {
                api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
            } else {
                Err(anyhow::anyhow!("API error: {}", api_response.error.unwrap_or_else(|| "Unknown error".to_string())))
            }
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow::anyhow!("Failed to create user expression: {}", error_text))
        }
    }

    pub async fn get_core_grammar(&self, definition_type: &str) -> Result<CoreGrammar, anyhow::Error> {
        let url = format!("{}/core-grammar/{}", self.base_url, definition_type);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<CoreGrammar> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to get core grammar: {}", response.status()))
        }
    }

    pub async fn list_community_grammars(&self, definition_type: Option<&str>) -> Result<Vec<CommunityGrammar>, anyhow::Error> {
        let url = if let Some(definition_type) = definition_type {
            format!("{}/community-grammar?definition_type={}", self.base_url, definition_type)
        } else {
            format!("{}/community-grammar", self.base_url)
        };
        
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<Vec<CommunityGrammar>> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to list community grammars: {}", response.status()))
        }
    }

    pub async fn get_system_definition(&self, definition_type: &str) -> Result<SystemDefinition, anyhow::Error> {
        let url = format!("{}/definition/{}", self.base_url, definition_type);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<SystemDefinition> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to get system definition: {}", response.status()))
        }
    }



    pub async fn search_user_instances(&self, query: &str) -> Result<Vec<UserExpression>, anyhow::Error> {
        let url = format!("{}/user-instances/search?q={}", self.base_url, query);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<Vec<UserExpression>> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to search user user_expressions: {}", response.status()))
        }
    }

    pub async fn save_user_instance(&self, name: &str, definition_type: &str, user_expressions: &[String]) -> Result<String, anyhow::Error> {
        let grammar_id = "core".to_string(); // Default to core grammar
        self.create_user_instance(name, definition_type, &grammar_id, user_expressions).await
    }
}

// Helper function for spawning async tasks in Yew components
pub fn spawn_api_call<F, T>(future: F, callback: Callback<Result<T, anyhow::Error>>)
where
    F: std::future::Future<Output = Result<T, anyhow::Error>> + 'static,
    T: 'static,
{
    spawn_local(async move {
        let result = future.await;
        callback.emit(result);
    });
}

 