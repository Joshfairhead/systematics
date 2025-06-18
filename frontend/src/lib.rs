use yew::{html, Component, Context, Html, TargetCast};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use web_sys::{HtmlInputElement, InputEvent};

mod components; // Declare the components module
mod services;   // Declare the services module
mod core;       // Declare the core module (framework-agnostic)

use components::system_selector::SystemSelector; // Import the SystemSelector
use components::system_overlay::SystemOverlay;
use components::geometric_renderer::GeometricRenderer;
use services::api::{ApiClient, StoredStructure, spawn_api_call};

pub struct App {
    // Replace hardcoded system selection with dynamic data
    structures: Vec<StoredStructure>,
    filtered_structures: Vec<StoredStructure>,
    selected_structure: Option<StoredStructure>,
    loading: bool,
    error: Option<String>,
    api_client: ApiClient,
    search_query: String,
    show_structure_browser: bool,
}

pub enum Msg {
    // Keep the old message for backward compatibility during transition
    SystemSelected(i32),
    // New messages for API integration
    StructureSelected(StoredStructure),
    LoadStructures,
    StructuresLoaded(Result<Vec<StoredStructure>, anyhow::Error>),
    ApiError(String),
    // Search and browse functionality
    SearchQueryChanged(String),
    ToggleStructureBrowser,
    SearchStructures,
    SearchResultsLoaded(Result<Vec<StoredStructure>, anyhow::Error>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let app = Self {
            structures: Vec::new(),
            filtered_structures: Vec::new(),
            selected_structure: None,
            loading: true,
            error: None,
            api_client: ApiClient::new(),
            search_query: String::new(),
            show_structure_browser: false,
        };
        
        // Load structures on component creation
        ctx.link().send_message(Msg::LoadStructures);
        
        app
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SystemSelected(system_num) => {
                // Legacy support - find structure by type
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
                
                // Find matching structure from loaded data
                if let Some(structure) = self.structures.iter()
                    .find(|s| s.structure_type == structure_type)
                    .cloned() 
                {
                    self.selected_structure = Some(structure);
                    true
                } else {
                    // If no API data yet, create a placeholder
                    self.selected_structure = Some(self.create_placeholder_structure(structure_type, system_num));
                    true
                }
            }
            Msg::StructureSelected(structure) => {
                self.selected_structure = Some(structure);
                self.show_structure_browser = false; // Close browser after selection
                true
            }
            Msg::LoadStructures => {
                self.loading = true;
                self.error = None;
                let api_client = self.api_client.clone();
                let callback = ctx.link().callback(Msg::StructuresLoaded);
                
                spawn_api_call(
                    async move { api_client.list_structures().await },
                    callback
                );
                true
            }
            Msg::StructuresLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(structures) => {
                        self.structures = structures;
                        self.filtered_structures = self.structures.clone();
                        // Select first structure if none selected
                        if self.selected_structure.is_none() && !self.structures.is_empty() {
                            self.selected_structure = Some(self.structures[0].clone());
                        }
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to load structures: {}", e));
                        // Fallback to placeholder data
                        self.structures = self.create_placeholder_structures();
                        self.filtered_structures = self.structures.clone();
                        if self.selected_structure.is_none() {
                            self.selected_structure = Some(self.structures[0].clone());
                        }
                    }
                }
                true
            }
            Msg::SearchQueryChanged(query) => {
                self.search_query = query;
                self.filter_structures();
                true
            }
            Msg::ToggleStructureBrowser => {
                self.show_structure_browser = !self.show_structure_browser;
                true
            }
            Msg::SearchStructures => {
                if self.search_query.trim().is_empty() {
                    self.filtered_structures = self.structures.clone();
                    return true;
                }
                
                self.loading = true;
                let api_client = self.api_client.clone();
                let query = self.search_query.clone();
                let callback = ctx.link().callback(Msg::SearchResultsLoaded);
                
                spawn_api_call(
                    async move { api_client.search_structures(&query).await },
                    callback
                );
                true
            }
            Msg::SearchResultsLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(structures) => {
                        self.filtered_structures = structures;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Search failed: {}", e));
                        self.filter_structures(); // Fallback to local filtering
                    }
                }
                true
            }
            Msg::ApiError(error) => {
                self.error = Some(error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_system_selected = ctx.link().callback(Msg::SystemSelected);
        
        // Determine the structure type and system number from selected structure
        let (structure_type, system_num) = if let Some(ref structure) = self.selected_structure {
            let num = self.structure_type_to_number(&structure.structure_type);
            (structure.structure_type.clone(), num)
        } else {
            ("monad".to_string(), 1)
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
                        <GeometricRenderer system_type={structure_type} size={500.0} />
                        {self.render_structure_overlay()}
                    </div>
                </div>
                {self.render_structure_info()}
                {self.render_structure_browser(ctx)}
            </div>
        }
    }
}

impl App {
    fn render_header(&self, ctx: &Context<Self>) -> Html {
        let toggle_browser = ctx.link().callback(|_| Msg::ToggleStructureBrowser);
        
        html! {
            <header class="app-header">
                <div class="header-content">
                    <div class="header-title">
                        <h1>{"SysteMaster"}</h1>
                        <p>{"Systematic Thinking Framework"}</p>
                    </div>
                    <div class="header-controls">
                        <button class="load-button" onclick={toggle_browser}>
                            {if self.show_structure_browser { "Hide Browser" } else { "Load Structure" }}
                        </button>
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
        
        let search_submit = ctx.link().callback(|_| Msg::SearchStructures);
        
        html! {
            <div class="search-controls">
                <div class="search-bar">
                    <input 
                        type="text" 
                        placeholder="Search structures by name, type, or terms..." 
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
                    {"Loading structures..."}
                </div>
            }
        } else if let Some(ref error) = self.error {
            html! {
                <div class="error">
                    <p>{"⚠️ "}{error}</p>
                    <p><small>{"Using fallback data"}</small></p>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn render_structure_overlay(&self) -> Html {
        if let Some(ref structure) = self.selected_structure {
            let system_num = self.structure_type_to_number(&structure.structure_type);
            html! {
                <SystemOverlay system_num={system_num} structure={structure.clone()} />
            }
        } else {
            html! {
                <SystemOverlay system_num={1} />
            }
        }
    }

    fn render_structure_info(&self) -> Html {
        if let Some(ref structure) = self.selected_structure {
            html! {
                <div class="structure-info">
                    <h3>{&structure.name}</h3>
                    <p><strong>{"Type: "}</strong>{&structure.structure_type}</p>
                    {if let Some(ref desc) = structure.description {
                        html! { <p><strong>{"Description: "}</strong>{desc}</p> }
                    } else {
                        html! {}
                    }}
                    <p><strong>{"Terms: "}</strong>{structure.terms.join(", ")}</p>
                </div>
            }
        } else {
            html! {}
        }
    }
    
    fn render_structure_browser(&self, ctx: &Context<Self>) -> Html {
        if !self.show_structure_browser {
            return html! {};
        }
        
        let structures_to_show = if self.search_query.trim().is_empty() {
            &self.structures
        } else {
            &self.filtered_structures
        };
        
        html! {
            <div class="structure-browser-overlay">
                <div class="structure-browser">
                    <div class="browser-header">
                        <h3>{"Available Structures"}</h3>
                        <button 
                            class="close-browser" 
                            onclick={ctx.link().callback(|_| Msg::ToggleStructureBrowser)}
                        >
                            {"×"}
                        </button>
                    </div>
                    <div class="structure-list">
                        {for structures_to_show.iter().map(|structure| {
                            let structure_clone = structure.clone();
                            let select_structure = ctx.link().callback(move |_| {
                                Msg::StructureSelected(structure_clone.clone())
                            });
                            
                            html! {
                                <div class="structure-item" onclick={select_structure}>
                                    <div class="structure-item-header">
                                        <h4>{&structure.name}</h4>
                                        <span class="structure-type">{&structure.structure_type}</span>
                                    </div>
                                    <div class="structure-item-terms">
                                        {structure.terms.join(", ")}
                                    </div>
                                    {if let Some(ref desc) = structure.description {
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
                    {if structures_to_show.is_empty() {
                        html! {
                            <div class="no-results">
                                {"No structures found"}
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        }
    }
    
    fn filter_structures(&mut self) {
        if self.search_query.trim().is_empty() {
            self.filtered_structures = self.structures.clone();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_structures = self.structures
                .iter()
                .filter(|structure| {
                    structure.name.to_lowercase().contains(&query) ||
                    structure.structure_type.to_lowercase().contains(&query) ||
                    structure.terms.iter().any(|term| term.to_lowercase().contains(&query)) ||
                    structure.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query))
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

    fn create_placeholder_structure(&self, structure_type: &str, system_num: i32) -> StoredStructure {
        let terms = match system_num {
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

        StoredStructure {
            id: serde_json::Value::String(format!("placeholder-{}", structure_type)),
            name: format!("Default {}", structure_type.to_uppercase()),
            structure_type: structure_type.to_string(),
            terms,
            connectives: HashMap::new(),
            created_at: "placeholder".to_string(),
            updated_at: "placeholder".to_string(),
            description: Some(format!("Default {} structure", structure_type)),
            metadata: HashMap::new(),
        }
    }

    fn create_placeholder_structures(&self) -> Vec<StoredStructure> {
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
            self.create_placeholder_structure(structure_type, i)
        }).collect()
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
} 