use yew::{html, Component, Context, Html, Properties};
use crate::services::api::{StoredStructure, ApiClient, StructureSchema, spawn_api_call};
use crate::core::geometry::GeometryCalculator;
use std::f64::consts::PI;
use web_sys;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub system_num: i32,
    #[prop_or_default]
    pub structure: Option<StoredStructure>,
    #[prop_or_default]
    pub creation_mode: bool,
    #[prop_or_default]
    pub structure_name: Option<String>,
}

pub enum Msg {
    SchemaLoaded(Result<StructureSchema, anyhow::Error>),
}

pub struct SystemOverlay {
    schema: Option<StructureSchema>,
    api_client: ApiClient,
    loading_schema: bool,
}

impl Component for SystemOverlay {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut component = Self {
            schema: None,
            api_client: ApiClient::new(),
            loading_schema: false,
        };
        
        // Load schema for the current system
        component.load_schema_for_system(ctx, ctx.props().system_num);
        
        component
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SchemaLoaded(result) => {
                self.loading_schema = false;
                match result {
                    Ok(schema) => {
                        self.schema = Some(schema);
                        true
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Failed to load schema: {}", err).into());
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
            self.load_schema_for_system(ctx, new_props.system_num);
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
        if new_props.structure != old_props.structure {
            should_update = true;
        }
        
        should_update
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let system_num = ctx.props().system_num;
        let structure = &ctx.props().structure;
        let creation_mode = ctx.props().creation_mode;
        

        
        // Show loading state while schema is being fetched
        if self.loading_schema {
            return html! {
                <div class="system-overlay loading">
                    <div class="loading-message">
                        <p>{"Loading structure schema..."}</p>
                    </div>
                </div>
            };
        }
        
        // Show error state if schema failed to load
        if self.schema.is_none() {
            return html! {
                <div class="system-overlay error">
                    <div class="error-message">
                        <p>{"Unable to load structure schema"}</p>
                        <p><small>{"Please check your connection and try again"}</small></p>
                    </div>
                </div>
            };
        }
        
        html! {
            <>
                {self.render_control_buttons(ctx)}
                {self.render_structure_content(ctx, system_num, structure)}
            </>
        }
    }
}

impl SystemOverlay {
    fn render_control_buttons(&self, ctx: &Context<Self>) -> Html {
        // Controls are now handled by the parent App component
        html! {}
    }
    
    fn render_structure_content(&self, ctx: &Context<Self>, system_num: i32, structure: &Option<StoredStructure>) -> Html {
        match system_num {
            1 => self.render_monad(ctx, structure),
            2 => self.render_dyad(ctx, structure),
            3 => self.render_triad(ctx, structure),
            4 => self.render_tetrad(ctx, structure),
            5 => self.render_pentad(ctx, structure),
            6 => self.render_hexad(ctx, structure),
            7 => self.render_heptad(ctx, structure),
            8 => self.render_octad(ctx, structure),
            9 => self.render_ennead(ctx, structure),
            10 => self.render_decad(ctx, structure),
            11 => self.render_undecad(ctx, structure),
            12 => self.render_dodecad(ctx, structure),
            _ => html! { <div class="system-overlay">{"Unsupported system"}</div> },
        }
    }

    fn load_schema_for_system(&mut self, ctx: &Context<Self>, system_num: i32) {
        if self.loading_schema {
            return;
        }
        
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
            _ => return,
        };
        
        self.loading_schema = true;
        let api_client = self.api_client.clone();
        let callback = ctx.link().callback(Msg::SchemaLoaded);
        
        spawn_api_call(
            async move {
                api_client.get_structure_schema(structure_type).await
            },
            callback,
        );
    }

    fn get_term_character(&self, position: usize) -> Option<String> {
        self.schema
            .as_ref()
            .and_then(|s| s.term_characters.get(position))
            .cloned()
    }
    
    fn render_system_with_terms(&self, ctx: &Context<Self>, system_type: &str, expected_terms: usize) -> Html {
        let svg_size = 400.0;
        let points = self.get_system_layout(system_type, svg_size);
        
        let terms: Result<Vec<String>, ()> = (0..expected_terms)
            .map(|i| self.get_term_character(i).ok_or(()))
            .collect();
            
        match terms {
            Ok(term_characters) => {
                self.render_structure_with_points(ctx, &term_characters, &points, svg_size, system_type)
            }
            Err(_) => {
                html! {
                    <div class="system-overlay incomplete">
                        <div class="incomplete-message">
                            <p>{format!("Incomplete schema for {} - missing term characters", system_type)}</p>
                        </div>
                    </div>
                }
            }
        }
    }
    
    fn render_structure_with_points(&self, ctx: &Context<Self>, terms: &[String], points: &[(f64, f64)], svg_size: f64, system_type: &str) -> Html {
        let is_octad = system_type == "octad";
        let point_elements: Vec<Html> = terms.iter()
            .zip(points.iter())
            .enumerate()
            .map(|(i, (term, (x, y)))| {
                let (adjusted_x, adjusted_y) = self.apply_label_positioning(system_type, i, *x, *y, svg_size, points);
                let top = self.svg_to_css_percent(adjusted_y, svg_size);
                let left = self.svg_to_css_percent(adjusted_x, svg_size);
                self.render_point_with_positioning(ctx, term, &top, &left, is_octad)
            })
            .collect();
            
        html! {
            <div class="system-overlay">
                { for point_elements }
            </div>
        }
    }
    
    fn apply_label_positioning(&self, system_type: &str, index: usize, x: f64, y: f64, svg_size: f64, points: &[(f64, f64)]) -> (f64, f64) {
        match system_type {
            "monad" => {
                (x, y)
            },
            "dyad" => {
                (x, y)
            },
            "triad" => {
                match index {
                    0 => (x, y - 45.0),
                    1 => (x + 60.0, y),
                    2 => (x, y + 45.0),
                    _ => (x, y),
                }
            },
            "tetrad" => {
                match index {
                    0 => (x, y - 50.0),
                    1 => (x + 60.0, y),
                    2 => (x - 65.0, y),
                    3 => (x, y + 50.0),
                    _ => (x, y),
                }
            },
            "pentad" => {
                match index {
                    0 => (x, y - 45.0),
                    1 => (x, y - 45.0),
                    2 => (x - 65.0, y),
                    3 => (x, y + 45.0),
                    4 => (x, y + 45.0),
                    _ => (x, y),
                }
            },
            "hexad" => {
                let push_distance = 45.0;
                let diagonal_push = 60.0;
                match index {
                    0 => (x - diagonal_push * 0.7, y - diagonal_push * 0.7),
                    1 => (x, y - push_distance),
                    2 => (x + diagonal_push * 0.7, y - diagonal_push * 0.7),
                    3 => (x + diagonal_push * 0.7, y + diagonal_push * 0.7),
                    4 => (x, y + push_distance),
                    5 => (x - diagonal_push * 0.7, y + diagonal_push * 0.7),
                    _ => (x, y),
                }
            },
            "heptad" => {
                let push_distance = 60.0;
                let center_x = svg_size / 2.0;
                match index {
                    0 => (x, y - push_distance + 15.0),
                    1 => {
                        if points.len() > 6 {
                            let value_x = points[6].0 - push_distance;
                            let value_distance_from_center = center_x - value_x;
                            let research_x = center_x + value_distance_from_center;
                            (research_x, y - push_distance * 0.7)
                        } else {
                            (x + push_distance, y - push_distance * 0.7)
                        }
                    },
                    2 => (x + push_distance, y + push_distance * 0.7 - 40.0),
                    3 => (x + push_distance * 0.7, y + push_distance * 0.7),
                    4 => {
                        if points.len() > 3 {
                            let synthesis_x = points[3].0 + push_distance * 0.7;
                            let synthesis_distance_from_center = synthesis_x - center_x;
                            let application_x = center_x - synthesis_distance_from_center;
                            (application_x, points[3].1 + push_distance * 0.7)
                        } else {
                            (x - push_distance * 0.7, y + push_distance * 0.7)
                        }
                    },
                    5 => {
                        if points.len() > 2 {
                            let design_x = points[2].0 + push_distance;
                            let design_distance_from_center = design_x - center_x;
                            let delivery_x = center_x - design_distance_from_center;
                            (delivery_x, points[2].1 + push_distance * 0.7 - 40.0)
                        } else {
                            (x - push_distance, y + push_distance * 0.7 - 40.0)
                        }
                    },
                    6 => (x - push_distance, y - push_distance * 0.7),
                    _ => (x, y),
                }
            },
            "octad" => {
                let push_distance = 60.0;
                let diagonal_push = 70.0;
                match index {
                    0 => (x + push_distance, y),
                    1 => (x + diagonal_push * 0.7, y + diagonal_push * 0.7),
                    2 => (x, y + push_distance),
                    3 => (x - diagonal_push * 0.7, y + diagonal_push * 0.7),
                    4 => (x - push_distance, y),
                    5 => (x - diagonal_push * 0.7, y - diagonal_push * 0.7),
                    6 => (x, y - push_distance),
                    7 => (x + diagonal_push * 0.7, y - diagonal_push * 0.7),
                    _ => (x, y),
                }
            },
            _ => {
                let radius = svg_size * 0.05;
                let center_x = svg_size / 2.0;
                let center_y = svg_size / 2.0;
                let dx = x - center_x;
                let dy = y - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance > 0.0 {
                    let push_x = dx / distance * radius;
                    let push_y = dy / distance * radius;
                    (x + push_x, y + push_y)
                } else {
                    (x, y)
                }
            }
        }
    }

    fn get_system_layout(&self, system_type: &str, svg_size: f64) -> Vec<(f64, f64)> {
        let center = svg_size / 2.0;
        let layout = GeometryCalculator::calculate_system_layout(system_type, center, center, svg_size);
        layout.nodes.into_iter().map(|point| (point.x, point.y)).collect()
    }

    fn regular_polygon_points(&self, n: usize, cx: f64, cy: f64, radius: f64, rotation: f64) -> Vec<(f64, f64)> {
        (0..n).map(|i| {
            let angle = 2.0 * PI * i as f64 / n as f64 + rotation;
            let x = cx + radius * angle.cos();
            let y = cy + radius * angle.sin();
            (x, y)
        }).collect()
    }

    fn svg_to_css_percent(&self, coord: f64, svg_size: f64) -> String {
        format!("{}%", (coord / svg_size) * 100.0)
    }

    fn render_point(&self, ctx: &Context<Self>, term: &str, top: &str, left: &str) -> Html {
        self.render_point_with_positioning(ctx, term, top, left, false)
    }
    
    fn render_point_with_positioning(&self, ctx: &Context<Self>, term: &str, top: &str, left: &str, is_octad: bool) -> Html {
        let creation_mode = ctx.props().creation_mode;
        

        
        if is_octad {
            let (formatted_term, css_class, container_style) = match term {
                "Smallest Significant Holon" => {
                    let adjusted_left = format!("{}%", left.trim_end_matches('%').parse::<f64>().unwrap_or(0.0) - 8.75);
                    (html! { {term} }, "point-label", format!("top: {}; left: {}; transform: translate(0%, -50%);", top, adjusted_left))
                },
                "Integrative Totality" => {
                    let adjusted_left = format!("{}%", left.trim_end_matches('%').parse::<f64>().unwrap_or(0.0) + 8.75);
                    (html! { {term} }, "point-label", format!("top: {}; left: {}; transform: translate(-100%, -50%);", top, adjusted_left))
                },
                _ => {
                    (html! { {term} }, "point-label", format!("top: {}; left: {}; transform: translate(-50%, -50%);", top, left))
                }
            };
            
            html! {
                <div class={format!("point-container {}", if creation_mode { "creation-mode" } else { "display-mode" })} style={container_style}>
                    <div class={css_class}>{formatted_term}</div>
                    {if creation_mode {
                        html! { <input class="point-input" placeholder="Enter term..." /> }
                    } else {
                        html! {}
                    }}
                </div>
            }
        } else {
            html! {
                <div class={format!("point-container {} {}", if creation_mode { "creation-mode" } else { "display-mode" }, if is_octad { "octad" } else { "" })} style={format!("top: {}; left: {}; transform: translate(-50%, -50%);", top, left)}>
                    <div class="point-label">{term}</div>
                    {if creation_mode {
                        html! { <input class="point-input" placeholder="Enter term..." /> }
                    } else {
                        html! {}
                    }}
                </div>
            }
        }
    }

    fn render_monad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "monad", 1)
    }

    fn render_dyad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "dyad", 2)
    }

    fn render_triad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "triad", 3)
    }

    fn render_tetrad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "tetrad", 4)
    }

    fn render_pentad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "pentad", 5)
    }

    fn render_hexad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "hexad", 6)
    }

    fn render_heptad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "heptad", 7)
    }

    fn render_octad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "octad", 8)
    }

    fn render_ennead(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "ennead", 9)
    }

    fn render_decad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "decad", 10)
    }

    fn render_undecad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "undecad", 11)
    }

    fn render_dodecad(&self, ctx: &Context<Self>, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms(ctx, "dodecad", 12)
    }
} 