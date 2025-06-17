#[cfg(feature = "server")]
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::collections::HashMap;
#[cfg(feature = "server")]
use tower_http::cors::CorsLayer;

use crate::{SurrealStorage, StoredStructure, SystematicsError};

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct AppState {
    pub storage: SurrealStorage,
}

#[cfg(feature = "server")]
#[derive(Deserialize)]
pub struct SearchQuery {
    q: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Deserialize)]
pub struct CreateStructureRequest {
    pub name: String,
    pub structure_type: String,
    pub terms: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[cfg(feature = "server")]
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[cfg(feature = "server")]
pub fn create_router(storage: SurrealStorage) -> Router {
    let state = AppState { storage };

    Router::new()
        .route("/structures", get(list_structures))
        .route("/structures", post(create_structure))
        .route("/structures/search", get(search_structures))
        .route("/structures/:id", get(get_structure))
        .route("/structures/:id", delete(delete_structure))
        .route("/structures/:id/related", get(get_related_structures))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#[cfg(feature = "server")]
async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("SysteMaster API is running"))
}

#[cfg(feature = "server")]
async fn list_structures(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredStructure>>>, StatusCode> {
    match state.storage.list_structures().await {
        Ok(structures) => Ok(Json(ApiResponse::success(structures))),
        Err(e) => {
            eprintln!("Error listing structures: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn search_structures(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredStructure>>>, StatusCode> {
    let query = params.q.unwrap_or_default();
    
    match state.storage.search_structures(&query).await {
        Ok(structures) => Ok(Json(ApiResponse::success(structures))),
        Err(e) => {
            eprintln!("Error searching structures: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn get_structure(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<StoredStructure>>, StatusCode> {
    match state.storage.get_structure(&id).await {
        Ok(Some(structure)) => Ok(Json(ApiResponse::success(structure))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error getting structure: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn create_structure(
    State(_state): State<AppState>,
    Json(_payload): Json<CreateStructureRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // Note: This is a simplified create endpoint
    // In practice, you'd want to validate the structure type and create the appropriate structure
    // For now, we'll return an error indicating this needs to be implemented
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[cfg(feature = "server")]
async fn delete_structure(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    match state.storage.delete_structure(&id).await {
        Ok(deleted) => Ok(Json(ApiResponse::success(deleted))),
        Err(e) => {
            eprintln!("Error deleting structure: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn get_related_structures(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredStructure>>>, StatusCode> {
    match state.storage.get_related_structures(&id).await {
        Ok(structures) => Ok(Json(ApiResponse::success(structures))),
        Err(e) => {
            eprintln!("Error getting related structures: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
pub async fn start_server(storage: SurrealStorage, port: u16) -> Result<(), SystematicsError> {
    let app = create_router(storage);
    let addr = format!("0.0.0.0:{}", port);
    
    println!("🚀 SysteMaster API server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| SystematicsError::Storage(format!("Failed to bind to {}: {}", addr, e)))?;
    
    axum::serve(listener, app)
        .await
        .map_err(|e| SystematicsError::Storage(format!("Server error: {}", e)))?;
    
    Ok(())
} 