use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// Mirror the API types from the server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredUserDefinition {
    pub id: serde_json::Value, // Thing type from SurrealDB
    pub name: String,
    pub structure_type: String,
    pub user_instance_index: Vec<String>, // Matches backend field exactly
    pub connectives: HashMap<String, String>,
    pub created_at: String, // Simplified for frontend
    pub updated_at: String,
    pub description: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStructureRequest {
    pub name: String,
    pub structure_type: String,
    pub user_instance_index: Vec<String>, // Matches backend field exactly
    pub connectives: HashMap<String, String>,
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
    pub from_position: usize,
    pub to_position: usize,
    pub relationship: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemDefinition {
    pub structure_type: String,
    pub term_count: usize,
    pub term_characters: Vec<String>,
    pub coherence_attribute: String,
    pub term_designation: String,
    pub source: String,
    pub first_order_connectives_type: String,
    pub connectives: Vec<ConnectiveInfo>,
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

    pub fn with_base_url(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn health_check(&self) -> Result<String, anyhow::Error> {
        let url = format!("{}/health", self.base_url);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<String> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Health check failed: {}", response.status()))
        }
    }

    pub async fn list_definitions(&self) -> Result<Vec<StoredUserDefinition>, anyhow::Error> {
        let url = format!("{}/definitions", self.base_url);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<Vec<StoredUserDefinition>> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to list definitions: {}", response.status()))
        }
    }

    pub async fn get_definition(&self, id: &str) -> Result<StoredUserDefinition, anyhow::Error> {
        let url = format!("{}/definitions/{}", self.base_url, id);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<StoredUserDefinition> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to get definition: {}", response.status()))
        }
    }

    pub async fn create_definition(&self, request: CreateStructureRequest) -> Result<String, anyhow::Error> {
        let url = format!("{}/definitions", self.base_url);
        let response = Request::post(&url)
            .json(&request)?
            .send()
            .await?;
        
        if response.ok() {
            let api_response: ApiResponse<String> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow::anyhow!("Failed to create definition: {}", error_text))
        }
    }

    pub async fn save_definition(&self, name: &str, structure_type: &str, user_instances: &[String]) -> Result<String, anyhow::Error> {
        let request = CreateStructureRequest {
            name: name.to_string(),
            structure_type: structure_type.to_string(),
            user_instance_index: user_instances.to_vec(), // Consistent terminology
            connectives: std::collections::HashMap::new(), // Empty for now
            description: Some(format!("User-created {} definition", structure_type)),
        };
        
        self.create_definition(request).await
    }

    pub async fn search_definitions(&self, query: &str) -> Result<Vec<StoredUserDefinition>, anyhow::Error> {
        let url = format!("{}/definitions/search?q={}", self.base_url, query);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<Vec<StoredUserDefinition>> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to search definitions: {}", response.status()))
        }
    }

    pub async fn get_system_definition(&self, structure_type: &str) -> Result<SystemDefinition, anyhow::Error> {
        let url = format!("{}/definition/{}", self.base_url, structure_type);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<SystemDefinition> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to get system definition: {}", response.status()))
        }
    }

    pub async fn delete_definition(&self, id: &str) -> Result<bool, anyhow::Error> {
        let url = format!("{}/definitions/{}", self.base_url, id);
        let response = Request::delete(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<bool> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to delete definition: {}", response.status()))
        }
    }

    pub async fn get_related_definitions(&self, id: &str) -> Result<Vec<StoredUserDefinition>, anyhow::Error> {
        let url = format!("{}/definitions/{}/related", self.base_url, id);
        let response = Request::get(&url).send().await?;
        
        if response.ok() {
            let api_response: ApiResponse<Vec<StoredUserDefinition>> = response.json().await?;
            api_response.data.ok_or_else(|| anyhow::anyhow!("No data in response"))
        } else {
            Err(anyhow::anyhow!("Failed to get related definitions: {}", response.status()))
        }
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