use yew::{html, Component, Context, Html, TargetCast};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use web_sys::{HtmlInputElement, InputEvent, window};

mod components; // Declare the components module
mod services;   // Declare the services module
mod core;       // Declare the core module (framework-agnostic)

use components::system_selector::SystemSelector; // Import the SystemSelector
use components::system_overlay::SystemOverlay;
use components::geometric_renderer::GeometricRenderer;
use services::api::{ApiClient, StoredUserDefinition, SystemDefinition, spawn_api_call};

pub struct App {
    // Replace hardcoded system selection with dynamic data
    definitions: Vec<StoredUserDefinition>,
    filtered_definitions: Vec<StoredUserDefinition>,
    selected_definition: Option<StoredUserDefinition>,
    current_definition: Option<SystemDefinition>,
    current_system_num: i32, // Track the currently selected system
    loading: bool,
    error: Option<String>,
    success_message: Option<String>,
    api_client: ApiClient,
    search_query: String,
    show_structure_browser: bool,
    // Creation state
    creation_mode: bool,
    structure_name: Option<String>,
    user_instance_index: Vec<String>, // Track user input for each position
    saving: bool,
}

pub enum Msg {
    // Keep the old message for backward compatibility during transition
    SystemSelected(i32),
    // New messages for API integration
    DefinitionSelected(StoredUserDefinition),
    LoadDefinitions,
    DefinitionsLoaded(Result<Vec<StoredUserDefinition>, anyhow::Error>),
    ApiError(String),
    // Search and browse functionality
    SearchQueryChanged(String),
    ToggleStructureBrowser,
    SearchDefinitions,
    SearchResultsLoaded(Result<Vec<StoredUserDefinition>, anyhow::Error>),
    // System definition loading
    DefinitionLoaded(Result<SystemDefinition, anyhow::Error>),
    // Creation functionality
    CreateDefinition,
    CancelCreate,
    SaveDefinition,
    DefinitionSaved(Result<String, anyhow::Error>),
    UserInstanceChanged(usize, String),
    ClearNotifications,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let app = Self {
            definitions: Vec::new(), // Start with empty definitions, load from API
            filtered_definitions: Vec::new(),
            selected_definition: None, // No initial selection, let user choose
            current_definition: None,
            current_system_num: 1, // Default to monad
            loading: true,
            error: None,
            success_message: None,
            api_client: ApiClient::new(),
            search_query: String::new(),
            show_structure_browser: false,
            creation_mode: false,
            structure_name: None,
            user_instance_index: vec![String::new(); 8], // Initialize user_instance_index with 8 empty strings
            saving: false,
        };
        
        // Load definitions on component creation
        ctx.link().send_message(Msg::LoadDefinitions);
        
                    // Load definition for the initially selected system (monad)
        ctx.link().send_message(Msg::SystemSelected(1));
        
        app
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SystemSelected(system_num) => {
                // Update current system selection
                self.current_system_num = system_num;
                
                let structure_type = match system_num {
                    1 => "monad",
                    2 => "dyad", 
                    3 => "triad",
                    4 => "tetrad",
                    5 => "pentad",
                    6 => "hexad",
                    7 => "heptad",
                    8 => "octad",
                    9 => "ennead",
                    10 => "decad",
                    11 => "undecad",
                    12 => "dodecad",
                    _ => return false,
                };
                
                // Find matching definition from loaded data (don't create placeholders)
                if let Some(definition) = self.definitions.iter()
                    .find(|s| s.structure_type == structure_type && !s.id.as_str().map_or(false, |id| id.starts_with("placeholder-")))
                    .cloned() 
                {
                    self.selected_definition = Some(definition);
                } else {
                    // No real definition found - clear selection to show definition data
                    self.selected_definition = None;
                }
                
                // Always load definition for the selected structure type
                self.load_definition_for_structure_type(ctx, structure_type);
                true
            }
            Msg::DefinitionSelected(definition) => {
                let structure_type = definition.structure_type.clone();
                self.selected_definition = Some(definition);
                self.show_structure_browser = false; // Close browser after selection
                
                // Load definition for the selected structure type
                self.load_definition_for_structure_type(ctx, &structure_type);
                self.current_system_num = self.structure_type_to_number(&structure_type);
                true
            }
            Msg::LoadDefinitions => {
                self.loading = true;
                self.error = None;
                let api_client = self.api_client.clone();
                let callback = ctx.link().callback(Msg::DefinitionsLoaded);
                
                spawn_api_call(
                    async move { api_client.list_definitions().await },
                    callback
                );
                true
            }
            Msg::DefinitionsLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(definitions) => {
                        self.definitions = definitions;
                        self.filtered_definitions = self.definitions.clone();
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to load definitions: {}", e));
                        // Don't create placeholder data - let the system work with empty definitions and definition data
                        self.definitions = Vec::new();
                        self.filtered_definitions = Vec::new();
                    }
                }
                true
            }
            Msg::SearchQueryChanged(query) => {
                self.search_query = query;
                self.filter_definitions();
                true
            }
            Msg::ToggleStructureBrowser => {
                self.show_structure_browser = !self.show_structure_browser;
                true
            }
            Msg::SearchDefinitions => {
                if self.search_query.trim().is_empty() {
                    self.filtered_definitions = self.definitions.clone();
                    return true;
                }
                
                self.loading = true;
                let api_client = self.api_client.clone();
                let query = self.search_query.clone();
                let callback = ctx.link().callback(Msg::SearchResultsLoaded);
                
                spawn_api_call(
                    async move { api_client.search_definitions(&query).await },
                    callback
                );
                true
            }
            Msg::SearchResultsLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(definitions) => {
                        self.filtered_definitions = definitions;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Search failed: {}", e));
                        self.filter_definitions(); // Fallback to local filtering
                    }
                }
                true
            }
            Msg::ApiError(error) => {
                self.error = Some(error);
                true
            }
            Msg::DefinitionLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(definition) => {
                        self.current_definition = Some(definition);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to load schema: {}", e));
                    }
                }
                true
            }
            Msg::CreateDefinition => {
                if let Some(window) = window() {
                    if let Ok(Some(name)) = window.prompt_with_message("Enter a name for your definition:") {
                        if !name.trim().is_empty() {
                            self.structure_name = Some(name.trim().to_string());
                            self.creation_mode = true;
                            
                            // Initialize user_instance_index with the right number of empty strings
                            let term_count = if let Some(ref definition) = self.selected_definition {
                                self.structure_type_to_number(&definition.structure_type) as usize
                            } else {
                                1 // Default to monad
                            };
                            self.user_instance_index = vec![String::new(); term_count];
                            
                            return true;
                        }
                    }
                }
                false
            }
            Msg::CancelCreate => {
                self.creation_mode = false;
                self.structure_name = None;
                self.user_instance_index.clear();
                true
            }
            Msg::SaveDefinition => {
                self.saving = true;
                self.save_definition(ctx);
                true
            }
            Msg::DefinitionSaved(result) => {
                self.saving = false;
                match result {
                    Ok(definition_id) => {
                        self.error = None;
                        self.success_message = Some(format!("✅ Definition saved successfully!"));
                        // Exit creation mode after successful save
                        self.creation_mode = false;
                        self.structure_name = None;
                        self.user_instance_index.clear();
                        // Reload definitions to show the new one
                        self.load_definitions(ctx);
                        // Auto-dismiss notification after 3 seconds
                        let link = ctx.link().clone();
                        gloo_timers::callback::Timeout::new(3000, move || {
                            link.send_message(Msg::ClearNotifications);
                        }).forget();
                    }
                    Err(e) => {
                        self.success_message = None;
                        self.error = Some(format!("Failed to save definition: {}", e));
                        // Auto-dismiss error notification after 5 seconds
                        let link = ctx.link().clone();
                        gloo_timers::callback::Timeout::new(5000, move || {
                            link.send_message(Msg::ClearNotifications);
                        }).forget();
                    }
                }
                true
            }
            Msg::UserInstanceChanged(index, instance) => {
                self.user_instance_index[index] = instance;
                true
            }
            Msg::ClearNotifications => {
                self.success_message = None;
                self.error = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_system_selected = ctx.link().callback(Msg::SystemSelected);
        
        // Determine the structure type and system number from selected definition or current selection
        let (structure_type, system_num) = if let Some(ref definition) = self.selected_definition {
            let num = self.structure_type_to_number(&definition.structure_type);
            (definition.structure_type.clone(), num)
        } else {
            // Use current system selection instead of defaulting to monad
            let structure_type = match self.current_system_num {
                1 => "monad",
                2 => "dyad",
                3 => "triad",
                4 => "tetrad",
                5 => "pentad",
                6 => "hexad",
                7 => "heptad",
                8 => "octad",
                9 => "ennead",
                10 => "decad",
                11 => "undecad",
                12 => "dodecad",
                _ => "monad",
            };
            (structure_type.to_string(), self.current_system_num)
        };

        // Create system-specific CSS class
        let system_class = format!("system-{}", system_num);

        html! {
            <div class="app-container">
                {self.render_header(ctx)}
                {self.render_search_controls(ctx)}
                <div class="system-selector-container">
                    <SystemSelector {on_system_selected} selected_system={system_num} />
                </div>
                <div class={format!("main-content {}", system_class)}>
                    {self.render_loading_or_error()}
                    <div class="geometric-container">
                        <GeometricRenderer 
                            system_type={structure_type} 
                            size={400.0}
                            connectives={self.current_definition.as_ref().map(|s| s.connectives.clone())}
                        />
                        {self.render_structure_overlay(ctx)}
                    </div>
                </div>
                {self.render_structure_browser(ctx)}
                {self.render_notifications()}
            </div>
        }
    }
}

impl App {
    fn render_header(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header class="app-header">
                <div class="header-content">
                    <div class="header-title">
                        <h1>{"SysteMaster"}</h1>
                        <p>{"Systematic Thinking Framework"}</p>
                    </div>
                </div>
            </header>
        }
    }
    
    fn render_search_controls(&self, ctx: &Context<Self>) -> Html {
        let search_input = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::SearchQueryChanged(input.value())
        });
        
        let search_submit = ctx.link().callback(|_| Msg::SearchDefinitions);
        let create_callback = ctx.link().callback(|_| Msg::CreateDefinition);
        let cancel_callback = ctx.link().callback(|_| Msg::CancelCreate);
        
        html! {
            <div class="search-controls">
                <div class="search-bar">
                    <div class="action-buttons">
                        <button class="load-button" onclick={ctx.link().callback(|_| Msg::ToggleStructureBrowser)}>
                            {"Load"}
                        </button>
                        {if !self.creation_mode {
                            html! {
                                <button class="create-button" onclick={create_callback}>
                                    {"Create"}
                                </button>
                            }
                        } else {
                            html! {
                                <>
                                    <button class="save-button" onclick={ctx.link().callback(|_| Msg::SaveDefinition)} disabled={self.saving}>
                                        {if self.saving { "Saving..." } else { "Save" }}
                                    </button>
                                    <button class="cancel-button" onclick={cancel_callback}>
                                        {"Cancel"}
                                    </button>
                                </>
                            }
                        }}
                    </div>
                    <input 
                        type="text" 
                        placeholder="Search definitions by name, type, or user instances..." 
                        value={self.search_query.clone()}
                        oninput={search_input}
                        class="search-input"
                    />
                    <button onclick={search_submit} class="search-button">
                        {"Search"}
                    </button>
                </div>
            </div>
        }
    }
    
    fn render_loading_or_error(&self) -> Html {
        if self.loading {
            html! {
                <div class="loading">
                    {"Loading definitions..."}
                </div>
            }
        } else {
            html! {}
        }
    }

    fn render_notifications(&self) -> Html {
        html! {
            <>
                {if let Some(ref success) = self.success_message {
                    html! {
                        <div class="notification success">
                            <p>{success}</p>
                        </div>
                    }
                } else {
                    html! {}
                }}
                {if let Some(ref error) = self.error {
                    html! {
                        <div class="notification error">
                            <p>{"⚠️ "}{error}</p>
                            <small>{"Using fallback data"}</small>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </>
        }
    }

    fn render_structure_overlay(&self, ctx: &Context<Self>) -> Html {
        if let Some(ref definition) = self.selected_definition {
            let system_num = self.structure_type_to_number(&definition.structure_type);
            html! {
                <SystemOverlay 
                    system_num={system_num} 
                    definition={definition.clone()} 
                    creation_mode={self.creation_mode}
                    structure_name={self.structure_name.clone()}
                    user_instance_index={self.user_instance_index.clone()}
                    on_instance_change={ctx.link().callback(|(index, instance)| Msg::UserInstanceChanged(index, instance))}
                />
            }
        } else {
            html! {
                <SystemOverlay 
                    system_num={self.current_system_num} 
                    creation_mode={self.creation_mode}
                    structure_name={self.structure_name.clone()}
                    user_instance_index={self.user_instance_index.clone()}
                    on_instance_change={ctx.link().callback(|(index, instance)| Msg::UserInstanceChanged(index, instance))}
                />
            }
        }
    }

    fn render_structure_browser(&self, ctx: &Context<Self>) -> Html {
        if !self.show_structure_browser {
            return html! {};
        }
        
        let definitions_to_show = if self.search_query.trim().is_empty() {
            &self.definitions
        } else {
            &self.filtered_definitions
        };
        
        html! {
            <div class="structure-browser-overlay">
                <div class="structure-browser">
                    <div class="browser-header">
                        <h3>{"Available Definitions"}</h3>
                        <button 
                            class="close-browser" 
                            onclick={ctx.link().callback(|_| Msg::ToggleStructureBrowser)}
                        >
                            {"×"}
                        </button>
                    </div>
                    <div class="structure-list">
                        {for definitions_to_show.iter().map(|definition| {
                            let definition_clone = definition.clone();
                            let select_definition = ctx.link().callback(move |_| {
                                Msg::DefinitionSelected(definition_clone.clone())
                            });
                            
                            html! {
                                <div class="structure-item" onclick={select_definition}>
                                    <div class="structure-item-header">
                                        <h4>{&definition.name}</h4>
                                        <span class="structure-type">{&definition.structure_type}</span>
                                    </div>
                                                        <div class="structure-item-terms">
                        {definition.user_instance_index.join(", ")}
                    </div>
                                    {if let Some(ref desc) = definition.description {
                                        html! { 
                                            <div class="structure-item-description">
                                                {desc}
                                            </div> 
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </div>
                            }
                        })}
                    </div>
                    {if definitions_to_show.is_empty() {
                        html! {
                            <div class="no-results">
                                {"No definitions found"}
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        }
    }
    
    fn filter_definitions(&mut self) {
        if self.search_query.trim().is_empty() {
            self.filtered_definitions = self.definitions.clone();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_definitions = self.definitions
                .iter()
                .filter(|definition| {
                    definition.name.to_lowercase().contains(&query) ||
                    definition.structure_type.to_lowercase().contains(&query) ||
                    definition.user_instance_index.iter().any(|instance| instance.to_lowercase().contains(&query)) ||
                    definition.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query))
                })
                .cloned()
                .collect();
        }
    }

    fn structure_type_to_number(&self, structure_type: &str) -> i32 {
        match structure_type {
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
            _ => 1,
        }
    }

    fn create_placeholder_definition(&self, structure_type: &str, system_num: i32) -> StoredUserDefinition {
        let user_instance_index = match system_num {
            1 => vec!["Unity".to_string()],
            2 => vec!["Essence".to_string(), "Existence".to_string()],
            3 => vec!["Active".to_string(), "Passive".to_string(), "Reconciling".to_string()],
            4 => vec!["Ground".to_string(), "Ideal".to_string(), "Instrumental".to_string(), "Directive".to_string()],
            5 => vec!["Purpose".to_string(), "Higher Potential".to_string(), "Quintessence".to_string(), "Lower Potential".to_string(), "Source".to_string()],
            6 => vec!["Resources".to_string(), "Values".to_string(), "Options".to_string(), "Criteria".to_string(), "Facts".to_string(), "Priorities".to_string()],
            7 => vec!["Insight".to_string(), "Research".to_string(), "Design".to_string(), "Synthesis".to_string(), "Application".to_string(), "Delivery".to_string(), "Value".to_string()],
            8 => vec!["Element 1".to_string(), "Element 2".to_string(), "Element 3".to_string(), "Element 4".to_string(), "Element 5".to_string(), "Element 6".to_string(), "Element 7".to_string(), "Element 8".to_string()],
            _ => (1..=system_num).map(|i| format!("Term {}", i)).collect(),
        };

        StoredUserDefinition {
            id: serde_json::Value::String(format!("placeholder-{}", structure_type)),
            name: format!("Default {}", structure_type.to_uppercase()),
            structure_type: structure_type.to_string(),
            user_instance_index,
            connectives: HashMap::new(),
            created_at: "placeholder".to_string(),
            updated_at: "placeholder".to_string(),
            description: Some(format!("Default {} definition", structure_type)),
            metadata: HashMap::new(),
        }
    }

    fn create_placeholder_definitions(&self) -> Vec<StoredUserDefinition> {
        (1..=8).map(|i| {
            let structure_type = match i {
                1 => "monad",
                2 => "dyad",
                3 => "triad", 
                4 => "tetrad",
                5 => "pentad",
                6 => "hexad",
                7 => "heptad",
                8 => "octad",
                _ => "unknown",
            };
            self.create_placeholder_definition(structure_type, i)
        }).collect()
    }

    // Static method for initial creation
    fn create_initial_placeholder_definitions() -> Vec<StoredUserDefinition> {
        (1..=8).map(|i| {
            let structure_type = match i {
                1 => "monad",
                2 => "dyad",
                3 => "triad", 
                4 => "tetrad",
                5 => "pentad",
                6 => "hexad",
                7 => "heptad",
                8 => "octad",
                _ => "unknown",
            };
            Self::create_static_placeholder_definition(structure_type, i)
        }).collect()
    }

    fn create_static_placeholder_definition(structure_type: &str, system_num: i32) -> StoredUserDefinition {
        let user_instance_index = match system_num {
            1 => vec!["Unity".to_string()],
            2 => vec!["Essence".to_string(), "Existence".to_string()],
            3 => vec!["Active".to_string(), "Passive".to_string(), "Reconciling".to_string()],
            4 => vec!["Ground".to_string(), "Ideal".to_string(), "Instrumental".to_string(), "Directive".to_string()],
            5 => vec!["Purpose".to_string(), "Higher Potential".to_string(), "Quintessence".to_string(), "Lower Potential".to_string(), "Source".to_string()],
            6 => vec!["Resources".to_string(), "Values".to_string(), "Options".to_string(), "Criteria".to_string(), "Facts".to_string(), "Priorities".to_string()],
            7 => vec!["Insight".to_string(), "Research".to_string(), "Design".to_string(), "Synthesis".to_string(), "Application".to_string(), "Delivery".to_string(), "Value".to_string()],
            8 => vec!["Element 1".to_string(), "Element 2".to_string(), "Element 3".to_string(), "Element 4".to_string(), "Element 5".to_string(), "Element 6".to_string(), "Element 7".to_string(), "Element 8".to_string()],
            _ => (1..=system_num).map(|i| format!("Term {}", i)).collect(),
        };

        StoredUserDefinition {
            id: serde_json::Value::String(format!("placeholder-{}", structure_type)),
            name: format!("Default {}", structure_type.to_uppercase()),
            structure_type: structure_type.to_string(),
            user_instance_index,
            connectives: HashMap::new(),
            created_at: "placeholder".to_string(),
            updated_at: "placeholder".to_string(),
            description: Some(format!("Default {} definition", structure_type)),
            metadata: HashMap::new(),
        }
    }

    fn load_definition_for_structure_type(&self, ctx: &Context<Self>, structure_type: &str) {
        let api_client = self.api_client.clone();
        let structure_type = structure_type.to_string();
        let callback = ctx.link().callback(Msg::DefinitionLoaded);
        
        spawn_api_call(
            async move {
                api_client.get_system_definition(&structure_type).await
            },
            callback,
        );
    }

    fn load_definitions(&self, ctx: &Context<Self>) {
        let api_client = self.api_client.clone();
        let callback = ctx.link().callback(Msg::DefinitionsLoaded);
        
        spawn_api_call(
            async move {
                api_client.list_definitions().await
            },
            callback,
        );
    }

    fn save_definition(&self, ctx: &Context<Self>) {
        let structure_type = if let Some(definition) = &self.selected_definition {
            definition.structure_type.clone()
        } else {
            // Default to monad if no definition is selected
            "monad".to_string()
        };
        
        let definition_name = self.structure_name.clone().unwrap_or_else(|| "Unnamed Definition".to_string());
        
        let api_client = self.api_client.clone();
        let user_instances = self.user_instance_index.clone();
        let callback = ctx.link().callback(Msg::DefinitionSaved);
        
        spawn_api_call(
            async move {
                api_client.save_definition(&definition_name, &structure_type, &user_instances).await
            },
            callback,
        );
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
} 