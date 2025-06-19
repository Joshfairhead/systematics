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
        
        let svg_size = 400.0;
        let center = svg_size / 2.0;
        
        html! {
            <div class="system-overlay">
                {self.render_point(&term, &self.svg_to_css_percent(center, svg_size), &self.svg_to_css_percent(center, svg_size))}
            </div>
        }
    }

    fn render_dyad(&self, _structure: &Option<StoredStructure>) -> Html {
        // API canonical order: [Action, Reaction]
        // Map API indices to visual positions:
        let term1 = self.get_canonical_term(0, "Action");     // Index 0 → Left
        let term2 = self.get_canonical_term(1, "Reaction");   // Index 1 → Right
        
        let svg_size = 400.0;
        let points = self.get_system_layout("dyad", svg_size);
        
        // Custom positioning for dyad labels - centered ON nodes
        let action_top = self.svg_to_css_percent(points[0].1, svg_size);
        let action_left = self.svg_to_css_percent(points[0].0, svg_size);
        
        let reaction_top = self.svg_to_css_percent(points[1].1, svg_size);
        let reaction_left = self.svg_to_css_percent(points[1].0, svg_size);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&term1, &action_top, &action_left)}
                {self.render_point(&term2, &reaction_top, &reaction_left)}
            </div>
        }
    }

    fn render_triad(&self, _structure: &Option<StoredStructure>) -> Html {
        // API canonical order: [Will, Being, Function]
        // Map API indices to visual positions:
        let term1 = self.get_canonical_term(0, "Will");       // Index 0 → Top-left
        let term2 = self.get_canonical_term(1, "Being");      // Index 1 → Right
        let term3 = self.get_canonical_term(2, "Function");   // Index 2 → Bottom-left
        
        let svg_size = 400.0;
        let points = self.get_system_layout("triad", svg_size);
        
        // Custom positioning for labels - fine-tuned positioning
        // Will (top-left) - pull closer to node
        let will_top = self.svg_to_css_percent(points[0].1 - 45.0, svg_size);
        let will_left = self.svg_to_css_percent(points[0].0, svg_size);
        
        // Being (right) - push further to the right
        let being_top = self.svg_to_css_percent(points[1].1, svg_size);
        let being_left = self.svg_to_css_percent(points[1].0 + 60.0, svg_size);
        
        // Function (bottom-left) - pull closer to node
        let function_top = self.svg_to_css_percent(points[2].1 + 45.0, svg_size);
        let function_left = self.svg_to_css_percent(points[2].0, svg_size);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&term1, &will_top, &will_left)}
                {self.render_point(&term2, &being_top, &being_left)}
                {self.render_point(&term3, &function_top, &function_left)}
            </div>
        }
    }

    fn render_tetrad(&self, _structure: &Option<StoredStructure>) -> Html {
        // API canonical order: [Ideal, Directive, Instrumental, Ground]
        // Map to visual diamond positions:
        let ideal = self.get_canonical_term(0, "Ideal");              // Index 0 → Top
        let directive = self.get_canonical_term(1, "Directive");      // Index 1 → Right
        let instrumental = self.get_canonical_term(2, "Instrumental"); // Index 2 → Left
        let ground = self.get_canonical_term(3, "Ground");            // Index 3 → Bottom
        
        let svg_size = 400.0;
        let points = self.get_system_layout("tetrad", svg_size);
        
        // Custom positioning for labels - fine-tuned positioning
        // Ideal (top) - keep current position (good as is)
        let ideal_top = self.svg_to_css_percent(points[0].1 - 50.0, svg_size);
        let ideal_left = self.svg_to_css_percent(points[0].0, svg_size);
        
        // Directive (right) - push further right on horizontal axis
        let directive_top = self.svg_to_css_percent(points[1].1, svg_size);
        let directive_left = self.svg_to_css_percent(points[1].0 + 60.0, svg_size);
        
        // Instrumental (left) - push further left on horizontal axis
        let instrumental_top = self.svg_to_css_percent(points[2].1, svg_size);
        let instrumental_left = self.svg_to_css_percent(points[2].0 - 65.0, svg_size);
        
        // Ground (bottom) - keep current position (good as is)
        let ground_top = self.svg_to_css_percent(points[3].1 + 50.0, svg_size);
        let ground_left = self.svg_to_css_percent(points[3].0, svg_size);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&ideal, &ideal_top, &ideal_left)}
                {self.render_point(&directive, &directive_top, &directive_left)}
                {self.render_point(&instrumental, &instrumental_top, &instrumental_left)}
                {self.render_point(&ground, &ground_top, &ground_left)}
            </div>
        }
    }

    fn render_pentad(&self, _structure: &Option<StoredStructure>) -> Html {
        // API canonical order: [Purpose, Higher Potential, Quintessence, Lower Potential, Source]
        // Map to our three-column layout:
        let purpose = self.get_canonical_term(0, "Purpose");                    // Index 0 → Top-right
        let higher_potential = self.get_canonical_term(1, "Higher Potential");  // Index 1 → Top-middle
        let quintessence = self.get_canonical_term(2, "Quintessence");          // Index 2 → Left center
        let lower_potential = self.get_canonical_term(3, "Lower Potential");    // Index 3 → Bottom-middle
        let source = self.get_canonical_term(4, "Source");                      // Index 4 → Bottom-right
        
        let svg_size = 400.0;
        let points = self.get_system_layout("pentad", svg_size);
        
        // Adjust label positions away from nodes
        // Purpose: above top-right node
        let purpose_top = self.svg_to_css_percent(points[0].1 - 45.0, svg_size);
        let purpose_left = self.svg_to_css_percent(points[0].0, svg_size);
        
        // Higher Potential: above top-middle node
        let higher_potential_top = self.svg_to_css_percent(points[1].1 - 45.0, svg_size);
        let higher_potential_left = self.svg_to_css_percent(points[1].0, svg_size);
        
        // Quintessence: left of left-center node
        let quintessence_top = self.svg_to_css_percent(points[2].1, svg_size);
        let quintessence_left = self.svg_to_css_percent(points[2].0 - 65.0, svg_size);
        
        // Lower Potential: below bottom-middle node
        let lower_potential_top = self.svg_to_css_percent(points[3].1 + 45.0, svg_size);
        let lower_potential_left = self.svg_to_css_percent(points[3].0, svg_size);
        
        // Source: below bottom-right node
        let source_top = self.svg_to_css_percent(points[4].1 + 45.0, svg_size);
        let source_left = self.svg_to_css_percent(points[4].0, svg_size);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&purpose, &purpose_top, &purpose_left)}
                {self.render_point(&higher_potential, &higher_potential_top, &higher_potential_left)}
                {self.render_point(&quintessence, &quintessence_top, &quintessence_left)}
                {self.render_point(&lower_potential, &lower_potential_top, &lower_potential_left)}
                {self.render_point(&source, &source_top, &source_left)}
            </div>
        }
    }

    fn render_hexad(&self, structure: &Option<StoredStructure>) -> Html {
        // API canonical order: [Resources, Values, Options, Criteria, Facts, Priorities]
        let resources = self.get_canonical_term(0, "Resources");       // Index 0
        let values = self.get_canonical_term(1, "Values");             // Index 1
        let options = self.get_canonical_term(2, "Options");           // Index 2
        let criteria = self.get_canonical_term(3, "Criteria");         // Index 3
        let facts = self.get_canonical_term(4, "Facts");               // Index 4
        let priorities = self.get_canonical_term(5, "Priorities");     // Index 5
        
        let svg_size = 400.0;
        let points = self.get_system_layout("hexad", svg_size);
        
        // Push labels outward from nodes
        let push_distance = 45.0;  // Reduced from 90.0 to bring labels closer
        let diagonal_push = 60.0;  // Slightly more push for diagonal positions
        
        // Updated mapping to match new geometry rotation:
        // Position 0: top-left → Resources
        let resources_top = self.svg_to_css_percent(points[0].1 - diagonal_push * 0.7, svg_size);
        let resources_left = self.svg_to_css_percent(points[0].0 - diagonal_push * 0.7, svg_size);
        
        // Position 1: top → Values  
        let values_top = self.svg_to_css_percent(points[1].1 - push_distance, svg_size);
        let values_left = self.svg_to_css_percent(points[1].0, svg_size);
        
        // Position 2: top-right → Options
        let options_top = self.svg_to_css_percent(points[2].1 - diagonal_push * 0.7, svg_size);
        let options_left = self.svg_to_css_percent(points[2].0 + diagonal_push * 0.7, svg_size);
        
        // Position 3: bottom-right → Criteria
        let criteria_top = self.svg_to_css_percent(points[3].1 + diagonal_push * 0.7, svg_size);
        let criteria_left = self.svg_to_css_percent(points[3].0 + diagonal_push * 0.7, svg_size);
        
        // Position 4: bottom → Facts
        let facts_top = self.svg_to_css_percent(points[4].1 + push_distance, svg_size);
        let facts_left = self.svg_to_css_percent(points[4].0, svg_size);
        
        // Position 5: bottom-left → Priorities
        let priorities_top = self.svg_to_css_percent(points[5].1 + diagonal_push * 0.7, svg_size);
        let priorities_left = self.svg_to_css_percent(points[5].0 - diagonal_push * 0.7, svg_size);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&resources, &resources_top, &resources_left)}
                {self.render_point(&values, &values_top, &values_left)}
                {self.render_point(&options, &options_top, &options_left)}
                {self.render_point(&criteria, &criteria_top, &criteria_left)}
                {self.render_point(&facts, &facts_top, &facts_left)}
                {self.render_point(&priorities, &priorities_top, &priorities_left)}
            </div>
        }
    }

    fn render_heptad(&self, structure: &Option<StoredStructure>) -> Html {
        // API canonical order: [Insight, Research, Design, Synthesis, Application, Delivery, Value]
        let insight = self.get_canonical_term(0, "Insight");           // Index 0 → Top
        let research = self.get_canonical_term(1, "Research");         // Index 1 → Clockwise
        let design = self.get_canonical_term(2, "Design");             // Index 2 → Clockwise
        let synthesis = self.get_canonical_term(3, "Synthesis");       // Index 3 → Clockwise
        let application = self.get_canonical_term(4, "Application");   // Index 4 → Clockwise
        let delivery = self.get_canonical_term(5, "Delivery");         // Index 5 → Clockwise
        let value = self.get_canonical_term(6, "Value");               // Index 6 → Clockwise
        
        let svg_size = 400.0;
        let points = self.get_system_layout("heptad", svg_size);
        
        // Push labels outward from nodes
        let push_distance = 60.0;  // Outward positioning for all labels
        
        // Clean mapping: API index → geometry position with outward push
        let insight_top = self.svg_to_css_percent(points[0].1 - push_distance + 15.0, svg_size); // Lowered by 15px total
        let insight_left = self.svg_to_css_percent(points[0].0, svg_size);
        
        // Calculate center x position for symmetry
        let center_x = svg_size / 2.0;
        
        // Calculate value's distance from center, apply reverse to research
        let value_x = points[6].0 - push_distance;
        let value_distance_from_center = center_x - value_x; // Distance left of center
        let research_x = center_x + value_distance_from_center; // Mirror to right of center
        
        let research_top = self.svg_to_css_percent(points[1].1 - push_distance * 0.7, svg_size);
        let research_left = self.svg_to_css_percent(research_x, svg_size);
        
        // Calculate design's distance from center, invert for delivery
        let design_x = points[2].0 + push_distance;
        let design_distance_from_center = design_x - center_x; // Distance right of center
        let delivery_x = center_x - design_distance_from_center; // Mirror to left of center
        
        // Align design with delivery on y-axis
        let design_top = self.svg_to_css_percent(points[5].1 + push_distance * 0.7 - 40.0, svg_size); // Raised by 40px (was 49px, lowered by 9px)
        let design_left = self.svg_to_css_percent(design_x, svg_size);
        
        // Calculate synthesis's distance from center, invert for application
        let synthesis_x = points[3].0 + push_distance * 0.7;
        let synthesis_distance_from_center = synthesis_x - center_x; // Distance right of center
        let application_x = center_x - synthesis_distance_from_center; // Mirror to left of center
        
        let synthesis_top = self.svg_to_css_percent(points[3].1 + push_distance * 0.7, svg_size);
        let synthesis_left = self.svg_to_css_percent(synthesis_x, svg_size);
        
        // Align application with synthesis on y-axis
        let application_top = self.svg_to_css_percent(points[3].1 + push_distance * 0.7, svg_size); // Same y as synthesis
        let application_left = self.svg_to_css_percent(application_x, svg_size);
        
        let delivery_top = self.svg_to_css_percent(points[5].1 + push_distance * 0.7 - 40.0, svg_size); // Raised by 40px (was 49px, lowered by 9px)
        let delivery_left = self.svg_to_css_percent(delivery_x, svg_size);
        
        // Align value with research on y-axis
        let value_top = self.svg_to_css_percent(points[1].1 - push_distance * 0.7, svg_size); // Same y as research
        let value_left = self.svg_to_css_percent(value_x, svg_size);
        
        html! {
            <div class="system-overlay">
                {self.render_point(&insight, &insight_top, &insight_left)}
                {self.render_point(&research, &research_top, &research_left)}
                {self.render_point(&design, &design_top, &design_left)}
                {self.render_point(&synthesis, &synthesis_top, &synthesis_left)}
                {self.render_point(&application, &application_top, &application_left)}
                {self.render_point(&delivery, &delivery_top, &delivery_left)}
                {self.render_point(&value, &value_top, &value_left)}
            </div>
        }
    }

    fn render_octad_point(&self, term: &str, top: &str, left: &str) -> Html {
        // Handle special positioning for certain terms
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
    }

    fn render_octad(&self, structure: &Option<StoredStructure>) -> Html {
        // API canonical order: [Smallest Significant Holon, Critical Functions, Supportive Platform, Necessary Resourcing, Integrative Totality, Inherent Values, Intrinsic Nature, Organisational Modes]
        let smallest_significant_holon = self.get_canonical_term(0, "Smallest Significant Holon");   // Index 0
        let critical_functions = self.get_canonical_term(1, "Critical Functions");                   // Index 1
        let supportive_platform = self.get_canonical_term(2, "Supportive Platform");                // Index 2
        let necessary_resourcing = self.get_canonical_term(3, "Necessary Resourcing");              // Index 3
        let integrative_totality = self.get_canonical_term(4, "Integrative Totality");              // Index 4
        let inherent_values = self.get_canonical_term(5, "Inherent Values");                        // Index 5
        let intrinsic_nature = self.get_canonical_term(6, "Intrinsic Nature");                      // Index 6
        let organisational_modes = self.get_canonical_term(7, "Organisational Modes");             // Index 7

        let svg_size = 400.0;
        let points = self.get_system_layout("octad", svg_size);
        
        // Push labels outward from nodes
        let push_distance = 60.0;  // Outward positioning for all labels
        let diagonal_push = 70.0;  // Extra push for diagonal positions
        
        // Clean mapping: API index → geometry position with outward push
        // Compensating for geometry rotation: Position 0 is now at right, Position 6 at top
        
        // Position 0: right → Smallest Significant Holon
        let ssh_top = self.svg_to_css_percent(points[0].1, svg_size);
        let ssh_left = self.svg_to_css_percent(points[0].0 + push_distance, svg_size);
        
        // Position 1: bottom-right → Critical Functions  
        let cf_top = self.svg_to_css_percent(points[1].1 + diagonal_push * 0.7, svg_size);
        let cf_left = self.svg_to_css_percent(points[1].0 + diagonal_push * 0.7, svg_size);
        
        // Position 2: bottom → Supportive Platform
        let sp_top = self.svg_to_css_percent(points[2].1 + push_distance, svg_size);
        let sp_left = self.svg_to_css_percent(points[2].0, svg_size);
        
        // Position 3: bottom-left → Necessary Resourcing
        let nr_top = self.svg_to_css_percent(points[3].1 + diagonal_push * 0.7, svg_size); // Back to 0.7 to match other diagonals
        let nr_left = self.svg_to_css_percent(points[3].0 - diagonal_push * 0.7, svg_size); // Back to 0.7 to match other diagonals
        
        // Position 4: left → Integrative Totality
        let it_top = self.svg_to_css_percent(points[4].1, svg_size);
        let it_left = self.svg_to_css_percent(points[4].0 - push_distance, svg_size);
        
        // Position 5: top-left → Inherent Values
        let iv_top = self.svg_to_css_percent(points[5].1 - diagonal_push * 0.7, svg_size);
        let iv_left = self.svg_to_css_percent(points[5].0 - diagonal_push * 0.7, svg_size);
        
        // Position 6: top → Intrinsic Nature
        let in_top = self.svg_to_css_percent(points[6].1 - push_distance, svg_size);
        let in_left = self.svg_to_css_percent(points[6].0, svg_size);
        
        // Position 7: top-right → Organisational Modes
        let om_top = self.svg_to_css_percent(points[7].1 - diagonal_push * 0.7, svg_size);
        let om_left = self.svg_to_css_percent(points[7].0 + diagonal_push * 0.7, svg_size);

        html! {
            <div class="system-overlay">
                {self.render_octad_point(&smallest_significant_holon, &ssh_top, &ssh_left)}
                {self.render_octad_point(&critical_functions, &cf_top, &cf_left)}
                {self.render_octad_point(&supportive_platform, &sp_top, &sp_left)}
                {self.render_octad_point(&necessary_resourcing, &nr_top, &nr_left)}
                {self.render_octad_point(&integrative_totality, &it_top, &it_left)}
                {self.render_octad_point(&inherent_values, &iv_top, &iv_left)}
                {self.render_octad_point(&intrinsic_nature, &in_top, &in_left)}
                {self.render_octad_point(&organisational_modes, &om_top, &om_left)}
            </div>
        }
    }

    fn render_ennead(&self, structure: &Option<StoredStructure>) -> Html {
        let terms: Vec<String> = (0..9)
            .map(|i| self.get_canonical_term(i, &format!("Term {}", i + 1)))
            .collect();
        
        let svg_size = 400.0;
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
        
        let svg_size = 400.0;
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
        
        let svg_size = 400.0;
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
        
        let svg_size = 400.0;
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