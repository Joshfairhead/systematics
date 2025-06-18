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
pub struct StructureSchema {
    pub structure_type: String,
    pub term_count: usize,
    pub canonical_terms: Vec<String>,
    pub coherence_attribute: String,
    pub term_designation: String,
    pub source: String,
    pub first_order_connectives_name: String,
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
        .route("/schema/:structure_type", get(get_structure_schema))
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
async fn test_simple_handler() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Simple test response".to_string()))
}

#[cfg(feature = "server")]
async fn get_structure_schema(
    Path(structure_type): Path<String>,
) -> Result<Json<ApiResponse<StructureSchema>>, StatusCode> {
    use systematics_library::System;
    
    let schema = match structure_type.as_str() {
        "triad" => {
            let system = systematics_library::TriadicSystem;
            StructureSchema {
                structure_type: "triad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "monad" => {
            let system = systematics_library::MonadicSystem;
            StructureSchema {
                structure_type: "monad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "dyad" => {
            let system = systematics_library::DyadicSystem;
            StructureSchema {
                structure_type: "dyad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "tetrad" => {
            let system = systematics_library::TetradicSystem;
            StructureSchema {
                structure_type: "tetrad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "pentad" => {
            let system = systematics_library::PentadicSystem;
            StructureSchema {
                structure_type: "pentad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "hexad" => {
            let system = systematics_library::HexadicSystem;
            StructureSchema {
                structure_type: "hexad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "heptad" => {
            let system = systematics_library::HeptadicSystem;
            StructureSchema {
                structure_type: "heptad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "octad" => {
            let system = systematics_library::OctadicSystem;
            StructureSchema {
                structure_type: "octad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "ennead" => {
            let system = systematics_library::EnneadicSystem;
            StructureSchema {
                structure_type: "ennead".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "decad" => {
            let system = systematics_library::DecadicSystem;
            StructureSchema {
                structure_type: "decad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "undecad" => {
            let system = systematics_library::UndecadicSystem;
            StructureSchema {
                structure_type: "undecad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        "dodecad" => {
            let system = systematics_library::DodecadicSystem;
            StructureSchema {
                structure_type: "dodecad".to_string(),
                term_count: system.term_count(),
                canonical_terms: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_name: system.first_order_connectives_name().to_string(),
            }
        },
        _ => {
            return Ok(Json(ApiResponse::error(format!(
                "Unknown structure type: {}. Valid types: monad, dyad, triad, tetrad, pentad, hexad, heptad, octad, ennead, decad, undecad, dodecad",
                structure_type
            ))));
        }
    };
    
    Ok(Json(ApiResponse::success(schema)))
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