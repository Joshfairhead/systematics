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

    fn get_canonical_term(&self, position: usize, fallback: &str) -> String {
        self.schema
            .as_ref()
            .and_then(|s| s.canonical_terms.get(position))
            .cloned()
            .unwrap_or_else(|| fallback.to_string())
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
        html! {
            <div class="point-container" style={format!("top: {}; left: {}; transform: translate(-50%, -50%);", top, left)}>
                <div class="point-label">{term}</div>
                <input class="point-input" placeholder="Instance" />
            </div>
        }
    }

    fn render_monad(&self, _structure: &Option<StoredStructure>) -> Html {
        let term = self.get_canonical_term(0, "Unity");
        
        html! {
            <div class="system-overlay">
                {self.render_point(&term, "50%", "50%")}
            </div>
        }
    }

    fn render_dyad(&self, _structure: &Option<StoredStructure>) -> Html {
        let term1 = self.get_canonical_term(0, "Essence");
        let term2 = self.get_canonical_term(1, "Existence");
        
        // Match the geometric renderer's dyad positioning
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let offset = svg_size * 0.15;
        
        let left1 = self.svg_to_css_percent(center - offset, svg_size);
        let left2 = self.svg_to_css_percent(center + offset, svg_size);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&term1, "50%", &left1)}
                {self.render_point(&term2, "50%", &left2)}
            </div>
        }
    }

    fn render_triad(&self, _structure: &Option<StoredStructure>) -> Html {
        // Use canonical terms from API instead of hardcoded values
        let term1 = self.get_canonical_term(0, "Will");      // Index 0: canonical first term
        let term2 = self.get_canonical_term(1, "Being");     // Index 1: canonical second term  
        let term3 = self.get_canonical_term(2, "Function");  // Index 2: canonical third term
        
        let svg_size = 500.0;
        let points = self.get_system_layout("triad", svg_size);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&term1, &self.svg_to_css_percent(points[0].1, svg_size), &self.svg_to_css_percent(points[0].0, svg_size))}
                {self.render_point(&term2, &self.svg_to_css_percent(points[1].1, svg_size), &self.svg_to_css_percent(points[1].0, svg_size))}
                {self.render_point(&term3, &self.svg_to_css_percent(points[2].1, svg_size), &self.svg_to_css_percent(points[2].0, svg_size))}
            </div>
        }
    }

    fn render_tetrad(&self, structure: &Option<StoredStructure>) -> Html {
        let term1 = self.get_canonical_term(0, "Ground");
        let term2 = self.get_canonical_term(1, "Ideal");
        let term3 = self.get_canonical_term(2, "Instrumental");
        let term4 = self.get_canonical_term(3, "Directive");
        
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.15;
        let points = self.regular_polygon_points(4, center, center, radius, PI/4.0);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&term1, &self.svg_to_css_percent(points[0].1, svg_size), &self.svg_to_css_percent(points[0].0, svg_size))}
                {self.render_point(&term2, &self.svg_to_css_percent(points[1].1, svg_size), &self.svg_to_css_percent(points[1].0, svg_size))}
                {self.render_point(&term3, &self.svg_to_css_percent(points[2].1, svg_size), &self.svg_to_css_percent(points[2].0, svg_size))}
                {self.render_point(&term4, &self.svg_to_css_percent(points[3].1, svg_size), &self.svg_to_css_percent(points[3].0, svg_size))}
            </div>
        }
    }

    fn render_pentad(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..5)
            .map(|i| self.get_canonical_term(i, &format!("Term {}", i + 1)))
            .collect();
        
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.15;
        let points = self.regular_polygon_points(5, center, center, radius, -PI/2.0);
        
        html! {
            <div class="system-overlay">
                {for points.iter().enumerate().map(|(i, (x, y))| {
                    self.render_point(&terms[i], &self.svg_to_css_percent(*y, svg_size), &self.svg_to_css_percent(*x, svg_size))
                })}
            </div>
        }
    }

    fn render_hexad(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..6)
            .map(|i| self.get_canonical_term(i, &format!("Term {}", i + 1)))
            .collect();
        
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.15;
        let points = self.regular_polygon_points(6, center, center, radius, 0.0);
        
        html! {
            <div class="system-overlay">
                {for points.iter().enumerate().map(|(i, (x, y))| {
                    self.render_point(&terms[i], &self.svg_to_css_percent(*y, svg_size), &self.svg_to_css_percent(*x, svg_size))
                })}
            </div>
        }
    }

    fn render_heptad(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..7)
            .map(|i| self.get_canonical_term(i, &format!("Term {}", i + 1)))
            .collect();
        
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.18;
        let points = self.regular_polygon_points(7, center, center, radius, -PI/2.0);
        
        html! {
            <div class="system-overlay">
                {for points.iter().enumerate().map(|(i, (x, y))| {
                    self.render_point(&terms[i], &self.svg_to_css_percent(*y, svg_size), &self.svg_to_css_percent(*x, svg_size))
                })}
            </div>
        }
    }

    fn render_octad(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..8)
            .map(|i| self.get_canonical_term(i, &format!("Element {}", i + 1)))
            .collect();

        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.18;
        let points = self.regular_polygon_points(8, center, center, radius, PI/8.0);

        html! {
            <div class="system-overlay">
                {for points.iter().enumerate().map(|(i, (x, y))| {
                    self.render_point(&terms[i], &self.svg_to_css_percent(*y, svg_size), &self.svg_to_css_percent(*x, svg_size))
                })}
            </div>
        }
    }

    fn render_ennead(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..9)
            .map(|i| self.get_canonical_term(i, &format!("Term {}", i + 1)))
            .collect();
        
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.20;
        let points = self.regular_polygon_points(9, center, center, radius, -PI/2.0);
        
        html! {
            <div class="system-overlay">
                {for points.iter().enumerate().map(|(i, (x, y))| {
                    self.render_point(&terms[i], &self.svg_to_css_percent(*y, svg_size), &self.svg_to_css_percent(*x, svg_size))
                })}
            </div>
        }
    }

    fn render_decad(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..10)
            .map(|i| self.get_canonical_term(i, &format!("Term {}", i + 1)))
            .collect();
        
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.20;
        let points = self.regular_polygon_points(10, center, center, radius, -PI/2.0);
        
        html! {
            <div class="system-overlay">
                {for points.iter().enumerate().map(|(i, (x, y))| {
                    self.render_point(&terms[i], &self.svg_to_css_percent(*y, svg_size), &self.svg_to_css_percent(*x, svg_size))
                })}
            </div>
        }
    }

    fn render_undecad(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..11)
            .map(|i| self.get_canonical_term(i, &format!("Term {}", i + 1)))
            .collect();
        
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.22;
        let points = self.regular_polygon_points(11, center, center, radius, -PI/2.0);
        
        html! {
            <div class="system-overlay">
                {for points.iter().enumerate().map(|(i, (x, y))| {
                    self.render_point(&terms[i], &self.svg_to_css_percent(*y, svg_size), &self.svg_to_css_percent(*x, svg_size))
                })}
            </div>
        }
    }

    fn render_dodecad(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..12)
            .map(|i| self.get_canonical_term(i, &format!("Term {}", i + 1)))
            .collect();
        
        let svg_size = 500.0;
        let center = svg_size / 2.0;
        let radius = svg_size * 0.22;
        let points = self.regular_polygon_points(12, center, center, radius, -PI/2.0);
        
        html! {
            <div class="system-overlay">
                {for points.iter().enumerate().map(|(i, (x, y))| {
                    self.render_point(&terms[i], &self.svg_to_css_percent(*y, svg_size), &self.svg_to_css_percent(*x, svg_size))
                })}
            </div>
        }
    }
} 