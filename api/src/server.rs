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
    State(state): State<AppState>,
    Json(payload): Json<CreateStructureRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // Validate structure type
    let valid_types = ["monad", "dyad", "triad", "tetrad", "pentad", "hexad", "heptad", "octad", "ennead", "decad", "undecad", "dodecad"];
    if !valid_types.contains(&payload.structure_type.as_str()) {
        return Ok(Json(ApiResponse::error(format!(
            "Invalid structure type '{}'. Valid types: {}",
            payload.structure_type,
            valid_types.join(", ")
        ))));
    }

    // Validate term count matches structure type
    let expected_term_count = match payload.structure_type.as_str() {
        "monad" => 1,
        "dyad" => 2,
        "triad" => 3,
        "tetrad" => 4,
        "pentad" => 5,
        "hexad" => 6,
        "heptad" => 7,
        "octad" => 8,
        "ennead" => 9,
        "decad" => 10,
        "undecad" => 11,
        "dodecad" => 12,
        _ => return Ok(Json(ApiResponse::error("Invalid structure type".to_string()))),
    };

    if payload.terms.len() != expected_term_count {
        return Ok(Json(ApiResponse::error(format!(
            "Structure type '{}' requires exactly {} terms, got {}",
            payload.structure_type,
            expected_term_count,
            payload.terms.len()
        ))));
    }

    // Validate terms are not empty
    for (i, term) in payload.terms.iter().enumerate() {
        if term.trim().is_empty() {
            return Ok(Json(ApiResponse::error(format!(
                "Term at position {} cannot be empty",
                i + 1
            ))));
        }
    }

    // Store the structure
    match state.storage.store_structure_direct(
        &payload.name,
        &payload.structure_type,
        payload.terms,
        payload.connectives,
        payload.description,
    ).await {
        Ok(id) => Ok(Json(ApiResponse::success(id))),
        Err(e) => {
            eprintln!("Error creating structure: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
    
    println!("✅ Server successfully bound to {}", addr);
    println!("🌐 Server is now listening for connections...");
    
    axum::serve(listener, app)
        .await
        .map_err(|e| SystematicsError::Storage(format!("Server error: {}", e)))?;
    
    println!("⚠️  Server has stopped");
    Ok(())
} 