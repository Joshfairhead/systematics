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

use crate::{SurrealStorage, StoredUserExpression, SystematicsError, DatabaseEnvironment};

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct AppState {
    pub storage: SurrealStorage,
}

// Global state for environment switching
#[cfg(feature = "server")]
static CURRENT_ENVIRONMENT: std::sync::OnceLock<std::sync::Arc<tokio::sync::RwLock<DatabaseEnvironment>>> = std::sync::OnceLock::new();

// Helper function to get storage for current environment
#[cfg(feature = "server")]
async fn get_current_storage() -> Result<SurrealStorage, SystematicsError> {
    if let Some(env_lock) = CURRENT_ENVIRONMENT.get() {
        let env = env_lock.read().await;
        SurrealStorage::new_with_environment(env.clone()).await
    } else {
        // Fallback to development environment
        SurrealStorage::new_with_environment(DatabaseEnvironment::Development).await
    }
}

#[cfg(feature = "server")]
#[derive(Deserialize)]
pub struct SwitchEnvironmentRequest {
    pub environment: String, // "testing" or "development"
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
    pub definition_type: String,
    pub user_expressions: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Deserialize)]
pub struct CreateUserInstanceRequest {
    pub name: String,
    pub definition_type: String,
    pub grammar_id: String,
    pub user_expressions: Vec<String>,
    pub connectives: HashMap<String, String>,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Serialize)]
pub struct ConnectiveInfo {
    pub from_index: usize,
    pub to_index: usize,
    pub relation_type: String,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Serialize)]
pub struct CoreGrammar {
    pub definition_type: String,
    pub name: String,
    pub term_characters: Vec<String>,
    pub coherence_attribute: String,
    pub term_designation: String,
    pub source: String,
    pub first_order_connectives_type: String,
    pub connectives: Vec<ConnectiveInfo>,
}

#[cfg(feature = "server")]
#[derive(Serialize)]
pub struct CommunityGrammar {
    pub id: serde_json::Value,
    pub definition_type: String,
    pub name: String,
    pub term_characters: Vec<String>,
    pub author: String,
    pub mapping_notes: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Deserialize)]
pub struct CreateCommunityGrammarRequest {
    pub definition_type: String,
    pub name: String,
    pub term_characters: Vec<String>,
    pub author: String,
    pub mapping_notes: String,
    pub description: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Serialize)]
pub struct SystemDefinition {
    pub definition_type: String,
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
    // Initialize global environment state
    let env = storage.environment().clone();
    CURRENT_ENVIRONMENT.get_or_init(|| std::sync::Arc::new(tokio::sync::RwLock::new(env)));
    
    let state = AppState { storage };

    Router::new()
        // Legacy endpoints for backward compatibility
        .route("/definitions", get(list_definitions))
        .route("/definitions", post(create_definition))
        .route("/definitions/search", get(search_definitions))
        .route("/definitions/:id", get(get_definition))
        .route("/definitions/:id", delete(delete_definition))
        .route("/definitions/:id/related", get(get_related_definitions))
        .route("/definition/:definition_type", get(get_system_definition))
        
        // Language Tetrad Architecture endpoints
        .route("/core-grammar/:definition_type", get(get_core_grammar))
        .route("/community-grammar", get(list_community_grammars))
        .route("/community-grammar", post(create_community_grammar))
        .route("/community-grammar/search", get(search_community_grammars))
        .route("/community-grammar/:id", get(get_community_grammar))
        .route("/community-grammar/:id", delete(delete_community_grammar))
        .route("/user-instances", get(list_user_expressions))
        .route("/user-instances", post(create_user_instance))
        .route("/user-instances/search", get(search_user_expressions))
        
        // Database environment management
        .route("/environment", get(get_current_environment))
        .route("/environment/switch", post(switch_environment))
        
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
) -> Result<Json<ApiResponse<Vec<StoredUserExpression>>>, StatusCode> {
    match state.storage.list_user_expressions().await {
        Ok(user_instances) => Ok(Json(ApiResponse::success(user_instances))),
        Err(e) => {
            eprintln!("Error listing user user_expressions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn search_definitions(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredUserExpression>>>, StatusCode> {
    let query = params.q.unwrap_or_default();
    
    match state.storage.search_user_expressions(&query).await {
        Ok(user_instances) => Ok(Json(ApiResponse::success(user_instances))),
        Err(e) => {
            eprintln!("Error searching user user_expressions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn get_definition(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<StoredUserExpression>>, StatusCode> {
    match state.storage.get_user_expression(&id).await {
        Ok(Some(user_instance)) => Ok(Json(ApiResponse::success(user_instance))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error getting user expression: {}", e);
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
    let valid_types = [
        "monad", "dyad", "triad", "tetrad", "pentad", "hexad", 
        "heptad", "octad", "ennead", "decad", "undecad", "dodecad"
    ];
    
    if !valid_types.contains(&payload.definition_type.as_str()) {
        return Ok(Json(ApiResponse::error(format!(
            "Invalid structure type: {}. Valid types: {:?}",
            payload.definition_type,
            valid_types
        ))));
    }

    // Validate term count matches structure type
    let expected_term_count = match payload.definition_type.as_str() {
        "monad" => 1, "dyad" => 2, "triad" => 3, "tetrad" => 4, "pentad" => 5, "hexad" => 6,
        "heptad" => 7, "octad" => 8, "ennead" => 9, "decad" => 10, "undecad" => 11, "dodecad" => 12,
        _ => return Ok(Json(ApiResponse::error("Invalid structure type".to_string()))),
    };

    if payload.user_expressions.len() != expected_term_count {
        return Ok(Json(ApiResponse::error(format!(
            "Structure type '{}' requires exactly {} user expressions, got {}",
            payload.definition_type,
            expected_term_count,
            payload.user_expressions.len()
        ))));
    }

    // Validate no empty user expressions
    for (i, user_expression) in payload.user_expressions.iter().enumerate() {
        if user_expression.trim().is_empty() {
            return Ok(Json(ApiResponse::error(format!(
                "User expression at position {} cannot be empty",
                i + 1
            ))));
        }
    }

    match state.storage.save_user_expression(
        &payload.name,
        &payload.definition_type,
        payload.user_expressions,
        payload.connectives,
        payload.description,
    ).await {
        Ok(id) => Ok(Json(ApiResponse::success(id))),
        Err(e) => {
            eprintln!("Error saving user expression: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn delete_definition(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    match state.storage.delete_user_expression(&id).await {
        Ok(deleted) => Ok(Json(ApiResponse::success(deleted))),
        Err(e) => {
            eprintln!("Error deleting user expression: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn get_related_definitions(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredUserExpression>>>, StatusCode> {
    match state.storage.get_related_user_expressions(&id).await {
        Ok(user_instances) => Ok(Json(ApiResponse::success(user_instances))),
        Err(e) => {
            eprintln!("Error getting related user user_expressions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn get_system_definition(
    Path(definition_type): Path<String>,
) -> Result<Json<ApiResponse<SystemDefinition>>, StatusCode> {
    use systematics_library::System;
    
    let definition = match definition_type.as_str() {
        "triad" => {
            let system = systematics_library::TriadicSystem;
            SystemDefinition {
                definition_type: "triad".to_string(),
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
                definition_type: "monad".to_string(),
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
                definition_type: "dyad".to_string(),
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
                from_index: c.from_position,
                to_index: c.to_position,
                relation_type: c.relationship,
                description: c.description,
            }).collect();
            SystemDefinition {
                definition_type: "tetrad".to_string(),
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
                from_index: c.from_position,
                to_index: c.to_position,
                relation_type: c.relationship,
                description: c.description,
            }).collect();
            SystemDefinition {
                definition_type: "pentad".to_string(),
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
                definition_type: "hexad".to_string(),
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
                definition_type: "heptad".to_string(),
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
                definition_type: "octad".to_string(),
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
                definition_type: "ennead".to_string(),
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
                definition_type: "decad".to_string(),
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
                definition_type: "undecad".to_string(),
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
                definition_type: "dodecad".to_string(),
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
                definition_type
            ))));
        }
    };
    
    Ok(Json(ApiResponse::success(definition)))
}

#[cfg(feature = "server")]
async fn get_core_grammar(
    Path(definition_type): Path<String>,
) -> Result<Json<ApiResponse<CoreGrammar>>, StatusCode> {
    use systematics_library::System;
    
    let grammar = match definition_type.as_str() {
        "monad" => {
            let system = systematics_library::MonadicSystem;
            CoreGrammar {
                definition_type: "monad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "dyad" => {
            let system = systematics_library::DyadicSystem;
            CoreGrammar {
                definition_type: "dyad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "triad" => {
            let system = systematics_library::TriadicSystem;
            CoreGrammar {
                definition_type: "triad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "tetrad" => {
            let system = systematics_library::TetradicSystem;
            CoreGrammar {
                definition_type: "tetrad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "pentad" => {
            let system = systematics_library::PentadicSystem;
            CoreGrammar {
                definition_type: "pentad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "hexad" => {
            let system = systematics_library::HexadicSystem;
            CoreGrammar {
                definition_type: "hexad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "heptad" => {
            let system = systematics_library::HeptadicSystem;
            CoreGrammar {
                definition_type: "heptad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "octad" => {
            let system = systematics_library::OctadicSystem;
            CoreGrammar {
                definition_type: "octad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "ennead" => {
            let system = systematics_library::EnneadicSystem;
            CoreGrammar {
                definition_type: "ennead".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "decad" => {
            let system = systematics_library::DecadicSystem;
            CoreGrammar {
                definition_type: "decad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "undecad" => {
            let system = systematics_library::UndecadicSystem;
            CoreGrammar {
                definition_type: "undecad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        "dodecad" => {
            let system = systematics_library::DodecadicSystem;
            CoreGrammar {
                definition_type: "dodecad".to_string(),
                name: system.name().to_string(),
                term_characters: system.term_characters().iter().map(|s| s.to_string()).collect(),
                coherence_attribute: system.coherence_attribute().to_string(),
                term_designation: system.term_designation().to_string(),
                source: system.source().to_string(),
                first_order_connectives_type: system.first_order_connectives_type().to_string(),
                connectives: system.connectives_traits().into_iter().map(|c| ConnectiveInfo {
                    from_index: c.from_position,
                    to_index: c.to_position,
                    relation_type: c.relationship,
                    description: c.description,
                }).collect(),
            }
        },
        _ => return Err(StatusCode::NOT_FOUND),
    };

    Ok(Json(ApiResponse::success(grammar)))
}

#[cfg(feature = "server")]
async fn list_user_expressions(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredUserExpression>>>, StatusCode> {
    match get_current_storage().await {
        Ok(storage) => {
            match storage.list_user_expressions().await {
                Ok(user_instances) => Ok(Json(ApiResponse::success(user_instances))),
                Err(e) => {
                    eprintln!("Error listing user user_expressions: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting current storage: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn search_user_expressions(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StoredUserExpression>>>, StatusCode> {
    let query = params.q.unwrap_or_default();
    
    match state.storage.search_user_expressions(&query).await {
        Ok(user_instances) => Ok(Json(ApiResponse::success(user_instances))),
        Err(e) => {
            eprintln!("Error searching user user_expressions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn create_user_instance(
    State(_state): State<AppState>,
    Json(payload): Json<CreateUserInstanceRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // Validate structure type
    let valid_types = ["monad", "dyad", "triad", "tetrad", "pentad", "hexad", "heptad", "octad", "ennead", "decad", "undecad", "dodecad"];
    if !valid_types.contains(&payload.definition_type.as_str()) {
        return Ok(Json(ApiResponse::error(format!(
            "Invalid structure type '{}'. Valid types: {}",
            payload.definition_type,
            valid_types.join(", ")
        ))));
    }

    // Validate user expression count matches structure type
    let expected_term_count = match payload.definition_type.as_str() {
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

    if payload.user_expressions.len() != expected_term_count {
        return Ok(Json(ApiResponse::error(format!(
            "Structure type '{}' requires exactly {} instances, got {}",
            payload.definition_type,
            expected_term_count,
            payload.user_expressions.len()
        ))));
    }

    // Validate instances are not empty
    for (i, instance) in payload.user_expressions.iter().enumerate() {
        if instance.trim().is_empty() {
            return Ok(Json(ApiResponse::error(format!(
                "Instance at position {} cannot be empty",
                i + 1
            ))));
        }
    }

    // Store the user expression
    match get_current_storage().await {
        Ok(storage) => {
            match storage.save_user_expression(
                &payload.name,
                &payload.definition_type,
                payload.user_expressions,
                payload.connectives,
                payload.description,
            ).await {
                Ok(id) => Ok(Json(ApiResponse::success(id))),
                Err(e) => {
                    eprintln!("Error creating user expression: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting current storage: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Community Grammar handlers
#[cfg(feature = "server")]
async fn list_community_grammars(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<CommunityGrammar>>>, StatusCode> {
    let definition_type = params.get("definition_type").map(|s| s.as_str());
    
    match state.storage.list_community_grammars(definition_type).await {
        Ok(stored_grammars) => {
            let community_grammars: Vec<CommunityGrammar> = stored_grammars
                .into_iter()
                .map(|stored| CommunityGrammar {
                    id: serde_json::to_value(&stored.id).unwrap_or(serde_json::Value::Null),
                    definition_type: stored.definition_type,
                    name: stored.name,
                    term_characters: stored.term_characters,
                    author: stored.author,
                    mapping_notes: stored.mapping_notes,
                    created_at: stored.created_at.to_string(),
                    updated_at: stored.updated_at.to_string(),
                    description: stored.description,
                })
                .collect();
            
            Ok(Json(ApiResponse::success(community_grammars)))
        },
        Err(e) => {
            eprintln!("Error listing community grammars: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn get_community_grammar(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<CommunityGrammar>>, StatusCode> {
    match state.storage.get_community_grammar(&id).await {
        Ok(Some(stored)) => {
            let community_grammar = CommunityGrammar {
                id: serde_json::to_value(&stored.id).unwrap_or(serde_json::Value::Null),
                definition_type: stored.definition_type,
                name: stored.name,
                term_characters: stored.term_characters,
                author: stored.author,
                mapping_notes: stored.mapping_notes,
                created_at: stored.created_at.to_string(),
                updated_at: stored.updated_at.to_string(),
                description: stored.description,
            };
            Ok(Json(ApiResponse::success(community_grammar)))
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error getting community grammar: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn create_community_grammar(
    State(state): State<AppState>,
    Json(payload): Json<CreateCommunityGrammarRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // Validate structure type
    let valid_types = ["monad", "dyad", "triad", "tetrad", "pentad", "hexad", "heptad", "octad", "ennead", "decad", "undecad", "dodecad"];
    if !valid_types.contains(&payload.definition_type.as_str()) {
        return Ok(Json(ApiResponse::error(format!(
            "Invalid structure type '{}'. Valid types: {}",
            payload.definition_type,
            valid_types.join(", ")
        ))));
    }

    // Validate term characters count matches structure type
    let expected_term_count = match payload.definition_type.as_str() {
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

    if payload.term_characters.len() != expected_term_count {
        return Ok(Json(ApiResponse::error(format!(
            "Structure type '{}' requires exactly {} term characters, got {}",
            payload.definition_type,
            expected_term_count,
            payload.term_characters.len()
        ))));
    }

    // Validate term characters are not empty
    for (i, term) in payload.term_characters.iter().enumerate() {
        if term.trim().is_empty() {
            return Ok(Json(ApiResponse::error(format!(
                "Term character at position {} cannot be empty",
                i + 1
            ))));
        }
    }

    // Validate required fields
    if payload.name.trim().is_empty() {
        return Ok(Json(ApiResponse::error("Name cannot be empty".to_string())));
    }
    if payload.author.trim().is_empty() {
        return Ok(Json(ApiResponse::error("Author cannot be empty".to_string())));
    }

    // Store the community grammar
    match state.storage.create_community_grammar(
        &payload.definition_type,
        &payload.name,
        payload.term_characters,
        &payload.author,
        &payload.mapping_notes,
        payload.description,
    ).await {
        Ok(id) => Ok(Json(ApiResponse::success(id))),
        Err(e) => {
            eprintln!("Error creating community grammar: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn search_community_grammars(
    Query(params): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<CommunityGrammar>>>, StatusCode> {
    let query = params.q.unwrap_or_default();
    
    match state.storage.search_community_grammars(&query).await {
        Ok(stored_grammars) => {
            let community_grammars: Vec<CommunityGrammar> = stored_grammars
                .into_iter()
                .map(|stored| CommunityGrammar {
                    id: serde_json::to_value(&stored.id).unwrap_or(serde_json::Value::Null),
                    definition_type: stored.definition_type,
                    name: stored.name,
                    term_characters: stored.term_characters,
                    author: stored.author,
                    mapping_notes: stored.mapping_notes,
                    created_at: stored.created_at.to_string(),
                    updated_at: stored.updated_at.to_string(),
                    description: stored.description,
                })
                .collect();
            
            Ok(Json(ApiResponse::success(community_grammars)))
        },
        Err(e) => {
            eprintln!("Error searching community grammars: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(feature = "server")]
async fn delete_community_grammar(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    match state.storage.delete_community_grammar(&id).await {
        Ok(deleted) => Ok(Json(ApiResponse::success(deleted))),
        Err(e) => {
            eprintln!("Error deleting community grammar: {}", e);
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

#[cfg(feature = "server")]
async fn get_current_environment(
    State(_state): State<AppState>,
) -> Json<ApiResponse<String>> {
    if let Some(env_lock) = CURRENT_ENVIRONMENT.get() {
        let env = env_lock.read().await;
        let environment = match *env {
            DatabaseEnvironment::Testing => "testing",
            DatabaseEnvironment::Development => "development",
        };
        Json(ApiResponse::success(environment.to_string()))
    } else {
        Json(ApiResponse::error("Environment not initialized".to_string()))
    }
}

#[cfg(feature = "server")]
async fn switch_environment(
    State(_state): State<AppState>,
    Json(payload): Json<SwitchEnvironmentRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let new_environment = match payload.environment.as_str() {
        "testing" => DatabaseEnvironment::Testing,
        "development" => DatabaseEnvironment::Development,
        _ => return Ok(Json(ApiResponse::error("Invalid environment. Use 'testing' or 'development'".to_string()))),
    };
    
    // Test the new environment connection
    match SurrealStorage::new_with_environment(new_environment.clone()).await {
        Ok(_new_storage) => {
            // Update global environment state
            if let Some(env_lock) = CURRENT_ENVIRONMENT.get() {
                let mut env = env_lock.write().await;
                *env = new_environment.clone();
            }
            
            let env_name = match new_environment {
                DatabaseEnvironment::Testing => "testing",
                DatabaseEnvironment::Development => "development",
            };
            Ok(Json(ApiResponse::success(format!("Switched to {} environment", env_name))))
        }
        Err(e) => {
            eprintln!("Error switching environment: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to switch environment: {}", e))))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app_state = AppState {
            storage: Arc::new(SurrealStorage::new("memory").await.unwrap()),
        };
        let app = create_router(app_state);
        let server = TestServer::new(app).unwrap();
        
        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }
} 