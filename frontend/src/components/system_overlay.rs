// TODO: REFACTOR NEEDED - This file has accumulated technical debt
// - Terminology inconsistencies throughout
// - Complex positioning logic that could be simplified
// - Mixed responsibilities (rendering, positioning, data handling)
// - Method names that don't follow our terminology rules
// - Hardcoded positioning values that should be configurable
// Priority: High - affects core systematic structure display

use yew::{html, Component, Context, Html, Properties, Callback};
use crate::services::api::{ApiClient, SystemDefinition, spawn_api_call};
use crate::core::geometry::GeometryCalculator;
use crate::{ContentItem, ContentSource, AddContentType};
use crate::components::geometric_renderer::GeometricRenderer;

use web_sys;

#[derive(Properties, PartialEq)]
pub struct SystemOverlayProps {
    pub system_num: i32,
    pub definition: Option<ContentItem>,
    pub creation_mode: bool,
    pub structure_name: Option<String>,
    pub user_expressions: Vec<String>,
    pub on_instance_change: Callback<(usize, String)>,
    pub content_source: ContentSource,
    pub selected_add_type: Option<AddContentType>,
}

pub enum SystemOverlayMsg {
    DefinitionLoaded(Result<SystemDefinition, anyhow::Error>),
}

pub struct SystemOverlay {
    current_definition: Option<SystemDefinition>,
    api_client: ApiClient,
    loading_definition: bool,
}

impl Component for SystemOverlay {
    type Message = SystemOverlayMsg;
    type Properties = SystemOverlayProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut component =         Self {
            current_definition: None,
            api_client: ApiClient::new(),
            loading_definition: false,
        };
        
        // Load definition for the current system
        component.load_definition_for_system(ctx, ctx.props().system_num);
        
        component
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SystemOverlayMsg::DefinitionLoaded(result) => {
                self.loading_definition = false;
                match result {
                    Ok(definition) => {
                        self.current_definition = Some(definition);
                        true
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Failed to load definition: {}", err).into());
                        true
                    }
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let new_props = ctx.props();
        let mut should_update = false;
        
        // Check if system_num changed
        if new_props.system_num != old_props.system_num {
            self.load_definition_for_system(ctx, new_props.system_num);
            should_update = true;
        }
        
        // Check if creation_mode changed
        if new_props.creation_mode != old_props.creation_mode {
            should_update = true;
        }
        
        // Check if structure_name changed
        if new_props.structure_name != old_props.structure_name {
            should_update = true;
        }
        
        // Check if structure changed
        if new_props.definition != old_props.definition {
            should_update = true;
        }
        
        // Check if user_expressions changed
        if new_props.user_expressions != old_props.user_expressions {
            should_update = true;
        }
        
        should_update
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let system_num = ctx.props().system_num;
        let definition = &ctx.props().definition;
        let _creation_mode = ctx.props().creation_mode;
        

        
        // Show loading state while definition is being fetched
        if self.loading_definition {
            return html! {
                <div class="system-overlay loading">
                    <div class="loading-message">
                        <p>{"Loading system definition..."}</p>
                    </div>
                </div>
            };
        }
        
        // Show error state if definition failed to load
        if self.current_definition.is_none() {
            return html! {
                <div class="system-overlay error">
                    <div class="error-message">
                        <p>{"Unable to load system definition"}</p>
                        <p><small>{"Please check your connection and try again"}</small></p>
                    </div>
                </div>
            };
        }
        
        html! {
            <>
                {self.render_control_buttons(ctx)}
                {self.render_structure_content(ctx, system_num, definition)}
            </>
        }
    }
}

impl SystemOverlay {
    fn render_creation_header(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        
        if !props.creation_mode {
            return html! {};
        }
        
        let (content_type_name, content_type_icon) = if let Some(add_type) = props.selected_add_type {
            match add_type {
                AddContentType::System => ("System", "📐"),
                AddContentType::Paper => ("Paper", "📄"),
                AddContentType::SystemCollection => ("Collection", "📚"),
                AddContentType::Module => ("Module", "🎓"),
                AddContentType::Book => ("Book", "📖"),
                AddContentType::Definition => ("Definition", "📋"),
                AddContentType::Collection => ("Collection", "📚"),
            }
        } else {
            ("Content", "📝")
        };
        
        let structure_name = props.structure_name.as_deref().unwrap_or("Untitled");
        let content_source_name = match props.content_source {
            ContentSource::CoreGrammar => "Core Grammar",
            ContentSource::CommunityGrammar => "Community Grammar", 
            ContentSource::UserExpressions => "User Expressions",
        };
        
        html! {
            <div class="creation-header">
                <div class="creation-header-content">
                    <span class="creation-icon">{content_type_icon}</span>
                    <div class="creation-info">
                        <h4>{format!("Creating {} in {}", content_type_name, content_source_name)}</h4>
                        <p>{structure_name}</p>
                    </div>
                </div>
            </div>
        }
    }

    fn render_control_buttons(&self, _ctx: &Context<Self>) -> Html {
        // Controls are now handled by the parent App component
        html! {}
    }
    
    fn render_structure_content(&self, ctx: &Context<Self>, system_num: i32, definition: &Option<ContentItem>) -> Html {
        match system_num {
            1 => self.render_monad(ctx, definition),
            2 => self.render_dyad(ctx, definition),
            3 => self.render_triad(ctx, definition),
            4 => self.render_tetrad(ctx, definition),
            5 => self.render_pentad(ctx, definition),
            6 => self.render_hexad(ctx, definition),
            7 => self.render_heptad(ctx, definition),
            8 => self.render_octad(ctx, definition),
            9 => self.render_ennead(ctx, definition),
            10 => self.render_decad(ctx, definition),
            11 => self.render_undecad(ctx, definition),
            12 => self.render_dodecad(ctx, definition),
            _ => html! { <div class="system-overlay">{"Unsupported system"}</div> },
        }
    }

    fn load_definition_for_system(&mut self, ctx: &Context<Self>, system_num: i32) {
        if self.loading_definition {
            return;
        }
        
        let definition_type = match system_num {
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
            _ => return,
        };
        
        self.loading_definition = true;
        let api_client = self.api_client.clone();
        let callback = ctx.link().callback(SystemOverlayMsg::DefinitionLoaded);
        
        spawn_api_call(
            async move {
                api_client.get_system_definition(definition_type).await
            },
            callback,
        );
    }

    fn get_term_character(&self, position: usize) -> Option<String> {
        self.current_definition
            .as_ref()
            .and_then(|s| s.term_characters.get(position))
            .cloned()
    }
    
    fn render_system_with_definition(&self, ctx: &Context<Self>, system_type: &str, expected_count: usize) -> Html {
        let svg_size = self.get_system_size(system_type);
        let points = self.get_system_layout(system_type, svg_size);
        
        // Determine what to display based on context
        let display_values: Vec<String> = if let Some(ref definition) = ctx.props().definition {
            match definition {
                ContentItem::CoreGrammar(_) | ContentItem::CommunityGrammar(_) | ContentItem::UserExpression(_) => {
                    // Use user expressions if available, otherwise fall back to term characters
                    if ctx.props().user_expressions.len() >= expected_count && 
                       ctx.props().user_expressions.iter().any(|s| !s.is_empty()) {
                        ctx.props().user_expressions[..expected_count].to_vec()
                    } else {
                        // Fallback to term characters
                        (0..expected_count)
                            .map(|i| self.get_term_character(i).unwrap_or_else(|| format!("Position {}", i + 1)))
                            .collect()
                    }
                }
            }
        } else {
            // No definition loaded - show term characters
            (0..expected_count)
                .map(|i| self.get_term_character(i).unwrap_or_else(|| format!("Position {}", i + 1)))
                .collect()
        };
        
        self.render_structure_with_points(ctx, &display_values, &points, svg_size, system_type)
    }
    
    fn render_structure_with_points(&self, ctx: &Context<Self>, display_values: &[String], _points: &[(f64, f64)], svg_size: f64, system_type: &str) -> Html {
        // Get connectives from the current definition
        let connectives = self.current_definition.as_ref().map(|def| {
            def.connectives.clone()
        });
            
        html! {
            <div class={format!("system-overlay system-{}", self.get_system_number(system_type))}>
                <GeometricRenderer 
                    system_type={system_type.to_string()}
                    size={svg_size}
                    connectives={connectives}
                    labels={Some(display_values.to_vec())}
                    creation_mode={ctx.props().creation_mode}
                    user_inputs={if ctx.props().creation_mode {
                        Some(ctx.props().user_expressions.clone())
                    } else {
                        None
                    }}
                    on_input_change={if ctx.props().creation_mode {
                        Some(ctx.props().on_instance_change.clone())
                    } else {
                        None
                    }}
                />
            </div>
        }
    }
    
    fn get_system_layout(&self, system_type: &str, svg_size: f64) -> Vec<(f64, f64)> {
        let center = svg_size / 2.0;
        let layout = GeometryCalculator::calculate_system_layout(system_type, center, center, svg_size);
        layout.nodes.into_iter().map(|point| (point.x, point.y)).collect()
    }
    
    fn get_system_number(&self, system_type: &str) -> usize {
        match system_type {
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
    
    fn get_system_size(&self, _system_type: &str) -> f64 {
        // Fixed canvas size for all systems - structures scale within this canvas
        500.0
    }
    


    fn render_monad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "monad", 1)
    }

    fn render_dyad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "dyad", 2)
    }

    fn render_triad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "triad", 3)
    }

    fn render_tetrad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "tetrad", 4)
    }

    fn render_pentad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "pentad", 5)
    }

    fn render_hexad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "hexad", 6)
    }

    fn render_heptad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "heptad", 7)
    }

    fn render_octad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "octad", 8)
    }

    fn render_ennead(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "ennead", 9)
    }

    fn render_decad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "decad", 10)
    }

    fn render_undecad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "undecad", 11)
    }

    fn render_dodecad(&self, ctx: &Context<Self>, _definition: &Option<ContentItem>) -> Html {
        self.render_system_with_definition(ctx, "dodecad", 12)
    }
} 