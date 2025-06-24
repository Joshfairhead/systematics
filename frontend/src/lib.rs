use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod state;
mod services;
mod components;
mod core;
mod workflows;

use workflows::creation_workflow::CreationWorkflow;

use state::app_state::*;
use services::api::*;
use components::system_overlay::SystemOverlay;
use components::geometric_renderer::GeometricRenderer;
use components::system_selector::SystemSelector;
use components::creation_dialogue::CreationDialogue;

/// Content Sources - The three layers of the Language Tetrad
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentSource {
    CoreGrammar,        // Bennett's canonical terms
    CommunityGrammar,   // Community mappings  
    UserExpressions,    // User applications
}

/// Content Items - unified representation
#[derive(Debug, Clone, PartialEq)]
pub enum ContentItem {
    CoreGrammar(CoreGrammar),
    CommunityGrammar(CommunityGrammar), 
    UserExpression(UserExpression),
}

/// Add Content Types for creation workflow (Phase 2)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddContentType {
    Definition,
    Collection,
    System,
    Paper,
    SystemCollection,
    Module,
    Book,
}

/// Structure Types for systematic structures
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StructureType {
    Monad,
    Dyad,
    Triad,
    Tetrad,
    Pentad,
    Hexad,
    Heptad,
    Octad,
    Ennead,
    Decad,
    Undecad,
    Dodecad,
}

impl std::fmt::Display for StructureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructureType::Monad => write!(f, "Monad"),
            StructureType::Dyad => write!(f, "Dyad"),
            StructureType::Triad => write!(f, "Triad"),
            StructureType::Tetrad => write!(f, "Tetrad"),
            StructureType::Pentad => write!(f, "Pentad"),
            StructureType::Hexad => write!(f, "Hexad"),
            StructureType::Heptad => write!(f, "Heptad"),
            StructureType::Octad => write!(f, "Octad"),
            StructureType::Ennead => write!(f, "Ennead"),
            StructureType::Decad => write!(f, "Decad"),
            StructureType::Undecad => write!(f, "Undecad"),
            StructureType::Dodecad => write!(f, "Dodecad"),
        }
    }
}

impl StructureType {
    pub fn term_count(&self) -> usize {
        match self {
            StructureType::Monad => 1,
            StructureType::Dyad => 2,
            StructureType::Triad => 3,
            StructureType::Tetrad => 4,
            StructureType::Pentad => 5,
            StructureType::Hexad => 6,
            StructureType::Heptad => 7,
            StructureType::Octad => 8,
            StructureType::Ennead => 9,
            StructureType::Decad => 10,
            StructureType::Undecad => 11,
            StructureType::Dodecad => 12,
        }
    }
}

impl ContentItem {
    pub fn name(&self) -> &str {
        match self {
            ContentItem::CoreGrammar(g) => &g.name,
            ContentItem::CommunityGrammar(g) => &g.name,
            ContentItem::UserExpression(i) => &i.name,
        }
    }
    
    pub fn definition_type(&self) -> &str {
        match self {
            ContentItem::CoreGrammar(g) => &g.definition_type,
            ContentItem::CommunityGrammar(g) => &g.definition_type,
            ContentItem::UserExpression(i) => &i.definition_type,
        }
    }
}

/// Application Messages - Clean State Machine Events
#[derive(Debug, Clone)]
pub enum Msg {
    // === CORE STATE TRANSITIONS ===
    ContentSourceChanged(ContentSource),
    ItemSelected(ContentItem),
    ClearSelection,
    
    // === SYSTEM SELECTION ===
    SystemSelected(i32),
    
    // === DATA LOADING ===
    LoadData,
    LoadDataSilent, // For background loading without notifications
    DataLoaded(Result<Vec<ContentItem>, String>),
    DataLoadedSilent(Result<Vec<ContentItem>, String>), // For silent loading
    
    // === SEARCH ===
    SearchChanged(String),
    ToggleBrowser,
    
    // === NOTIFICATIONS ===
    ClearNotifications,
    
    // === CREATION WORKFLOW ===
    CreateNewItem, // Start creation workflow - now opens dialogue
    CreateWithDialogue((AddContentType, String)), // Create with selected type and name from dialogue
    CancelCreationDialogue, // Cancel the creation dialogue
    UpdateCreationField((usize, String)), // Update user input field during creation
    ConfirmCreation, // Confirm and save the creation
    CancelCreation, // Cancel the creation and return to browsing
}

/// Minimal State Machine Application
pub struct App {
    state: AppState,
    api_client: services::api::ApiClient,
    show_creation_dialogue: bool,
    creation_user_inputs: Vec<String>, // Track user inputs during creation
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let api_client = services::api::ApiClient::new();
        let mut state = AppState::default();
        
        // Start with Core Grammar - the fundamental systematic structures
        state.mode = AppMode::Browsing { 
            source: ContentSource::CoreGrammar,
            selected_item: None,
            selected_system: 1, // Start with Monad
        };
        
        // Trigger initial data load
        ctx.link().send_message(Msg::LoadDataSilent);
        
        Self { 
            state, 
            api_client,
            show_creation_dialogue: false,
            creation_user_inputs: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        web_sys::console::log_1(&format!("Update: {:?}", msg).into());
        
        match msg {
            // === CORE STATE TRANSITIONS ===
            Msg::ContentSourceChanged(source) => {
                web_sys::console::log_1(&format!("Changing content source to: {:?}", source).into());
                self.state.browse_content_source(source);
                self.update_filtered_content();
                
                // Load data for the new source
                ctx.link().send_message(Msg::LoadData);
                true
            }
            
            Msg::ItemSelected(item) => {
                web_sys::console::log_1(&format!("Item selected: {:?}", item.name()).into());
                
                // Synchronize system selection with the selected item
                let system_num = match item.definition_type() {
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
                    _ => 1, // Default to monad
                };
                
                // Update both selected item and system selection
                self.state.select_item(Some(item));
                self.state.select_system(system_num);
                true
            }
            
            Msg::ClearSelection => {
                self.state.select_item(None);
                true
            }
            
            // === SYSTEM SELECTION ===
            Msg::SystemSelected(system) => {
                web_sys::console::log_1(&format!("System selected: {}", system).into());
                self.state.select_system(system);
                
                // Auto-select corresponding item from available content if in Core Grammar
                if self.state.current_content_source() == ContentSource::CoreGrammar {
                    let system_type = match system {
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
                        _ => "monad", // Default
                    };
                    
                    // Find matching item in current content
                    if let Some(matching_item) = self.state.filtered_content.iter()
                        .find(|item| item.definition_type() == system_type) {
                        self.state.select_item(Some(matching_item.clone()));
                    } else {
                        // If no matching item found, clear selection
                        self.state.select_item(None);
                    }
                }
                
                true
            }
            
            // === DATA LOADING ===
            Msg::LoadData => {
                self.state.enter_loading();
                
                let api_client = self.api_client.clone();
                let source = self.state.current_content_source();
                
                ctx.link().send_future(async move {
                    let result = match source {
                        ContentSource::CoreGrammar => {
                            // Load all systematic structures from API
                            let structure_types = vec![
                                "monad", "dyad", "triad", "tetrad", "pentad", "hexad",
                                "heptad", "octad", "ennead", "decad", "undecad", "dodecad"
                            ];
                            
                            let mut grammars = Vec::new();
                            for structure_type in structure_types {
                                match api_client.get_core_grammar(structure_type).await {
                                    Ok(grammar) => grammars.push(ContentItem::CoreGrammar(grammar)),
                                    Err(e) => {
                                        web_sys::console::log_1(&format!("Failed to load {}: {}", structure_type, e).into());
                                        continue;
                                    }
                                }
                            }
                            
                            if grammars.is_empty() {
                                Err("Failed to load any core grammars from API".to_string())
                            } else {
                                Ok(grammars)
                            }
                        }
                        ContentSource::CommunityGrammar => {
                            match api_client.list_community_grammars(None).await {
                                Ok(grammars) => {
                                    let items: Vec<ContentItem> = grammars.into_iter()
                                        .map(ContentItem::CommunityGrammar)
                                        .collect();
                                    Ok(items)
                                }
                                Err(e) => Err(format!("Failed to load community grammars: {}", e))
                            }
                        }
                        ContentSource::UserExpressions => {
                            match api_client.list_user_expressions().await {
                                Ok(expressions) => {
                                    let items: Vec<ContentItem> = expressions.into_iter()
                                        .map(ContentItem::UserExpression)
                                        .collect();
                                    Ok(items)
                                }
                                Err(e) => Err(format!("Failed to load user expressions: {}", e))
                            }
                        }
                    };
                    
                    Msg::DataLoaded(result)
                });
                
                true
            }
            
            Msg::LoadDataSilent => {
                self.state.enter_loading();
                
                let api_client = self.api_client.clone();
                let source = self.state.current_content_source();
                
                ctx.link().send_future(async move {
                    let result = match source {
                        ContentSource::CoreGrammar => {
                            // Load all systematic structures from API
                            let structure_types = vec![
                                "monad", "dyad", "triad", "tetrad", "pentad", "hexad",
                                "heptad", "octad", "ennead", "decad", "undecad", "dodecad"
                            ];
                            
                            let mut grammars = Vec::new();
                            for structure_type in structure_types {
                                match api_client.get_core_grammar(structure_type).await {
                                    Ok(grammar) => grammars.push(ContentItem::CoreGrammar(grammar)),
                                    Err(e) => {
                                        web_sys::console::log_1(&format!("Failed to load {}: {}", structure_type, e).into());
                                        continue;
                                    }
                                }
                            }
                            
                            if grammars.is_empty() {
                                Err("Failed to load any core grammars from API".to_string())
                            } else {
                                Ok(grammars)
                            }
                        }
                        ContentSource::CommunityGrammar => {
                            match api_client.list_community_grammars(None).await {
                                Ok(grammars) => {
                                    let items: Vec<ContentItem> = grammars.into_iter()
                                        .map(ContentItem::CommunityGrammar)
                                        .collect();
                                    Ok(items)
                                }
                                Err(e) => Err(format!("Failed to load community grammars: {}", e))
                            }
                        }
                        ContentSource::UserExpressions => {
                            match api_client.list_user_expressions().await {
                                Ok(expressions) => {
                                    let items: Vec<ContentItem> = expressions.into_iter()
                                        .map(ContentItem::UserExpression)
                                        .collect();
                                    Ok(items)
                                }
                                Err(e) => Err(format!("Failed to load user expressions: {}", e))
                            }
                        }
                    };
                    
                    Msg::DataLoadedSilent(result)
                });
                
                true
            }
            
            Msg::DataLoaded(result) => {
                let source = self.state.current_content_source();
                self.state.exit_loading(source);
                
                match result {
                    Ok(items) => {
                        match source {
                            ContentSource::CoreGrammar => {
                                let grammars: Vec<CoreGrammar> = items.into_iter()
                                    .filter_map(|item| match item {
                                        ContentItem::CoreGrammar(g) => Some(g),
                                        _ => None,
                                    })
                                    .collect();
                                self.state.core_grammars = grammars;
                            }
                            ContentSource::CommunityGrammar => {
                                let grammars: Vec<CommunityGrammar> = items.into_iter()
                                    .filter_map(|item| match item {
                                        ContentItem::CommunityGrammar(g) => Some(g),
                                        _ => None,
                                    })
                                    .collect();
                                self.state.community_grammars = grammars;
                            }
                            ContentSource::UserExpressions => {
                                let expressions: Vec<UserExpression> = items.into_iter()
                                    .filter_map(|item| match item {
                                        ContentItem::UserExpression(e) => Some(e),
                                        _ => None,
                                    })
                                    .collect();
                                self.state.user_expressions = expressions;
                            }
                        }
                        
                        self.update_filtered_content();
                        
                        let count = self.state.filtered_content.len();
                        let source_name = match source {
                            ContentSource::CoreGrammar => "Core Grammar",
                            ContentSource::CommunityGrammar => "Community Grammar",
                            ContentSource::UserExpressions => "User Expressions",
                        };
                        
                        self.state.set_success(format!("Loaded {} {} items", count, source_name));
                        
                        // Auto-dismiss success notification after 1.5 seconds
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(1500).await;
                            link.send_message(Msg::ClearNotifications);
                        });
                    }
                    Err(e) => {
                        self.state.set_error(e);
                        
                        // Auto-dismiss error notification after 1.5 seconds
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(1500).await;
                            link.send_message(Msg::ClearNotifications);
                        });
                    }
                }
                
                true
            }
            
            Msg::DataLoadedSilent(result) => {
                let source = self.state.current_content_source();
                self.state.exit_loading(source);
                
                match result {
                    Ok(items) => {
                        match source {
                            ContentSource::CoreGrammar => {
                                let grammars: Vec<CoreGrammar> = items.into_iter()
                                    .filter_map(|item| match item {
                                        ContentItem::CoreGrammar(g) => Some(g),
                                        _ => None,
                                    })
                                    .collect();
                                self.state.core_grammars = grammars;
                            }
                            ContentSource::CommunityGrammar => {
                                let grammars: Vec<CommunityGrammar> = items.into_iter()
                                    .filter_map(|item| match item {
                                        ContentItem::CommunityGrammar(g) => Some(g),
                                        _ => None,
                                    })
                                    .collect();
                                self.state.community_grammars = grammars;
                            }
                            ContentSource::UserExpressions => {
                                let expressions: Vec<UserExpression> = items.into_iter()
                                    .filter_map(|item| match item {
                                        ContentItem::UserExpression(e) => Some(e),
                                        _ => None,
                                    })
                                    .collect();
                                self.state.user_expressions = expressions;
                            }
                        }
                        
                        self.update_filtered_content();
                        
                        // Try to restore system selection if matching item becomes available
                        if source == ContentSource::CoreGrammar {
                            let current_system = self.state.selected_system();
                            let system_type = match current_system {
                                1 => "monad", 2 => "dyad", 3 => "triad", 4 => "tetrad",
                                5 => "pentad", 6 => "hexad", 7 => "heptad", 8 => "octad",
                                9 => "ennead", 10 => "decad", 11 => "undecad", 12 => "dodecad",
                                _ => "monad",
                            };
                            
                            // If we have a system selected but no item, try to find matching item
                            if self.state.selected_item().is_none() {
                                if let Some(matching_item) = self.state.filtered_content.iter()
                                    .find(|item| item.definition_type() == system_type) {
                                    self.state.select_item(Some(matching_item.clone()));
                                }
                            }
                        }
                        
                        // Silent success - no notification for background loading
                        web_sys::console::log_1(&format!("Successfully loaded {} items for {:?}", 
                            self.state.filtered_content.len(), source).into());
                    }
                    Err(e) => {
                        self.state.set_error(e);
                        
                        // Auto-dismiss error notification after 1.5 seconds
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(1500).await;
                            link.send_message(Msg::ClearNotifications);
                        });
                    }
                }
                
                true
            }
            
            // === SEARCH ===
            Msg::SearchChanged(query) => {
                self.state.set_search_query(query);
                self.update_filtered_content();
                true
            }
            
            Msg::ToggleBrowser => {
                self.state.toggle_content_browser();
                true
            }
            
            // === NOTIFICATIONS ===
            Msg::ClearNotifications => {
                self.state.clear_notifications();
                true
            }
            
            // === CREATION WORKFLOW ===
            Msg::CreateNewItem => {
                let current_source = self.state.current_content_source();
                
                // Systematic validation: Check if creation is allowed in this source
                if !CreationWorkflow::can_create_in_source(current_source) {
                    self.state.set_error("⚠️ Core Grammar is read-only. Switch to Community Grammar or User Expressions to create content.".to_string());
                    return true;
                }
                
                // Open the creation dialogue instead of using browser prompt
                self.show_creation_dialogue = true;
                true
            }
            
            Msg::CreateWithDialogue((add_type, name)) => {
                let current_source = self.state.current_content_source();
                
                // Validate creation readiness
                match CreationWorkflow::validate_creation_readiness(
                    current_source,
                    Some(add_type),
                    Some(&name)
                ) {
                    Ok(()) => {
                        // Enter creation mode with valid parameters
                        self.state.start_creation(add_type);
                        self.state.update_creation_name(name);
                        self.show_creation_dialogue = false;
                        
                        // Initialize user inputs based on selected system
                        let term_count = match self.state.selected_system() {
                            1 => 1, 2 => 2, 3 => 3, 4 => 4, 5 => 5, 6 => 6,
                            7 => 7, 8 => 8, 9 => 9, 10 => 10, 11 => 11, 12 => 12,
                            _ => 1,
                        };
                        self.creation_user_inputs = vec![String::new(); term_count];
                        
                        self.state.set_success("✅ Creation started! Fill in your systematic structure below.".to_string());
                        
                        // Auto-dismiss success after 2 seconds
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            link.send_message(Msg::ClearNotifications);
                        });
                    },
                    Err(error) => {
                        self.state.set_error(format!("❌ Creation failed: {}", error));
                        self.show_creation_dialogue = false;
                    }
                }
                
                true
            }
            
            Msg::CancelCreationDialogue => {
                self.show_creation_dialogue = false;
                true
            }
            
            Msg::UpdateCreationField((index, value)) => {
                // Update the user input at the specified index
                if index < self.creation_user_inputs.len() {
                    self.creation_user_inputs[index] = value;
                    true
                } else {
                    false
                }
            }
            
            Msg::ConfirmCreation => {
                // Implement actual creation logic here
                if let Some((add_type, name)) = self.state.creation_details() {
                    // TODO: Implement actual API call to create the content
                    self.state.set_success(format!("✅ Successfully created {} '{}'!", 
                        match add_type {
                            AddContentType::System => "System",
                            AddContentType::Paper => "Paper",
                            AddContentType::SystemCollection => "System Collection",
                            AddContentType::Module => "Module",
                            AddContentType::Book => "Book",
                            AddContentType::Definition => "Definition",
                            AddContentType::Collection => "Collection",
                        }, 
                        name
                    ));
                    self.state.complete_creation();
                    self.creation_user_inputs.clear();
                    
                    // Auto-dismiss after 3 seconds
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        gloo_timers::future::TimeoutFuture::new(3000).await;
                        link.send_message(Msg::ClearNotifications);
                    });
                }
                true
            }
            
            Msg::CancelCreation => {
                self.state.cancel_creation();
                self.creation_user_inputs.clear();
                self.state.set_success("ℹ️ Creation cancelled.".to_string());
                
                // Auto-dismiss after 1 second
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(1000).await;
                    link.send_message(Msg::ClearNotifications);
                });
                
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app-container">
                {self.render_header(ctx)}
                {self.render_search_controls(ctx)}
                {self.render_system_selector(ctx)}
                {self.render_main_content(ctx)}
                {self.render_notifications()}
                
                // Creation Dialogue
                <CreationDialogue 
                    show={self.show_creation_dialogue}
                    content_source={self.state.current_content_source()}
                    on_create={ctx.link().callback(Msg::CreateWithDialogue)}
                    on_cancel={ctx.link().callback(|_| Msg::CancelCreationDialogue)}
                />
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
                        <p>{"Systematic Knowledge Organization"}</p>
                    </div>
                    <div class="environment-controls">
                        <div class="environment-switch">
                            <button class="env-button active">{"Development"}</button>
                        </div>
                    </div>
                </div>
            </header>
        }
    }
    
    fn render_search_controls(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="search-controls">
                <div class="search-bar">
                    <div class="content-source-tabs">
                        {self.render_content_source_tab(ctx, ContentSource::CoreGrammar, "Core Grammar")}
                        {self.render_content_source_tab(ctx, ContentSource::CommunityGrammar, "Community Grammar")}
                        {self.render_content_source_tab(ctx, ContentSource::UserExpressions, "User Expressions")}
                    </div>
                    <div class="action-buttons">
                        <input 
                            type="text"
                            class="search-input"
                            placeholder="Search content..."
                            value={self.state.ui.search_query.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                Msg::SearchChanged(input.value())
                            })}
                        />
                        <button 
                            onclick={ctx.link().callback(|_| Msg::ToggleBrowser)}
                            class="search-button"
                        >
                            {"Browse"}
                        </button>
                        <button 
                            onclick={ctx.link().callback(|_| Msg::CreateNewItem)}
                            class={if self.state.current_content_source() == ContentSource::CoreGrammar { 
                                "load-button disabled" 
                            } else { 
                                "load-button" 
                            }}
                            disabled={self.state.current_content_source() == ContentSource::CoreGrammar}
                            title={if self.state.current_content_source() == ContentSource::CoreGrammar {
                                "Core Grammar is read-only - switch to Community Grammar or User Expressions to create content"
                            } else {
                                "Create new systematic structure"
                            }}
                        >
                            {"+ Add"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
    
    fn render_system_selector(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="system-selector-container">
                <SystemSelector 
                    selected_system={self.state.selected_system()}
                    on_system_selected={ctx.link().callback(Msg::SystemSelected)}
                    disabled={self.state.is_loading()}
                />
            </div>
        }
    }
    
    fn render_content_source_tab(&self, ctx: &Context<Self>, source: ContentSource, label: &str) -> Html {
        let current_source = self.state.current_content_source();
        let is_active = current_source == source;
        let class = if is_active { "system-tab-button active" } else { "system-tab-button" };
        
        html! {
            <button 
                class={class}
                onclick={ctx.link().callback(move |_| Msg::ContentSourceChanged(source))}
            >
                {label}
            </button>
        }
    }
    
    fn render_main_content(&self, ctx: &Context<Self>) -> Html {
        if self.state.is_loading() {
            return html! {
                <div class="loading">
                    <p>{"Loading..."}</p>
                </div>
            };
        }
        
        html! {
            <main class="main-content">
                {self.render_system_overlay(ctx)}
                {self.render_structure_info(ctx)}
                {self.render_content_browser(ctx)}
            </main>
        }
    }
    
    fn render_system_overlay(&self, ctx: &Context<Self>) -> Html {
        let system_num = self.state.selected_system();
        let current_source = self.state.current_content_source();
        
        // Always show SystemOverlay - get the appropriate data for the current system
        let system_data = if current_source == ContentSource::CoreGrammar {
            // For core grammar, find the matching system
            self.state.core_grammars.iter()
                .find(|g| {
                    let system_type = match system_num {
                        1 => "monad", 2 => "dyad", 3 => "triad", 4 => "tetrad",
                        5 => "pentad", 6 => "hexad", 7 => "heptad", 8 => "octad",
                        9 => "ennead", 10 => "decad", 11 => "undecad", 12 => "dodecad",
                        _ => "monad",
                    };
                    g.definition_type == system_type
                })
                .map(|g| ContentItem::CoreGrammar(g.clone()))
        } else {
            // For other sources, use selected item if available
            self.state.selected_item().cloned()
        };
        
        let system_name = match system_num {
            1 => "Monad", 2 => "Dyad", 3 => "Triad", 4 => "Tetrad",
            5 => "Pentad", 6 => "Hexad", 7 => "Heptad", 8 => "Octad",
            9 => "Ennead", 10 => "Decad", 11 => "Undecad", 12 => "Dodecad",
            _ => "Unknown",
        };
        
        html! {
            <SystemOverlay 
                system_num={system_num}
                definition={system_data.clone()}
                creation_mode={self.state.is_creating()}
                structure_name={Some(system_name.to_string())}
                user_expressions={
                    if self.state.is_creating() {
                        // Use creation inputs when in creation mode
                        self.creation_user_inputs.clone()
                    } else if let Some(ref item) = system_data {
                        self.get_user_expressions_for_item(item)
                    } else {
                        vec![] // Empty for now, will be filled when data loads
                    }
                }
                on_instance_change={
                    if self.state.is_creating() {
                        ctx.link().callback(Msg::UpdateCreationField)
                    } else {
                        ctx.link().callback(|_| Msg::ClearSelection)
                    }
                }
                content_source={current_source}
                selected_add_type={
                    if let Some((add_type, _)) = self.state.creation_details() {
                        Some(add_type)
                    } else {
                        None
                    }
                }
            />
        }
    }
    
    fn render_structure_info(&self, ctx: &Context<Self>) -> Html {
        let system_num = self.state.selected_system();
        let current_source = self.state.current_content_source();
        
        // Get the same system data as in render_system_overlay
        let system_data = if current_source == ContentSource::CoreGrammar {
            self.state.core_grammars.iter()
                .find(|g| {
                    let system_type = match system_num {
                        1 => "monad", 2 => "dyad", 3 => "triad", 4 => "tetrad",
                        5 => "pentad", 6 => "hexad", 7 => "heptad", 8 => "octad",
                        9 => "ennead", 10 => "decad", 11 => "undecad", 12 => "dodecad",
                        _ => "monad",
                    };
                    g.definition_type == system_type
                })
                .map(|g| ContentItem::CoreGrammar(g.clone()))
        } else {
            self.state.selected_item().cloned()
        };
        
        let system_name = match system_num {
            1 => "Monad", 2 => "Dyad", 3 => "Triad", 4 => "Tetrad",
            5 => "Pentad", 6 => "Hexad", 7 => "Heptad", 8 => "Octad",
            9 => "Ennead", 10 => "Decad", 11 => "Undecad", 12 => "Dodecad",
            _ => "Unknown",
        };
        
        html! {
            <div class="structure-display">
                // Show contextual information
                <div class="structure-info">
                    {if self.state.is_creating() {
                        // Show creation controls
                        if let Some((add_type, name)) = self.state.creation_details() {
                            html! {
                                <div class="creation-controls">
                                    <h3>{"Creating New "}{match add_type {
                                        AddContentType::System => "System",
                                        AddContentType::SystemCollection => "System Collection",
                                        AddContentType::Paper => "Paper",
                                        AddContentType::Module => "Module",
                                        AddContentType::Book => "Book",
                                        AddContentType::Definition => "Definition",
                                        AddContentType::Collection => "Collection",
                                    }}</h3>
                                    <p><strong>{"Name: "}</strong>{name}</p>
                                    <p><strong>{"Structure Type: "}</strong>{system_name}</p>
                                    <p><strong>{"Content Source: "}</strong>{match current_source {
                                        ContentSource::CommunityGrammar => "Community Grammar",
                                        ContentSource::UserExpressions => "User Expressions",
                                        ContentSource::CoreGrammar => "Core Grammar",
                                    }}</p>
                                    <div class="creation-actions">
                                        <button 
                                            class="confirm-button"
                                            onclick={ctx.link().callback(|_| Msg::ConfirmCreation)}
                                        >
                                            {"✅ Confirm Creation"}
                                        </button>
                                        <button 
                                            class="cancel-button"
                                            onclick={ctx.link().callback(|_| Msg::CancelCreation)}
                                        >
                                            {"❌ Cancel"}
                                        </button>
                                    </div>
                                    <p class="creation-instructions">
                                        {"Fill in the systematic structure above, then confirm to save."}
                                    </p>
                                </div>
                            }
                        } else {
                            html! { <div>{"Creating..."}</div> }
                        }
                    } else if let Some(ref item) = system_data {
                        // Show detailed item information
                        html! {
                            <>
                                <h3>{item.name()}{" System"}</h3>
                                <p><strong>{"Type: "}</strong>{item.definition_type()}</p>
                                {self.render_item_details(item)}
                                {if current_source != ContentSource::CoreGrammar {
                                    html! {
                                        <button 
                                            onclick={ctx.link().callback(|_| Msg::ClearSelection)}
                                            class="search-button"
                                        >
                                            {"Clear Selection"}
                                        </button>
                                    }
                                } else {
                                    html! {}
                                }}
                            </>
                        }
                    } else {
                        // Show system information and guidance
                        html! {
                            <>
                                <h3>{format!("{} System", system_name)}</h3>
                                <p><strong>{"Current Source: "}</strong>
                                    {match current_source {
                                        ContentSource::CoreGrammar => "Core Grammar - Bennett's systematic structures",
                                        ContentSource::CommunityGrammar => "Community Grammar - Community mappings", 
                                        ContentSource::UserExpressions => "User Expressions - Your applications",
                                    }}
                                </p>
                                {if self.state.is_loading() {
                                    html! {
                                        <p><em>{"Loading data..."}</em></p>
                                    }
                                } else if self.state.filtered_content.is_empty() {
                                    html! {
                                        <>
                                            <p><em>{"No content available in this source for the selected system."}</em></p>
                                            <p>{"💡 Try switching to Core Grammar to see all systematic structures, or use Browse to explore available content."}</p>
                                        </>
                                    }
                                } else {
                                    html! {
                                        <p>{"Use the Browse button to select specific content, or choose a different system using the tabs above."}</p>
                                    }
                                }}
                            </>
                        }
                    }}
                </div>
            </div>
        }
    }
    
    fn render_content_browser(&self, ctx: &Context<Self>) -> Html {
        if !self.state.ui.show_content_browser {
            return html! {};
        }
        
        html! {
            <div class="content-browser-overlay">
                <div class="content-browser">
                    <div class="browser-header">
                        <div class="browser-title-section">
                            <h3>{"Available Content"}</h3>
                            <div class="content-source-badge">
                                {match self.state.current_content_source() {
                                    ContentSource::CoreGrammar => "Core Grammar",
                                    ContentSource::CommunityGrammar => "Community Grammar", 
                                    ContentSource::UserExpressions => "User Expressions",
                                }}
                            </div>
                        </div>
                        <button 
                            onclick={ctx.link().callback(|_| Msg::ToggleBrowser)}
                            class="close-browser"
                        >
                            {"×"}
                        </button>
                    </div>
                    
                    <div class="content-list">
                        {if self.state.filtered_content.is_empty() {
                            html! {
                                <div class="no-results">
                                    <h4>{"No content available"}</h4>
                                    <p>{"Try switching to a different content source or adjusting your search."}</p>
                                </div>
                            }
                        } else {
                            html! {
                                <>
                                    {for self.state.filtered_content.iter().map(|item| {
                                        let item_clone = item.clone();
                                        html! {
                                            <div 
                                                class="content-item"
                                                onclick={ctx.link().callback(move |_| Msg::ItemSelected(item_clone.clone()))}
                                            >
                                                <div class="content-item-header">
                                                    <h4>{item.name()}</h4>
                                                    <div class="content-type-badge">
                                                        {item.definition_type()}
                                                    </div>
                                                </div>
                                                <div class="content-item-description">
                                                    {"Click to select this "}{item.definition_type().to_lowercase()}
                                                </div>
                                            </div>
                                        }
                                    })}
                                </>
                            }
                        }}
                    </div>
                </div>
            </div>
        }
    }
    
    fn render_notifications(&self) -> Html {
        html! {
            <>
                {if let Some(ref success) = self.state.ui.success_message {
                    html! {
                        <div class="notification success">
                            <p>{success}</p>
                            <small>{"Success"}</small>
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                {if let Some(ref error) = self.state.ui.error {
                    html! {
                        <div class="notification error">
                            <p>{error}</p>
                            <small>{"Error"}</small>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </>
        }
    }
    
    fn update_filtered_content(&mut self) {
        let source = self.state.current_content_source();
        let query = &self.state.ui.search_query;
        
        let all_items: Vec<ContentItem> = match source {
            ContentSource::CoreGrammar => {
                self.state.core_grammars.iter()
                    .map(|g| ContentItem::CoreGrammar(g.clone()))
                    .collect()
            }
            ContentSource::CommunityGrammar => {
                self.state.community_grammars.iter()
                    .map(|g| ContentItem::CommunityGrammar(g.clone()))
                    .collect()
            }
            ContentSource::UserExpressions => {
                self.state.user_expressions.iter()
                    .map(|e| ContentItem::UserExpression(e.clone()))
                    .collect()
            }
        };
        
        // Apply search filter
        if query.is_empty() {
            self.state.filtered_content = all_items;
        } else {
            self.state.filtered_content = all_items.into_iter()
                .filter(|item| {
                    item.name().to_lowercase().contains(&query.to_lowercase()) ||
                    item.definition_type().to_lowercase().contains(&query.to_lowercase())
                })
                .collect();
        }
    }
    
    fn get_user_expressions_for_item(&self, item: &ContentItem) -> Vec<String> {
        match item {
            ContentItem::CoreGrammar(grammar) => grammar.term_characters.clone(),
            ContentItem::CommunityGrammar(grammar) => grammar.term_characters.clone(),
            ContentItem::UserExpression(expression) => expression.user_expressions.clone(),
        }
    }
    
    fn render_item_details(&self, item: &ContentItem) -> Html {
        match item {
            ContentItem::CoreGrammar(grammar) => {
                html! {
                    <>
                        <p><strong>{"Author: "}</strong>{&grammar.source}</p>
                        <p><strong>{"Description: "}</strong>{&grammar.coherence_attribute}</p>
                        <div class="term-characters">
                            <strong>{"Term Characters:"}</strong>
                            <ul>
                                {for grammar.term_characters.iter().enumerate().map(|(i, term)| {
                                    html! {
                                        <li>{format!("{}. {}", i + 1, term)}</li>
                                    }
                                })}
                            </ul>
                        </div>
                    </>
                }
            }
            ContentItem::CommunityGrammar(grammar) => {
                html! {
                    <>
                        <p><strong>{"Author: "}</strong>{&grammar.author}</p>
                        <p><strong>{"Description: "}</strong>
                            {if let Some(ref desc) = grammar.description {
                                desc.clone()
                            } else {
                                "No description available".to_string()
                            }}
                        </p>
                        <div class="term-characters">
                            <strong>{"Term Characters:"}</strong>
                            <ul>
                                {for grammar.term_characters.iter().enumerate().map(|(i, term)| {
                                    html! {
                                        <li>{format!("{}. {}", i + 1, term)}</li>
                                    }
                                })}
                            </ul>
                        </div>
                    </>
                }
            }
            ContentItem::UserExpression(expression) => {
                html! {
                    <div class="user-expressions">
                        <strong>{"User Expressions:"}</strong>
                        <ul>
                            {for expression.user_expressions.iter().enumerate().map(|(i, expr)| {
                                html! {
                                    <li>{format!("{}. {}", i + 1, expr)}</li>
                                }
                            })}
                        </ul>
                    </div>
                }
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
} 