use yew::prelude::*;
use yew::classes;
use wasm_bindgen::prelude::*;

use web_sys::{HtmlInputElement, InputEvent, window};
use crate::components::system_overlay::SystemOverlay;
use crate::components::system_selector::SystemSelector;
use gloo_timers;

mod components; // Declare the components module
mod services;   // Declare the services module
mod core;       // Declare the core module (framework-agnostic)

use components::geometric_renderer::GeometricRenderer;
use services::api::{ApiClient, spawn_api_call, UserInstance, CoreGrammar, CommunityGrammar, SystemDefinition};

/// Enhanced type safety for structure types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureType {
    Monad = 1,
    Dyad = 2,
    Triad = 3,
    Tetrad = 4,
    Pentad = 5,
    Hexad = 6,
    Heptad = 7,
    Octad = 8,
    Ennead = 9,
    Decad = 10,
    Undecad = 11,
    Dodecad = 12,
}

impl StructureType {
    pub fn from_number(num: i32) -> Option<Self> {
        match num {
            1 => Some(Self::Monad),
            2 => Some(Self::Dyad),
            3 => Some(Self::Triad),
            4 => Some(Self::Tetrad),
            5 => Some(Self::Pentad),
            6 => Some(Self::Hexad),
            7 => Some(Self::Heptad),
            8 => Some(Self::Octad),
            9 => Some(Self::Ennead),
            10 => Some(Self::Decad),
            11 => Some(Self::Undecad),
            12 => Some(Self::Dodecad),
            _ => None,
        }
    }

    pub fn to_number(self) -> i32 {
        self as i32
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Self::Monad => "monad",
            Self::Dyad => "dyad",
            Self::Triad => "triad",
            Self::Tetrad => "tetrad",
            Self::Pentad => "pentad",
            Self::Hexad => "hexad",
            Self::Heptad => "heptad",
            Self::Octad => "octad",
            Self::Ennead => "ennead",
            Self::Decad => "decad",
            Self::Undecad => "undecad",
            Self::Dodecad => "dodecad",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "monad" => Some(Self::Monad),
            "dyad" => Some(Self::Dyad),
            "triad" => Some(Self::Triad),
            "tetrad" => Some(Self::Tetrad),
            "pentad" => Some(Self::Pentad),
            "hexad" => Some(Self::Hexad),
            "heptad" => Some(Self::Heptad),
            "octad" => Some(Self::Octad),
            "ennead" => Some(Self::Ennead),
            "decad" => Some(Self::Decad),
            "undecad" => Some(Self::Undecad),
            "dodecad" => Some(Self::Dodecad),
            _ => None,
        }
    }

    pub fn term_count(self) -> usize {
        self.to_number() as usize
    }
}

/// Content source enumeration for Language Tetrad navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentSource {
    CoreGrammar,     // Directive: Bennett's canonical terms
    CommunityGrammar, // Instrumental: User-contributed mappings  
    UserInstances,   // Ground: Concrete personal applications
}

// Unified content item for browser display
#[derive(Debug, Clone, PartialEq)]
pub enum ContentItem {
    CoreGrammar(CoreGrammar),
    CommunityGrammar(CommunityGrammar), 
    UserInstance(UserInstance),
}

impl ContentItem {
    pub fn name(&self) -> &str {
        match self {
            ContentItem::CoreGrammar(g) => &g.name,
            ContentItem::CommunityGrammar(g) => &g.name,
            ContentItem::UserInstance(i) => &i.name,
        }
    }
    
    pub fn structure_type(&self) -> &str {
        match self {
            ContentItem::CoreGrammar(g) => &g.structure_type,
            ContentItem::CommunityGrammar(g) => &g.structure_type,
            ContentItem::UserInstance(i) => &i.structure_type,
        }
    }
    
    pub fn description(&self) -> Option<&str> {
        match self {
            ContentItem::CoreGrammar(_) => None, // Core grammars don't have descriptions in this context
            ContentItem::CommunityGrammar(g) => g.description.as_deref(),
            ContentItem::UserInstance(i) => i.description.as_deref(),
        }
    }
    
    pub fn instances(&self) -> Vec<String> {
        match self {
            ContentItem::CoreGrammar(g) => g.term_characters.clone(),
            ContentItem::CommunityGrammar(g) => g.term_characters.clone(),
            ContentItem::UserInstance(i) => i.instances.clone(),
        }
    }
}

pub struct App {
    // Language Tetrad Data Structure
    user_instances: Vec<UserInstance>,          // Ground: User concrete applications
    core_grammars: Vec<CoreGrammar>,            // Directive: Bennett's canonical terms
    community_grammars: Vec<CommunityGrammar>,  // Instrumental: Community mappings
    filtered_content: Vec<ContentItem>,         // Currently displayed content
    selected_item: Option<ContentItem>,         // Currently selected item
    current_definition: Option<SystemDefinition>, // Source: Mathematical structure
    current_structure_type: StructureType, // Track the currently selected structure type
    loading: bool,
    error: Option<String>,
    success_message: Option<String>,
    api_client: ApiClient,
    search_query: String,
    show_content_browser: bool,
    // Creation state
    creation_mode: bool,
    structure_name: Option<String>,
    user_input: Vec<String>, // User input during creation
    saving: bool,
    // Content source toggle (Tetrad navigation)
    content_source: ContentSource,
}

pub enum Msg {
    // Keep the old message for backward compatibility during transition
    SystemSelected(i32),
    // New messages for API integration - Language Tetrad Architecture
    DefinitionSelected(ContentItem),
    
    // Ground: User Instances (concrete personal applications)
    LoadUserInstances,
    UserInstancesLoaded(Result<Vec<UserInstance>, anyhow::Error>),
    
    // Directive: Core Grammar (Bennett's canonical terms)
    LoadCoreGrammars,
    CoreGrammarsLoaded(Result<Vec<CoreGrammar>, anyhow::Error>),
    
    // Instrumental: Community Grammar (user-contributed mappings)
    LoadCommunityGrammars,
    CommunityGrammarsLoaded(Result<Vec<CommunityGrammar>, anyhow::Error>),
    
    // Source: System Definitions (pure mathematical structures)
    SystemDefinitionLoaded(Result<SystemDefinition, anyhow::Error>),
    
    ApiError(String),
    // Search and browse functionality
    SearchQueryChanged(String),
    ToggleStructureBrowser,
    SearchDefinitions,
    SearchResultsLoaded(Result<Vec<UserInstance>, anyhow::Error>),
    // Creation functionality
    CreateDefinition,
    CancelCreate,
    SaveDefinition,
    DefinitionSaved(Result<String, anyhow::Error>),
    UserInstanceChanged(usize, String),
    ClearNotifications,
    // Content source toggle
    ContentSourceChanged(ContentSource),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let app = Self {
            user_instances: Vec::new(),
            core_grammars: Vec::new(),
            community_grammars: Vec::new(),
            filtered_content: Vec::new(),
            selected_item: None,
            current_definition: None,
            current_structure_type: StructureType::Monad, // Default to monad
            loading: false,
            error: None,
            success_message: None,
            api_client: ApiClient::new(),
            search_query: String::new(),
            show_content_browser: false,
            creation_mode: false,
            structure_name: None,
            user_input: Vec::new(),
            saving: false,
            content_source: ContentSource::CoreGrammar,
        };

        // Load core definitions by default and definition for monad
        ctx.link().send_message(Msg::LoadCoreGrammars);
        ctx.link().send_message(Msg::SystemSelected(1)); // Load monad by default
        
        app
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SystemSelected(system_num) => {
                // Update current system selection with enhanced type safety
                if let Some(structure_type) = StructureType::from_number(system_num) {
                    self.current_structure_type = structure_type;
                } else {
                    return false;
                }
                
                let structure_type_str = self.current_structure_type.to_string();
                
                // Find matching definition from loaded data (don't create placeholders)
                if let Some(definition) = self.user_instances.iter()
                    .find(|s| s.structure_type == structure_type_str && !s.id.as_str().map_or(false, |id| id.starts_with("placeholder-")))
                    .cloned() 
                {
                    self.selected_item = Some(ContentItem::UserInstance(definition));
                } else {
                    // No real definition found - clear selection to show definition data
                    self.selected_item = None;
                }
                
                // Always load definition for the selected structure type
                self.load_definition_for_structure_type(ctx, structure_type_str);
                true
            }
            Msg::DefinitionSelected(content_item) => {
                let structure_type = content_item.structure_type().to_string();
                self.selected_item = Some(content_item);
                self.show_content_browser = false; // Close browser after selection
                
                // Load definition for the selected structure type
                self.load_definition_for_structure_type(ctx, &structure_type);
                if let Some(struct_type) = StructureType::from_string(&structure_type) {
                    self.current_structure_type = struct_type;
                }
                true
            }
            Msg::LoadUserInstances => {
                self.loading = true;
                self.error = None;
                let api_client = self.api_client.clone();
                let callback = ctx.link().callback(Msg::UserInstancesLoaded);
                
                spawn_api_call(
                    async move { api_client.list_user_instances().await },
                    callback
                );
                true
            }
            Msg::LoadCoreGrammars => {
                self.loading = true;
                self.error = None;
                self.load_core_definitions(ctx);
                true
            }
            Msg::LoadCommunityGrammars => {
                self.loading = true;
                self.error = None;
                self.load_community_definitions(ctx);
                true
            }
            Msg::UserInstancesLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(definitions) => {
                        self.user_instances = definitions;
                        
                        // Update filtered content if we're currently showing user instances
                        if self.content_source == ContentSource::UserInstances {
                            self.filtered_content = self.user_instances.iter().map(|item| ContentItem::UserInstance(item.clone())).collect();
                        }
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to load user instances: {}", e));
                        self.user_instances = Vec::new();
                        if self.content_source == ContentSource::UserInstances {
                            self.filtered_content = Vec::new();
                        }
                    }
                }
                true
            }
                         Msg::CoreGrammarsLoaded(result) => {
                 self.loading = false;
                 match result {
                     Ok(core_grammars) => {
                         self.core_grammars = core_grammars;
                         
                         // Update filtered content if we're currently showing core grammars
                         if self.content_source == ContentSource::CoreGrammar {
                             self.filtered_content = self.core_grammars.iter().map(|item| ContentItem::CoreGrammar(item.clone())).collect();
                         }
                         self.error = None;
                     }
                     Err(e) => {
                         self.error = Some(format!("Failed to load core grammars: {}", e));
                         self.core_grammars = Vec::new();
                         if self.content_source == ContentSource::CoreGrammar {
                             self.filtered_content = Vec::new();
                         }
                     }
                 }
                 true
             }
            Msg::CommunityGrammarsLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(community_grammars) => {
                        self.community_grammars = community_grammars;
                        
                        // Update filtered content if we're currently showing community grammars
                        if self.content_source == ContentSource::CommunityGrammar {
                            self.filtered_content = self.community_grammars.iter().map(|item| ContentItem::CommunityGrammar(item.clone())).collect();
                        }
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to load community grammars: {}", e));
                        self.community_grammars = Vec::new();
                        if self.content_source == ContentSource::CommunityGrammar {
                            self.filtered_content = Vec::new();
                        }
                    }
                }
                true
            }
            Msg::SystemDefinitionLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(definition) => {
                        self.current_definition = Some(definition);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to load system definition: {}", e));
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
                self.show_content_browser = !self.show_content_browser;
                true
            }
            Msg::SearchDefinitions => {
                if self.search_query.trim().is_empty() {
                    self.filtered_content = self.user_instances.iter().map(|item| ContentItem::UserInstance(item.clone())).collect();
                    return true;
                }
                
                self.loading = true;
                let api_client = self.api_client.clone();
                let query = self.search_query.clone();
                let callback = ctx.link().callback(Msg::SearchResultsLoaded);
                
                spawn_api_call(
                    async move { api_client.search_user_instances(&query).await },
                    callback
                );
                true
            }
            Msg::SearchResultsLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(definitions) => {
                        self.filtered_content = definitions.iter().map(|item| ContentItem::UserInstance(item.clone())).collect();
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
            Msg::CreateDefinition => {
                if let Some(window) = window() {
                    if let Ok(Some(name)) = window.prompt_with_message("Enter a name for your definition:") {
                        if !name.trim().is_empty() {
                            self.structure_name = Some(name.trim().to_string());
                            self.creation_mode = true;
                            
                            // Initialize user_input with the right number of empty strings based on current system
                            let term_count = self.current_structure_type.term_count();
                            self.user_input = vec![String::new(); term_count];
                            
                            return true;
                        }
                    }
                }
                false
            }
            Msg::CancelCreate => {
                self.creation_mode = false;
                self.structure_name = None;
                self.user_input.clear();
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
                    Ok(_definition_id) => {
                        self.error = None;
                        self.success_message = Some(format!("✅ Definition saved successfully!"));
                        // Exit creation mode after successful save
                        self.creation_mode = false;
                        self.structure_name = None;
                        self.user_input.clear();
                        // Reload user instances to show the new one
                        ctx.link().send_message(Msg::LoadUserInstances);
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
                self.user_input[index] = instance;
                true
            }
            Msg::ClearNotifications => {
                self.success_message = None;
                self.error = None;
                true
            }
            Msg::ContentSourceChanged(source) => {
                self.content_source = source;
                
                // Switch data source and reload
                match source {
                    ContentSource::CoreGrammar => {
                        if self.core_grammars.is_empty() {
                            ctx.link().send_message(Msg::LoadCoreGrammars);
                        } else {
                            self.filtered_content = self.core_grammars.iter().map(|item| ContentItem::CoreGrammar(item.clone())).collect();
                        }
                    }
                    ContentSource::CommunityGrammar => {
                        if self.community_grammars.is_empty() {
                            ctx.link().send_message(Msg::LoadCommunityGrammars);
                        } else {
                            self.filtered_content = self.community_grammars.iter().map(|item| ContentItem::CommunityGrammar(item.clone())).collect();
                        }
                    }
                    ContentSource::UserInstances => {
                        if self.user_instances.is_empty() {
                            ctx.link().send_message(Msg::LoadUserInstances);
                        } else {
                            self.filtered_content = self.user_instances.iter().map(|item| ContentItem::UserInstance(item.clone())).collect();
                        }
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_system_selected = ctx.link().callback(Msg::SystemSelected);
        
        // Determine the structure type and system number from selected definition or current selection
        let (structure_type, system_num) = if let Some(ref item) = self.selected_item {
            if let Some(struct_type) = StructureType::from_string(item.structure_type()) {
                (item.structure_type().to_string(), struct_type.to_number())
            } else {
                (self.current_structure_type.to_string().to_string(), self.current_structure_type.to_number())
            }
        } else {
            // Use current structure type selection
            (self.current_structure_type.to_string().to_string(), self.current_structure_type.to_number())
        };

        // Create system-specific CSS class
        let system_class = format!("system-{}", system_num);

        html! {
            <div class="app-container">
                {self.render_header(ctx)}
                {self.render_search_controls(ctx)}
                <div class="system-selector-container">
                    <SystemSelector 
                        {on_system_selected} 
                        selected_system={system_num}
                        disabled={self.creation_mode}
                    />
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
                {self.render_content_browser(ctx)}
                {self.render_notifications()}
            </div>
        }
    }
}

impl App {
    fn render_header(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header class="app-header">
                <div class="header-content">
                    <div class="header-title">
                        <h1>{"SysteMaster"}</h1>
                        <p>{"Systematic Learning Interface"}</p>
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
        
        // Content source callbacks
        let core_callback = ctx.link().callback(|_| Msg::ContentSourceChanged(ContentSource::CoreGrammar));
        let community_callback = ctx.link().callback(|_| Msg::ContentSourceChanged(ContentSource::CommunityGrammar));
        let user_instances_callback = ctx.link().callback(|_| Msg::ContentSourceChanged(ContentSource::UserInstances));
        
        html! {
            <div class="search-controls">
                <div class="search-bar">
                    <div class="content-source-tabs">
                        <button 
                            class={classes!("tab-button", if self.content_source == ContentSource::CoreGrammar { "active" } else { "" })}
                            onclick={core_callback}
                            disabled={self.creation_mode}
                        >
                            {"Core Grammar"}
                        </button>
                        <button 
                            class={classes!("tab-button", if self.content_source == ContentSource::CommunityGrammar { "active" } else { "" })}
                            onclick={community_callback}
                            disabled={self.creation_mode}
                        >
                            {"Community Grammar"}
                        </button>
                        <button 
                            class={classes!("tab-button", if self.content_source == ContentSource::UserInstances { "active" } else { "" })}
                            onclick={user_instances_callback}
                            disabled={self.creation_mode}
                        >
                            {"User Instances"}
                        </button>
                    </div>
                    <div class="action-buttons">
                        <button 
                            class="load-button" 
                            onclick={ctx.link().callback(|_| Msg::ToggleStructureBrowser)}
                            disabled={self.creation_mode}
                        >
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
                        placeholder={match self.content_source {
                            ContentSource::CoreGrammar => "Search core grammar definitions...",
                            ContentSource::CommunityGrammar => "Search community grammar definitions...",
                            ContentSource::UserInstances => "Search user instances...",
                        }}
                        value={self.search_query.clone()}
                        oninput={search_input}
                        class="search-input"
                        disabled={self.creation_mode}
                    />
                    <button 
                        onclick={search_submit} 
                        class="search-button"
                        disabled={self.creation_mode}
                    >
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
        if let Some(ref item) = self.selected_item {
            let system_num = StructureType::from_string(item.structure_type()).map(|s| s.to_number()).unwrap_or(1);
            html! {
                <SystemOverlay 
                    system_num={system_num} 
                    definition={Some(item.clone())} 
                    creation_mode={self.creation_mode}
                    structure_name={self.structure_name.clone()}
                    user_instance_index={self.user_input.clone()}
                    on_instance_change={ctx.link().callback(|(index, instance)| Msg::UserInstanceChanged(index, instance))}
                />
            }
        } else {
            html! {
                <SystemOverlay 
                    system_num={self.current_structure_type.to_number()} 
                    definition={None::<ContentItem>}
                    creation_mode={self.creation_mode}
                    structure_name={self.structure_name.clone()}
                    user_instance_index={self.user_input.clone()}
                    on_instance_change={ctx.link().callback(|(index, instance)| Msg::UserInstanceChanged(index, instance))}
                />
            }
        }
    }

    fn render_content_browser(&self, ctx: &Context<Self>) -> Html {
        if !self.show_content_browser {
            return html! {};
        }
        
        let definitions_to_show = if self.search_query.trim().is_empty() {
            &self.filtered_content
        } else {
            &self.filtered_content
        };
        
        let browser_title = match self.content_source {
            ContentSource::CoreGrammar => "Core Grammar Definitions",
            ContentSource::CommunityGrammar => "Community Grammar Definitions",
            ContentSource::UserInstances => "User Instances",
        };
        
        let empty_message = match self.content_source {
            ContentSource::CoreGrammar => "No core grammar definitions available",
            ContentSource::CommunityGrammar => "No community grammar definitions found",
            ContentSource::UserInstances => "No user instances found",
        };
        
        html! {
            <div class="content-browser-overlay">
                <div class="content-browser">
                    <div class="browser-header">
                        <div class="browser-title-section">
                            <h3>{browser_title}</h3>
                            <span class="content-source-badge">
                                {match self.content_source {
                                    ContentSource::CoreGrammar => "🎯 Curated",
                                    ContentSource::CommunityGrammar => "👥 Community",
                                    ContentSource::UserInstances => "👤 User",
                                }}
                            </span>
                        </div>
                        <button 
                            class="close-browser" 
                            onclick={ctx.link().callback(|_| Msg::ToggleStructureBrowser)}
                        >
                            {"×"}
                        </button>
                    </div>
                    <div class="content-list">
                        {for definitions_to_show.iter().map(|item| {
                            let item_clone = item.clone();
                            let select_item = ctx.link().callback(move |_| {
                                Msg::DefinitionSelected(item_clone.clone())
                            });
                            
                            html! {
                                <div class="content-item" onclick={select_item}>
                                    <div class="content-item-header">
                                        <h4>{&item.name()}</h4>
                                        <span class="structure-type">{&item.structure_type()}</span>
                                    </div>
                                    <div class="content-item-terms">
                                        {item.instances().join(", ")}
                                    </div>
                                    {if let Some(ref desc) = item.description() {
                                        html! { 
                                            <div class="content-item-description">
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
                                {empty_message}
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
            self.filtered_content = match self.content_source {
                ContentSource::CoreGrammar => {
                    self.core_grammars.iter().map(|item| ContentItem::CoreGrammar(item.clone())).collect()
                }
                ContentSource::CommunityGrammar => {
                    self.community_grammars.iter().map(|item| ContentItem::CommunityGrammar(item.clone())).collect()
                }
                ContentSource::UserInstances => {
                    self.user_instances.iter().map(|item| ContentItem::UserInstance(item.clone())).collect()
                }
            };
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_content = match self.content_source {
                ContentSource::CoreGrammar => {
                    self.core_grammars
                        .iter()
                        .filter(|item| {
                            item.name.to_lowercase().contains(&query) ||
                            item.structure_type.to_lowercase().contains(&query) ||
                            item.term_characters.iter().any(|instance| instance.to_lowercase().contains(&query))
                        })
                        .map(|item| ContentItem::CoreGrammar(item.clone()))
                        .collect()
                }
                ContentSource::CommunityGrammar => {
                    self.community_grammars
                        .iter()
                        .filter(|item| {
                            item.name.to_lowercase().contains(&query) ||
                            item.structure_type.to_lowercase().contains(&query) ||
                            item.term_characters.iter().any(|instance| instance.to_lowercase().contains(&query)) ||
                            item.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query))
                        })
                        .map(|item| ContentItem::CommunityGrammar(item.clone()))
                        .collect()
                }
                ContentSource::UserInstances => {
                    self.user_instances
                        .iter()
                        .filter(|item| {
                            item.name.to_lowercase().contains(&query) ||
                            item.structure_type.to_lowercase().contains(&query) ||
                            item.instances.iter().any(|instance| instance.to_lowercase().contains(&query)) ||
                            item.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query))
                        })
                        .map(|item| ContentItem::UserInstance(item.clone()))
                        .collect()
                }
            };
        }
    }



    fn load_definition_for_structure_type(&self, ctx: &Context<Self>, structure_type: &str) {
        let api_client = self.api_client.clone();
        let structure_type = structure_type.to_string();
        let callback = ctx.link().callback(Msg::SystemDefinitionLoaded);
        
        spawn_api_call(
            async move {
                api_client.get_system_definition(&structure_type).await
            },
            callback,
        );
    }

    fn load_core_definitions(&self, ctx: &Context<Self>) {
        let api_client = self.api_client.clone();
        let callback = ctx.link().callback(Msg::CoreGrammarsLoaded);
        
        spawn_api_call(
            async move {
                let mut core_grammars = Vec::new();
                let structure_types = ["monad", "dyad", "triad", "tetrad", "pentad", "hexad", "heptad", "octad", "ennead", "decad", "undecad", "dodecad"];
                
                for structure_type in structure_types {
                    if let Ok(core_grammar) = api_client.get_core_grammar(structure_type).await {
                        core_grammars.push(core_grammar);
                    }
                }
                
                Ok(core_grammars)
            },
            callback,
        );
    }

    fn load_community_definitions(&self, ctx: &Context<Self>) {
        let api_client = self.api_client.clone();
        let callback = ctx.link().callback(Msg::CommunityGrammarsLoaded);
        
        spawn_api_call(
            async move {
                api_client.list_community_grammars(None).await
            },
            callback,
        );
    }

    fn save_definition(&self, ctx: &Context<Self>) {
        let structure_type = self.current_structure_type.to_string().to_string();
        
        let definition_name = self.structure_name.clone().unwrap_or_else(|| "Unnamed Definition".to_string());
        
        let api_client = self.api_client.clone();
        let user_instances = self.user_input.clone();
        let callback = ctx.link().callback(Msg::DefinitionSaved);
        
        spawn_api_call(
            async move {
                api_client.save_user_instance(&definition_name, &structure_type, &user_instances).await
            },
            callback,
        );
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
} 