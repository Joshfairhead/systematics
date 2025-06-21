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

use crate::{SurrealStorage, StoredUserDefinition, SystematicsError};

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
    pub user_instance_index: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Serialize)]
pub struct ConnectiveInfo {
    pub from_position: usize,
    pub to_position: usize,
    pub relationship: String,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Serialize)]
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
        .route("/definitions", get(list_definitions))
        .route("/definitions", post(create_definition))
        .route("/definitions/search", get(search_definitions))
        .route("/definitions/:id", get(get_definition))
        .route("/definitions/:id", delete(delete_definition))
        .route("/definitions/:id/related", get(get_related_definitions))
        .route("/definition/:structure_type", get(get_system_definition))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#[cfg(feature = "server")]
async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("SysteMaster API is running"))
}

#[cfg(feature = "server")]
async fn list_definitions(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredUserDefinition>>>, StatusCode> {
    match state.storage.list_definitions().await {
        Ok(definitions) => Ok(Json(ApiResponse::success(definitions))),
        Err(e) => {
            eprintln!("Error listing definitions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn search_definitions(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredUserDefinition>>>, StatusCode> {
    let query = params.q.unwrap_or_default();
    
    match state.storage.search_definitions(&query).await {
        Ok(definitions) => Ok(Json(ApiResponse::success(definitions))),
        Err(e) => {
            eprintln!("Error searching definitions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn get_definition(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<StoredUserDefinition>>, StatusCode> {
    match state.storage.get_definition(&id).await {
        Ok(Some(definition)) => Ok(Json(ApiResponse::success(definition))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error getting definition: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn create_definition(
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

    // Validate user instance count matches structure type
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

    if payload.user_instance_index.len() != expected_term_count {
        return Ok(Json(ApiResponse::error(format!(
            "Structure type '{}' requires exactly {} user instances, got {}",
            payload.structure_type,
            expected_term_count,
            payload.user_instance_index.len()
        ))));
    }

    // Validate user instances are not empty
    for (i, user_instance) in payload.user_instance_index.iter().enumerate() {
        if user_instance.trim().is_empty() {
            return Ok(Json(ApiResponse::error(format!(
                "User instance at position {} cannot be empty",
                i + 1
            ))));
        }
    }

    // Store the definition
    match state.storage.store_definition_direct(
        &payload.name,
        &payload.structure_type,
        payload.user_instance_index,
        payload.connectives,
        payload.description,
    ).await {
        Ok(id) => Ok(Json(ApiResponse::success(id))),
        Err(e) => {
            eprintln!("Error creating definition: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn delete_definition(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    match state.storage.delete_definition(&id).await {
        Ok(deleted) => Ok(Json(ApiResponse::success(deleted))),
        Err(e) => {
            eprintln!("Error deleting definition: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn get_related_definitions(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredUserDefinition>>>, StatusCode> {
    match state.storage.get_related_definitions(&id).await {
        Ok(definitions) => Ok(Json(ApiResponse::success(definitions))),
        Err(e) => {
            eprintln!("Error getting related definitions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn test_simple_handler() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Simple test response".to_string()))
}

#[cfg(feature = "server")]
async fn get_system_definition(
    Path(structure_type): Path<String>,
) -> Result<Json<ApiResponse<SystemDefinition>>, StatusCode> {
    use systematics_library::System;
    
    let definition = match structure_type.as_str() {
        "triad" => {
            let system = systematics_library::TriadicSystem;
            SystemDefinition {
                structure_type: "triad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "monad" => {
            let system = systematics_library::MonadicSystem;
            SystemDefinition {
                structure_type: "monad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "dyad" => {
            let system = systematics_library::DyadicSystem;
            SystemDefinition {
                structure_type: "dyad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "tetrad" => {
            let system = systematics_library::TetradicSystem;
            let connectives = system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                from_position: c.from_position,
                to_position: c.to_position,
                relationship: c.relationship,
                description: c.description,
            }).collect();
            SystemDefinition {
                structure_type: "tetrad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives,
            }
        },
        "pentad" => {
            let system = systematics_library::PentadicSystem;
            let connectives = system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                from_position: c.from_position,
                to_position: c.to_position,
                relationship: c.relationship,
                description: c.description,
            }).collect();
            SystemDefinition {
                structure_type: "pentad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives,
            }
        },
        "hexad" => {
            let system = systematics_library::HexadicSystem;
            SystemDefinition {
                structure_type: "hexad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "heptad" => {
            let system = systematics_library::HeptadicSystem;
            SystemDefinition {
                structure_type: "heptad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "octad" => {
            let system = systematics_library::OctadicSystem;
            SystemDefinition {
                structure_type: "octad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "ennead" => {
            let system = systematics_library::EnneadicSystem;
            SystemDefinition {
                structure_type: "ennead".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "decad" => {
            let system = systematics_library::DecadicSystem;
            SystemDefinition {
                structure_type: "decad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "undecad" => {
            let system = systematics_library::UndecadicSystem;
            SystemDefinition {
                structure_type: "undecad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        "dodecad" => {
            let system = systematics_library::DodecadicSystem;
            SystemDefinition {
                structure_type: "dodecad".to_string(),
                term_count: system.term_count(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: Vec::new(),
            }
        },
        _ => {
            return Ok(Json(ApiResponse::error(format!(
                "Unknown structure type: {}. Valid types: monad, dyad, triad, tetrad, pentad, hexad, heptad, octad, ennead, decad, undecad, dodecad",
                structure_type
            ))));
        }
    };
    
    Ok(Json(ApiResponse::success(definition)))
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