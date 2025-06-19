use yew::{html, Component, Context, Html, Properties};
use crate::services::api::{StoredStructure, ApiClient, StructureSchema, spawn_api_call};
use crate::core::geometry::GeometryCalculator;
use std::f64::consts::PI;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub system_num: i32,
    #[prop_or_default]
    pub structure: Option<StoredStructure>,
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
        if ctx.props().system_num != old_props.system_num {
            self.load_schema_for_system(ctx, ctx.props().system_num);
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let system_num = ctx.props().system_num;
        let structure = &ctx.props().structure;
        
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
        
        // Render the appropriate structure
        match system_num {
            1 => self.render_monad(structure),
            2 => self.render_dyad(structure),
            3 => self.render_triad(structure),
            4 => self.render_tetrad(structure),
            5 => self.render_pentad(structure),
            6 => self.render_hexad(structure),
            7 => self.render_heptad(structure),
            8 => self.render_octad(structure),
            9 => self.render_ennead(structure),
            10 => self.render_decad(structure),
            11 => self.render_undecad(structure),
            12 => self.render_dodecad(structure),
            _ => html! { <div class="system-overlay">{"Unsupported system"}</div> },
        }
    }
}

impl SystemOverlay {
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
    
    fn render_system_with_terms(&self, system_type: &str, expected_terms: usize) -> Html {
        let svg_size = 400.0;
        let points = self.get_system_layout(system_type, svg_size);
        
        // Check if we have all required term characters
        let terms: Result<Vec<String>, ()> = (0..expected_terms)
            .map(|i| self.get_term_character(i).ok_or(()))
            .collect();
            
        match terms {
            Ok(term_characters) => {
                // All terms available - render the structure
                self.render_structure_with_points(&term_characters, &points, svg_size, system_type)
            }
            Err(_) => {
                // Missing terms - show error
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
    
    fn render_structure_with_points(&self, terms: &[String], points: &[(f64, f64)], svg_size: f64, system_type: &str) -> Html {
        let is_octad = system_type == "octad";
        let point_elements: Vec<Html> = terms.iter()
            .zip(points.iter())
            .enumerate()
            .map(|(i, (term, (x, y)))| {
                let (adjusted_x, adjusted_y) = self.apply_label_positioning(system_type, i, *x, *y, svg_size, points);
                let top = self.svg_to_css_percent(adjusted_y, svg_size);
                let left = self.svg_to_css_percent(adjusted_x, svg_size);
                self.render_point_with_positioning(term, &top, &left, is_octad)
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
                // Center position - no adjustment needed
                (x, y)
            },
            "dyad" => {
                // Centered on nodes for dyad
                (x, y)
            },
            "triad" => {
                match index {
                    0 => (x, y - 45.0),      // Will (top-left) - pull closer to node
                    1 => (x + 60.0, y),      // Being (right) - push further to the right
                    2 => (x, y + 45.0),      // Function (bottom-left) - pull closer to node
                    _ => (x, y),
                }
            },
            "tetrad" => {
                match index {
                    0 => (x, y - 50.0),      // Ideal (top)
                    1 => (x + 60.0, y),      // Directive (right) - push further right
                    2 => (x - 65.0, y),      // Instrumental (left) - push further left
                    3 => (x, y + 50.0),      // Ground (bottom)
                    _ => (x, y),
                }
            },
            "pentad" => {
                match index {
                    0 => (x, y - 45.0),      // Purpose: above node
                    1 => (x, y - 45.0),      // Higher Potential: above node
                    2 => (x - 65.0, y),      // Quintessence: left of node
                    3 => (x, y + 45.0),      // Lower Potential: below node
                    4 => (x, y + 45.0),      // Source: below node
                    _ => (x, y),
                }
            },
            "hexad" => {
                let push_distance = 45.0;
                let diagonal_push = 60.0;
                match index {
                    0 => (x - diagonal_push * 0.7, y - diagonal_push * 0.7), // Resources (top-left)
                    1 => (x, y - push_distance),                              // Values (top)
                    2 => (x + diagonal_push * 0.7, y - diagonal_push * 0.7), // Options (top-right)
                    3 => (x + diagonal_push * 0.7, y + diagonal_push * 0.7), // Criteria (bottom-right)
                    4 => (x, y + push_distance),                              // Facts (bottom)
                    5 => (x - diagonal_push * 0.7, y + diagonal_push * 0.7), // Priorities (bottom-left)
                    _ => (x, y),
                }
            },
                         "heptad" => {
                 let push_distance = 60.0;
                 let center_x = svg_size / 2.0;
                 match index {
                     0 => (x, y - push_distance + 15.0), // Insight (top) - lowered by 15px
                     1 => {
                         // Research - calculate symmetric position to Value (index 6)
                         if points.len() > 6 {
                             let value_x = points[6].0 - push_distance;
                             let value_distance_from_center = center_x - value_x;
                             let research_x = center_x + value_distance_from_center;
                             (research_x, y - push_distance * 0.7)
                         } else {
                             (x + push_distance, y - push_distance * 0.7) // Fallback
                         }
                     },
                     2 => (x + push_distance, y + push_distance * 0.7 - 40.0), // Design
                     3 => (x + push_distance * 0.7, y + push_distance * 0.7),  // Synthesis
                     4 => {
                         // Application - mirror synthesis position (index 3)
                         if points.len() > 3 {
                             let synthesis_x = points[3].0 + push_distance * 0.7;
                             let synthesis_distance_from_center = synthesis_x - center_x;
                             let application_x = center_x - synthesis_distance_from_center;
                             (application_x, points[3].1 + push_distance * 0.7)
                         } else {
                             (x - push_distance * 0.7, y + push_distance * 0.7) // Fallback
                         }
                     },
                     5 => {
                         // Delivery - mirror design position (index 2)
                         if points.len() > 2 {
                             let design_x = points[2].0 + push_distance;
                             let design_distance_from_center = design_x - center_x;
                             let delivery_x = center_x - design_distance_from_center;
                             (delivery_x, points[2].1 + push_distance * 0.7 - 40.0)
                         } else {
                             (x - push_distance, y + push_distance * 0.7 - 40.0) // Fallback
                         }
                     },
                     6 => (x - push_distance, y - push_distance * 0.7), // Value
                     _ => (x, y),
                 }
             },
            "octad" => {
                let push_distance = 60.0;
                let diagonal_push = 70.0;
                match index {
                    0 => (x + push_distance, y),                              // SSH (right)
                    1 => (x + diagonal_push * 0.7, y + diagonal_push * 0.7), // CF (bottom-right)
                    2 => (x, y + push_distance),                              // SP (bottom)
                    3 => (x - diagonal_push * 0.7, y + diagonal_push * 0.7), // NR (bottom-left)
                    4 => (x - push_distance, y),                              // IT (left)
                    5 => (x - diagonal_push * 0.7, y - diagonal_push * 0.7), // IV (top-left)
                    6 => (x, y - push_distance),                              // IN (top)
                    7 => (x + diagonal_push * 0.7, y - diagonal_push * 0.7), // OM (top-right)
                    _ => (x, y),
                }
            },
            _ => {
                // For ennead, decad, undecad, dodecad - use simple outward push
                let radius = svg_size * 0.05; // Small outward push
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

    // Use the framework-agnostic geometry calculator
    fn get_system_layout(&self, system_type: &str, svg_size: f64) -> Vec<(f64, f64)> {
        let center = svg_size / 2.0;
        let layout = GeometryCalculator::calculate_system_layout(system_type, center, center, svg_size);
        layout.nodes.into_iter().map(|point| (point.x, point.y)).collect()
    }

    // Temporary method for backward compatibility - will be removed as we migrate all systems
    fn regular_polygon_points(&self, n: usize, cx: f64, cy: f64, radius: f64, rotation: f64) -> Vec<(f64, f64)> {
        (0..n).map(|i| {
            let angle = 2.0 * PI * i as f64 / n as f64 + rotation;
            let x = cx + radius * angle.cos();
            let y = cy + radius * angle.sin();
            (x, y)
        }).collect()
    }

    // Convert SVG coordinates to CSS percentages
    fn svg_to_css_percent(&self, coord: f64, svg_size: f64) -> String {
        format!("{}%", (coord / svg_size) * 100.0)
    }

    fn render_point(&self, term: &str, top: &str, left: &str) -> Html {
        self.render_point_with_positioning(term, top, left, false)
    }
    
    fn render_point_with_positioning(&self, term: &str, top: &str, left: &str, is_octad: bool) -> Html {
        if is_octad {
            // Handle special positioning for certain octad terms
        let (formatted_term, css_class, container_style) = match term {
            "Smallest Significant Holon" => {
                // Move anchor point 35px left, then anchor at left edge and extend right
                let adjusted_left = format!("{}%", left.trim_end_matches('%').parse::<f64>().unwrap_or(0.0) - 8.75); // ~35px adjustment
                (html! { {term} }, "point-label", format!("top: {}; left: {}; transform: translate(0%, -50%);", top, adjusted_left))
            },
            "Integrative Totality" => {
                // Move anchor point 35px right, then anchor at right edge and extend left
                let adjusted_left = format!("{}%", left.trim_end_matches('%').parse::<f64>().unwrap_or(0.0) + 8.75); // ~35px adjustment
                (html! { {term} }, "point-label", format!("top: {}; left: {}; transform: translate(-100%, -50%);", top, adjusted_left))
            },
            _ => {
                // Default center positioning
                (html! { {term} }, "point-label", format!("top: {}; left: {}; transform: translate(-50%, -50%);", top, left))
            }
        };
        
        html! {
            <div class="point-container" style={container_style}>
                <div class={css_class}>{formatted_term}</div>
                <input class="point-input" placeholder="Instance" />
            </div>
        }
        } else {
            // Standard rendering
        html! {
                <div class="point-container" style={format!("top: {}; left: {}; transform: translate(-50%, -50%);", top, left)}>
                    <div class="point-label">{term}</div>
                    <input class="point-input" placeholder="Instance" />
            </div>
            }
        }
    }

    fn render_monad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("monad", 1)
    }

    fn render_dyad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("dyad", 2)
    }

    fn render_triad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("triad", 3)
    }

    fn render_tetrad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("tetrad", 4)
    }

    fn render_pentad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("pentad", 5)
    }

    fn render_hexad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("hexad", 6)
    }

    fn render_heptad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("heptad", 7)
    }

    fn render_octad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("octad", 8)
    }

    fn render_ennead(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("ennead", 9)
    }

    fn render_decad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("decad", 10)
    }

    fn render_undecad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("undecad", 11)
    }

    fn render_dodecad(&self, _structure: &Option<StoredStructure>) -> Html {
        self.render_system_with_terms("dodecad", 12)
    }


} 