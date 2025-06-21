use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use systematics_api::SystematicsError;

#[derive(Debug, Serialize)]
pub struct CreateStructureRequest {
    pub name: String,
    pub definition_type: String,
    pub user_instance_index: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StoredUserInstance {
    pub id: StructureId,
    pub name: String,
    pub definition_type: String,
    pub grammar_id: String,
    pub instances: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StructureId {
    pub tb: String,
    pub id: StructureIdValue,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StructureIdValue {
    String(String),
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: Option<String>) -> Self {
        let base_url = base_url.unwrap_or_else(|| "http://localhost:3001".to_string());
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn health_check(&self) -> Result<bool, SystematicsError> {
        let url = format!("{}/health", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    pub async fn list_user_instances(&self) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        let url = format!("{}/user-instances", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| SystematicsError::Storage(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(SystematicsError::Storage(format!(
                "API request failed with status: {}", 
                response.status()
            )));
        }

        let api_response: ApiResponse<Vec<StoredUserInstance>> = response
            .json()
            .await
            .map_err(|e| SystematicsError::Storage(format!("Failed to parse response: {}", e)))?;

        if api_response.success {
            Ok(api_response.data.unwrap_or_default())
        } else {
            Err(SystematicsError::Storage(
                api_response.error.unwrap_or_else(|| "Unknown API error".to_string())
            ))
        }
    }

    pub async fn search_user_instances(&self, query: &str) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        let url = format!("{}/user-instances/search?q={}", self.base_url, urlencoding::encode(query));
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| SystematicsError::Storage(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(SystematicsError::Storage(format!(
                "API request failed with status: {}", 
                response.status()
            )));
        }

        let api_response: ApiResponse<Vec<StoredUserInstance>> = response
            .json()
            .await
            .map_err(|e| SystematicsError::Storage(format!("Failed to parse response: {}", e)))?;

        if api_response.success {
            Ok(api_response.data.unwrap_or_default())
        } else {
            Err(SystematicsError::Storage(
                api_response.error.unwrap_or_else(|| "Unknown API error".to_string())
            ))
        }
    }

    pub async fn get_user_instance(&self, id: &str) -> Result<Option<StoredUserInstance>, SystematicsError> {
        let url = format!("{}/definitions/{}", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| SystematicsError::Storage(format!("HTTP request failed: {}", e)))?;

        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(SystematicsError::Storage(format!(
                "API request failed with status: {}", 
                response.status()
            )));
        }

        let api_response: ApiResponse<StoredUserInstance> = response
            .json()
            .await
            .map_err(|e| SystematicsError::Storage(format!("Failed to parse response: {}", e)))?;

        if api_response.success {
            Ok(api_response.data)
        } else {
            Err(SystematicsError::Storage(
                api_response.error.unwrap_or_else(|| "Unknown API error".to_string())
            ))
        }
    }

    pub async fn create_definition(
        &self,
        name: &str,
        definition_type: &str,
        user_instance_index: Vec<String>,
        connectives: HashMap<String, String>,
        description: Option<String>,
    ) -> Result<String, SystematicsError> {
        let url = format!("{}/definitions", self.base_url);
        
        let request = CreateStructureRequest {
            name: name.to_string(),
            definition_type: definition_type.to_string(),
            user_instance_index,
            connectives,
            description,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| SystematicsError::Storage(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(SystematicsError::Storage(format!(
                "API request failed with status: {}", 
                response.status()
            )));
        }

        let api_response: ApiResponse<String> = response
            .json()
            .await
            .map_err(|e| SystematicsError::Storage(format!("Failed to parse response: {}", e)))?;

        if api_response.success {
            Ok(api_response.data.unwrap_or_else(|| "Unknown ID".to_string()))
        } else {
            Err(SystematicsError::Storage(
                api_response.error.unwrap_or_else(|| "Unknown API error".to_string())
            ))
        }
    }



    pub async fn get_related_user_instances(&self, id: &str) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        let url = format!("{}/definitions/{}/related", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| SystematicsError::Storage(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(SystematicsError::Storage(format!(
                "API request failed with status: {}", 
                response.status()
            )));
        }

        let api_response: ApiResponse<Vec<StoredUserInstance>> = response
            .json()
            .await
            .map_err(|e| SystematicsError::Storage(format!("Failed to parse response: {}", e)))?;

        if api_response.success {
            Ok(api_response.data.unwrap_or_default())
        } else {
            Err(SystematicsError::Storage(
                api_response.error.unwrap_or_else(|| "Unknown API error".to_string())
            ))
        }
    }

    // Backward compatibility methods
    pub async fn list_definitions(&self) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        self.list_user_instances().await
    }

    pub async fn search_definitions(&self, query: &str) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        self.search_user_instances(query).await
    }

    pub async fn get_definition(&self, id: &str) -> Result<Option<StoredUserInstance>, SystematicsError> {
        self.get_user_instance(id).await
    }

    pub async fn get_related_definitions(&self, id: &str) -> Result<Vec<StoredUserInstance>, SystematicsError> {
        self.get_related_user_instances(id).await
    }

    pub async fn delete_definition(&self, id: &str) -> Result<bool, SystematicsError> {
        self.delete_user_instance(id).await
    }

    pub async fn delete_user_instance(&self, id: &str) -> Result<bool, SystematicsError> {
        let url = format!("{}/definitions/{}", self.base_url, id);
        
        let response = self.client
            .delete(&url)
            .send()
            .await
            .map_err(|e| SystematicsError::Storage(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(SystematicsError::Storage(format!(
                "API request failed with status: {}", 
                response.status()
            )));
        }

        let api_response: ApiResponse<bool> = response
            .json()
            .await
            .map_err(|e| SystematicsError::Storage(format!("Failed to parse response: {}", e)))?;

        if api_response.success {
            Ok(api_response.data.unwrap_or(false))
        } else {
            Err(SystematicsError::Storage(
                api_response.error.unwrap_or_else(|| "Unknown API error".to_string())
            ))
        }
    }
} 