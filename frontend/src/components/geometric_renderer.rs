use yew::{html, Component, Context, Html, Properties, Callback, TargetCast};
use web_sys::{HtmlInputElement, InputEvent};
use crate::core::geometry::{GeometryCalculator, Point, Edge, SymbolicCircle, SymbolicTriangle};
use crate::services::api::ConnectiveInfo;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub system_type: String,
    pub size: f64,
    #[prop_or_default]
    pub connectives: Option<Vec<ConnectiveInfo>>,
    #[prop_or_default]
    pub labels: Option<Vec<String>>,
    #[prop_or_default]
    pub creation_mode: bool,
    #[prop_or_default]
    pub user_inputs: Option<Vec<String>>,
    #[prop_or_default]
    pub on_input_change: Option<Callback<(usize, String)>>,
}

pub struct GeometricRenderer;

impl Component for GeometricRenderer {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let size = props.size;
        let system_type = &props.system_type;
        
        // Extended padding to accommodate wide label positioning (especially octad)
        let padding_x = 200.0; // Wide horizontal padding for extended labels
        let padding_y = 80.0;  // Standard vertical padding
        let total_width = size + 2.0 * padding_x;
        let total_height = size + 2.0 * padding_y;
        let viewbox = format!("{} {} {} {}", -padding_x, -padding_y, total_width, total_height);
        let center_x = size / 2.0;
        let center_y = size / 2.0;
        
        // Use simple structural layout - edges are just connections
        let layout = GeometryCalculator::calculate_system_layout(
            system_type,
            center_x,
            center_y,
            size,
        );
        
        html! {
            <div class="geometric-renderer-container">
                <svg 
                    class="geometric-renderer" 
                    width="100%"
                    height="100%"
                    viewBox={viewbox}
                    xmlns="http://www.w3.org/2000/svg"
                >
                    {self.render_symbolic_elements(&layout)}
                    {self.render_edges_with_connectives(&layout.edges, &layout.nodes, &props.connectives, system_type)}
                    {self.render_nodes(&layout.nodes, layout.node_radius)}
                    {self.render_labels(&layout.nodes, &props.labels, size, system_type)}
                </svg>
                
                // Render input fields as HTML overlays positioned to align with SVG labels
                {if props.creation_mode {
                    self.render_overlay_input_fields(ctx, &layout.nodes, &props.labels, &props.user_inputs, size, padding_x)
                } else {
                    html! {}
                }}
            </div>
        }
    }
}

impl GeometricRenderer {
    /// Calculate precise label position for a node - shared logic for labels and input fields
    fn calculate_label_position(&self, node: &Point, svg_center_x: f64, svg_center_y: f64) -> (f64, f64) {
        let dx = node.x - svg_center_x;
        let dy = node.y - svg_center_y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 5.0 {
            // Push label outward from center by 35 pixels
            let push_distance = 0.0;
            let unit_x = dx / distance;
            let unit_y = dy / distance;
            (
                node.x + unit_x * push_distance,
                node.y + unit_y * push_distance
            )
        } else {
            // For center nodes, place label above
            (node.x, node.y - 35.0)
        }
    }

    /// System-specific label positioning - allows custom positioning per system and node
    fn calculate_system_specific_label_position(&self, node: &Point, svg_center_x: f64, svg_center_y: f64, system_type: &str, node_index: usize) -> (f64, f64) {
        match system_type {
            "monad" => {
                // Monad: single center point - position label below
                (node.x, node.y - 35.0)
            },
            "dyad" => {
                // Dyad: two points horizontally - position labels above/below
                match node_index {
                    0 => (node.x - 0.0, node.y - 35.0), // Left point: left-up
                    1 => (node.x + 0.0, node.y - 35.0), // Right point: right-down
                    _ => (node.x, node.y)
                }
            },
            "triad" => {
                // Triad: center labels on nodes initially
                match node_index {
                    0 => (node.x, node.y - 40.0), // Will (top-left): centered
                    1 => (node.x +70.0, node.y), // Being (right): centered
                    2 => (node.x, node.y +40.0), // Function (bottom-left): centered
                    _ => (node.x, node.y)
                }
            },
            "tetrad" => {
                // Tetrad: cross formation - move labels outward from center
                match node_index {
                    0 => (node.x, node.y - 40.0), // Source: left-up
                    1 => (node.x + 90.0, node.y), // Directive: right-up
                    2 => (node.x - 100.0, node.y), // Instrumental: left-down
                    3 => (node.x, node.y + 40.0), // Ground: right-down
                    _ => (node.x, node.y)
                }
            },
            "pentad" => {
                // Pentad: hierarchical layout - position labels outside structure
                match node_index {
                    0 => (node.x + 0.0, node.y - 35.0), // Purpose: move right and up
                    1 => (node.x, node.y - 40.0), // Higher Potential: move left and up  
                    2 => (node.x - 100.0, node.y),        // Quintessence: move left
                    3 => (node.x, node.y + 40.0), // Lower Potential: move left and down
                    4 => (node.x + 0.0, node.y + 35.0), // Source: move right and down
                    _ => (node.x, node.y)
                }
            },
            "hexad" => {
                // Hexad: center labels on nodes initially
                match node_index {
                    0 => (node.x - 70.0, node.y - 35.0), // Resources (top-left): centered
                    1 => (node.x, node.y - 35.0), // Values (top): centered
                    2 => (node.x +70.0, node.y - 35.0), // Options (top-right): centered
                    3 => (node.x +70.0, node.y + 35.0), // Criteria (bottom-right): centered
                    4 => (node.x, node.y + 35.0), // Facts (bottom): centered
                    5 => (node.x - 70.0, node.y + 35.0), // Priorities (bottom-left): centered
                    _ => (node.x, node.y)
                }
            },
            "heptad" => {
                // Heptad: center labels on nodes initially
                match node_index {
                    0 => (node.x, node.y -35.0), // Insight (top): 
                    1 => (node.x + 70.0, node.y - 35.0), // Research (top-right): 
                    2 => (node.x + 70.0, node.y), // Design (right): 
                    3 => (node.x + 30.0, node.y + 35.0), // Synthesis (bottom-right): 
                    4 => (node.x - 30.0, node.y + 35.0), // Application (bottom-left): 
                    5 => (node.x - 80.0, node.y), // Delivery (left): 
                    6 => (node.x - 50.0, node.y - 35.0), // Value (top-left): 
                    _ => (node.x, node.y)
                }
            },
            "octad" => {
                // Octad: center labels on nodes initially
                match node_index {
                    0 => (node.x + 170.0, node.y), // Smallest Holon (top): 
                    1 => (node.x + 120.0, node.y + 30.0), // Critical Functions (top-right): 
                    2 => (node.x, node.y + 40.0), // Supportive Platform (right): 
                    3 => (node.x -130.0, node.y + 30.0), // Necessary Resourcing (bottom-right):
                    4 => (node.x - 140.0, node.y), // Integrative Totality (bottom): 
                    5 => (node.x - 100.0, node.y - 30.0), // Inherent Values (bottom-left): 
                    6 => (node.x, node.y - 40.0), // Intrinsic Nature (left):
                    7 => (node.x + 130.0, node.y - 30.0), // Organisational Modes (top-left): 
                    _ => (node.x, node.y)
                }
            },
            "ennead" => {
                // Ennead: 3x3 grid formation - position labels around grid
                match node_index {
                    0 => (node.x - 35.0, node.y - 25.0), // Top-left: left-up
                    1 => (node.x, node.y - 35.0),        // Top-center: up
                    2 => (node.x + 35.0, node.y - 25.0), // Top-right: right-up
                    3 => (node.x - 45.0, node.y),        // Middle-left: left
                    4 => (node.x, node.y),               // Center: no offset
                    5 => (node.x + 45.0, node.y),        // Middle-right: right
                    6 => (node.x - 35.0, node.y + 25.0), // Bottom-left: left-down
                    7 => (node.x, node.y + 35.0),        // Bottom-center: down
                    8 => (node.x + 35.0, node.y + 25.0), // Bottom-right: right-down
                    _ => (node.x, node.y)
                }
            },
            "decad" => {
                // Decad: complex 10-point structure - radial positioning
                match node_index {
                    0 => (node.x, node.y - 45.0),        // Top: up
                    1 => (node.x + 25.0, node.y - 35.0), // Top-right-1: right-up
                    2 => (node.x + 40.0, node.y - 15.0), // Right-1: right-up
                    3 => (node.x + 45.0, node.y + 10.0), // Right-2: right
                    4 => (node.x + 30.0, node.y + 35.0), // Bottom-right: right-down
                    5 => (node.x, node.y + 45.0),        // Bottom: down
                    6 => (node.x - 30.0, node.y + 35.0), // Bottom-left: left-down
                    7 => (node.x - 45.0, node.y + 10.0), // Left-2: left
                    8 => (node.x - 40.0, node.y - 15.0), // Left-1: left-up
                    9 => (node.x - 25.0, node.y - 35.0), // Top-left-1: left-up
                    _ => (node.x, node.y)
                }
            },
            "undecad" => {
                // Undecad: 11-point structure - radial positioning with tighter spacing
                match node_index {
                    0 => (node.x, node.y - 45.0),        // Top: up
                    1 => (node.x + 20.0, node.y - 40.0), // Top-right-1: right-up
                    2 => (node.x + 35.0, node.y - 25.0), // Top-right-2: right-up
                    3 => (node.x + 45.0, node.y - 5.0),  // Right-1: right
                    4 => (node.x + 40.0, node.y + 15.0), // Right-2: right-down
                    5 => (node.x + 25.0, node.y + 35.0), // Bottom-right: right-down
                    6 => (node.x, node.y + 45.0),        // Bottom: down
                    7 => (node.x - 25.0, node.y + 35.0), // Bottom-left: left-down
                    8 => (node.x - 40.0, node.y + 15.0), // Left-2: left-down
                    9 => (node.x - 45.0, node.y - 5.0),  // Left-1: left
                    10 => (node.x - 35.0, node.y - 25.0), // Top-left-2: left-up
                    _ => (node.x, node.y)
                }
            },
            "dodecad" => {
                // Dodecad: 12-point structure - full radial positioning
                match node_index {
                    0 => (node.x, node.y - 50.0),        // Autocracy (top): up
                    1 => (node.x + 25.0, node.y - 45.0), // Domination: right-up
                    2 => (node.x + 45.0, node.y - 25.0), // Creativity: right-up
                    3 => (node.x + 50.0, node.y),        // Pattern (right): right
                    4 => (node.x + 45.0, node.y + 25.0), // Individuality: right-down
                    5 => (node.x + 25.0, node.y + 45.0), // Structure: right-down
                    6 => (node.x, node.y + 50.0),        // Repetition (bottom): down
                    7 => (node.x - 25.0, node.y + 45.0), // Potentiality: left-down
                    8 => (node.x - 45.0, node.y + 25.0), // Subsistence: left-down
                    9 => (node.x - 50.0, node.y),        // Relatedness (left): left
                    10 => (node.x - 45.0, node.y - 25.0), // Polarity: left-up
                    11 => (node.x - 25.0, node.y - 45.0), // Wholeness: left-up
                    _ => (node.x, node.y)
                }
            },
            _ => {
                // Default: use generic positioning for all other systems
                self.calculate_label_position(node, svg_center_x, svg_center_y)
            }
        }
    }

    /// System-specific connective positioning - allows custom positioning per system and edge
    fn calculate_system_specific_connective_position(&self, from_node: &Point, to_node: &Point, system_type: &str, from_index: usize, to_index: usize) -> Vec<(String, f64, f64)> {
        match system_type {
            "tetrad" => {
                // Tetrad: custom positioning for each of the 6 connectives
                let mid_x = (from_node.x + to_node.x) / 2.0;
                let mid_y = (from_node.y + to_node.y) / 2.0;
                
                // Identify edge by node indices and apply custom offsets
                let (offset_x, offset_y) = match (from_index, to_index) {
                    (0, 1) | (1, 0) => (15.0, -15.0),   // Ideal-Directive: right-up
                    (0, 2) | (2, 0) => (-15.0, -15.0),  // Ideal-Instrumental: left-up  
                    (0, 3) | (3, 0) => (0.0, 0.0),      // Ideal-Ground: center (vertical axis)
                    (1, 2) | (2, 1) => (0.0, 0.0),      // Directive-Instrumental: center (horizontal axis)
                    (1, 3) | (3, 1) => (15.0, 15.0),    // Directive-Ground: right-down
                    (2, 3) | (3, 2) => (-15.0, 15.0),   // Instrumental-Ground: left-down
                    _ => (0.0, 0.0)
                };
                
                vec![("".to_string(), mid_x + offset_x, mid_y + offset_y)]
            },
            "pentad" => {
                // Pentad: custom positioning for each of the 10 connectives
                let mid_x = (from_node.x + to_node.x) / 2.0;
                let mid_y = (from_node.y + to_node.y) / 2.0;
                
                // Identify edge by node indices and apply custom offsets
                let (offset_x, offset_y) = match (from_index, to_index) {
                    (0, 1) | (1, 0) => (20.0, -20.0),   // Purpose-Higher Potential: right-up
                    (0, 2) | (2, 0) => (0.0, -10.0),    // Purpose-Quintessence: up
                    (0, 3) | (3, 0) => (20.0, 0.0),     // Purpose-Lower Potential: right
                    (0, 4) | (4, 0) => (20.0, 20.0),    // Purpose-Source: right-down
                    (1, 2) | (2, 1) => (-15.0, -10.0),  // Higher Potential-Quintessence: left-up
                    (1, 3) | (3, 1) => (0.0, 0.0),      // Higher Potential-Lower Potential: center
                    (1, 4) | (4, 1) => (15.0, 10.0),    // Higher Potential-Source: right-down
                    (2, 3) | (3, 2) => (-15.0, 10.0),   // Quintessence-Lower Potential: left-down
                    (2, 4) | (4, 2) => (0.0, 10.0),     // Quintessence-Source: down
                    (3, 4) | (4, 3) => (15.0, 20.0),    // Lower Potential-Source: right-down
                    _ => (0.0, 0.0)
                };
                
                vec![("".to_string(), mid_x + offset_x, mid_y + offset_y)]
            },
            _ => {
                // Default: use simple midpoint for all other systems
                let mid_x = (from_node.x + to_node.x) / 2.0;
                let mid_y = (from_node.y + to_node.y) / 2.0;
                vec![("".to_string(), mid_x, mid_y)]
            }
        }
    }

    /// Framework-specific rendering of symbolic circle (Yew HTML)
    /// Used for monad's outer circle representation
    fn render_symbolic_circle(&self, symbolic_circle: &Option<SymbolicCircle>) -> Html {
        match symbolic_circle {
            Some(circle) => html! {
                <circle 
                    cx={circle.center.x.to_string()} 
                    cy={circle.center.y.to_string()} 
                    r={circle.radius.to_string()} 
                    fill="none"
                    stroke="#667eea"
                    stroke-width="2"
                    opacity="0.6"
                />
            },
            None => html! {},
        }
    }

    /// Framework-specific rendering of multiple symbolic circles (Yew HTML)
    /// Used for dyad's vesica piscis representation
    fn render_symbolic_circles(&self, symbolic_circles: &[SymbolicCircle]) -> Html {
        if symbolic_circles.is_empty() {
            return html! {};
        }

        let circle_elements: Vec<Html> = symbolic_circles.iter().map(|circle| {
            html! {
                <circle 
                    cx={circle.center.x.to_string()} 
                    cy={circle.center.y.to_string()} 
                    r={circle.radius.to_string()} 
                    fill="none"
                    stroke="#667eea"
                    stroke-width="2"
                    opacity="0.6"
                />
            }
        }).collect();

        html! {
            <>
                {for circle_elements}
            </>
        }
    }

    /// Framework-specific rendering of edges (Yew HTML)
    /// Core logic is in GeometryCalculator (framework-agnostic)
    /// REPLACED BY: render_edges_with_connectives (extrinsic approach)
    
    /// Framework-specific rendering of nodes (Yew HTML)
    fn render_nodes(&self, nodes: &[Point], node_radius: f64) -> Html {
        let node_elements: Vec<Html> = nodes.iter().map(|node| {
            html! {
                <circle 
                    cx={node.x.to_string()} 
                    cy={node.y.to_string()} 
                    r={node_radius.to_string()} 
                    fill="#667eea"
                    stroke="#ffffff"
                    stroke-width="2"
                />
            }
        }).collect();

        html! {
            <>
                {for node_elements}
            </>
        }
    }

    /// Framework-specific rendering of labels positioned near nodes (Yew HTML)
    fn render_labels(&self, nodes: &[Point], labels: &Option<Vec<String>>, svg_size: f64, system_type: &str) -> Html {
        if let Some(label_texts) = labels {
            // Use dynamic SVG center based on actual size, not hardcoded values
            let svg_center_x = svg_size / 2.0; // Dynamic center
            let svg_center_y = svg_size / 2.0; // Dynamic center
            
            let label_elements: Vec<Html> = nodes.iter()
                .zip(label_texts.iter())
                .enumerate()
                .map(|(index, (node, label_text))| {
                    // System-specific label positioning
                    let (label_x, label_y) = self.calculate_system_specific_label_position(
                        node, svg_center_x, svg_center_y, system_type, index
                    );
                    
                    // More generous rectangle sizing based on text length
                    let char_width = 10.0; // Even more generous character width
                    let padding = 20.0; // More padding
                    let text_width = (label_text.len() as f64 * char_width + padding * 2.0).max(80.0);
                    let text_height = 35.0; // Even taller rectangles
                    let rect_x = label_x - (text_width / 2.0);
                    let rect_y = label_y - (text_height / 2.0);
                    
                    html! {
                        <g>
                            // Background rectangle for better readability
                            <rect 
                                x={rect_x.to_string()}
                                y={rect_y.to_string()}
                                width={text_width.to_string()}
                                height={text_height.to_string()}
                                fill="rgba(255, 255, 255, 0.95)"
                                stroke="#ddd"
                                stroke-width="1"
                                rx="6"
                                ry="6"
                            />
                            // Label text
                            <text 
                                x={label_x.to_string()}
                                y={label_y.to_string()}
                                text-anchor="middle"
                                dominant-baseline="middle"
                                font-size="16"
                                font-weight="bold"
                                fill="#333"
                            >
                                {label_text}
                            </text>
                        </g>
                    }
                })
                .collect();

            html! {
                <>
                    {for label_elements}
                </>
            }
        } else {
            html! {}
        }
    }
    


    /// Framework-specific rendering of symbolic triangle (Yew HTML)
    /// Used for triad's triangular representation
    fn render_symbolic_triangle(&self, symbolic_triangle: &Option<SymbolicTriangle>) -> Html {
        match symbolic_triangle {
            Some(triangle) => {
                let points = format!("{},{} {},{} {},{}",
                    triangle.vertices[0].x, triangle.vertices[0].y,
                    triangle.vertices[1].x, triangle.vertices[1].y,
                    triangle.vertices[2].x, triangle.vertices[2].y
                );
                
                html! {
                    <polygon 
                        points={points}
                        fill="none"
                        stroke="#667eea"
                        stroke-width="2"
                        opacity="0.6"
                    />
                }
            },
            None => html! {},
        }
    }

    /// Framework-specific rendering of symbolic elements (Yew HTML)
    /// Used for monad's outer circle representation and dyad's vesica piscis representation
    fn render_symbolic_elements(&self, layout: &crate::core::geometry::GraphLayout) -> Html {
        html! {
            <>
                {self.render_symbolic_circle(&layout.symbolic_circle)}
                {self.render_symbolic_circles(&layout.symbolic_circles)}
                {self.render_symbolic_triangle(&layout.symbolic_triangle)}
            </>
        }
    }

    /// Framework-specific rendering of edges with connectives (Yew HTML)
    /// Core logic is in GeometryCalculator (framework-agnostic)
    fn render_edges_with_connectives(&self, edges: &[Edge], nodes: &[Point], connectives: &Option<Vec<ConnectiveInfo>>, system_type: &str) -> Html {
        if edges.is_empty() {
            return html! {};
        }

        let edge_elements: Vec<Html> = edges.iter().map(|edge| {
            let from_node = &nodes[edge.from];
            let to_node = &nodes[edge.to];
            
            // Calculate edge angle for text alignment
            let dx = to_node.x - from_node.x;
            let dy = to_node.y - from_node.y;
            let angle_rad = dy.atan2(dx);
            let angle_deg = angle_rad * 180.0 / std::f64::consts::PI;
            
            // Adjust angle to keep text readable (not upside down)
            let text_angle = if angle_deg > 90.0 || angle_deg < -90.0 {
                angle_deg + 180.0
            } else {
                angle_deg
            };
            
            // Look up connective relationship for this edge (extrinsic approach)
            let relationship = if let Some(connectives) = connectives {
                connectives.iter()
                    .find(|c| (c.from_index == edge.from && c.to_index == edge.to) ||
                             (c.from_index == edge.to && c.to_index == edge.from))
                    .map(|c| c.relation_type.clone())
            } else {
                None
            };
            
            // Handle connective label positioning
            let render_labels = if let Some(relationship) = relationship {
                // TETRAD-SPECIFIC: Split multi-word connectives for axis edges to create gaps
                // Only apply this logic to tetrad (4 nodes) with its specific cross pattern
                if nodes.len() == 4 && (dx.abs() < 10.0 || dy.abs() < 10.0) {
                    // Tetrad axis edge - split words if multi-word
                    let words: Vec<&str> = relationship.split_whitespace().collect();
                    if words.len() >= 2 {
                        // Multi-word: split across the axis
                        let first_word = words[0].to_string();
                        let second_word = words[1..].join(" ");
                        
                        if dx.abs() < 10.0 {
                            // Vertical axis - first word at top, second word at bottom (reading order)
                            let center_x = (from_node.x + to_node.x) / 2.0;
                            let center_y = (from_node.y + to_node.y) / 2.0;
                            // Position words so gap is centered on crossing point
                            // Calculate actual text widths and create symmetric gap around center
                            let avg_char_width = 7.0; // Approximate width per character for 12px font
                            let first_word_width = first_word.len() as f64 * avg_char_width;
                            let second_word_width = second_word.len() as f64 * avg_char_width;
                            let gap_from_center = 15.0; // Distance from center to text edge
                            vec![
                                (first_word, center_x, center_y - gap_from_center - first_word_width / 2.0),
                                (second_word, center_x, center_y + gap_from_center + second_word_width / 2.0),
                            ]
                        } else {
                            // Horizontal axis - first word on left, second word on right (reading order)
                            let center_x = (from_node.x + to_node.x) / 2.0;
                            let center_y = (from_node.y + to_node.y) / 2.0;
                            
                            // Optical centering - adjust based on text length difference
                            let avg_char_width = 7.0;
                            let first_word_width = first_word.len() as f64 * avg_char_width;
                            let second_word_width = second_word.len() as f64 * avg_char_width;
                            let base_gap = 15.0;
                            
                            // Optical adjustment: longer text moves further, shorter text moves closer
                            let length_diff = first_word_width - second_word_width;
                            let optical_adjustment = length_diff * 0.15; // 15% compensation
                            
                            vec![
                                (first_word, center_x - base_gap - optical_adjustment - first_word_width / 2.0, center_y),
                                (second_word, center_x + base_gap - optical_adjustment + second_word_width / 2.0, center_y),
                            ]
                        }
                    } else {
                        // Single word - use midpoint
                        let mid_x = (from_node.x + to_node.x) / 2.0;
                        let mid_y = (from_node.y + to_node.y) / 2.0;
                        vec![(relationship, mid_x, mid_y)]
                    }
                                 } else if nodes.len() == 5 {
                     // PENTAD-SPECIFIC: Simple slide left/right along connective to prevent overlap
                     let mid_x = (from_node.x + to_node.x) / 2.0;
                     let mid_y = (from_node.y + to_node.y) / 2.0;
                     
                     // Calculate slide offset based on edge direction and index
                     let slide_distance = 20.0; // How far to slide along the line
                     let edge_vector_x = to_node.x - from_node.x;
                     let edge_vector_y = to_node.y - from_node.y;
                     let edge_length = (edge_vector_x * edge_vector_x + edge_vector_y * edge_vector_y).sqrt();
                     
                     // Normalize the edge vector
                     let unit_x = edge_vector_x / edge_length;
                     let unit_y = edge_vector_y / edge_length;
                     
                     // Slide along the edge based on which specific edge this is
                     let slide_factor = match (edge.from, edge.to) {
                         (0, 2) | (2, 0) => -0.3,  // Purpose-Quintessence: slide toward Purpose
                         (0, 4) | (4, 0) => 0.3,   // Purpose-Source: slide toward Source  
                         (1, 2) | (2, 1) => -0.2,  // Higher-Quintessence: slide toward Higher
                         (1, 4) | (4, 1) => 2.0,   // Higher-Source: slide toward Source
                         (3, 0) | (0, 3) => -2.0,  // Lower Potential-Purpose: slide toward Lower Potential
                         (2, 4) | (4, 2) => 0.0,   // Quintessence-Source: stay centered
                         _ => 0.0  // All other edges: stay centered
                     };
                     
                     let adjusted_x = mid_x + (unit_x * slide_distance * slide_factor);
                     let adjusted_y = mid_y + (unit_y * slide_distance * slide_factor);
                     
                     vec![(relationship, adjusted_x, adjusted_y)]
                 } else {
                     // ALL OTHER SYSTEMS: Use simple midpoint positioning for all edges
                     let mid_x = (from_node.x + to_node.x) / 2.0;
                     let mid_y = (from_node.y + to_node.y) / 2.0;
                     vec![(relationship, mid_x, mid_y)]
                 }
            } else {
                vec![]
            };
            
            html! {
                <>
                    // Render the edge line
                    <line 
                        x1={from_node.x.to_string()} 
                        y1={from_node.y.to_string()} 
                        x2={to_node.x.to_string()} 
                        y2={to_node.y.to_string()}
                        stroke="#667eea" 
                        stroke-width="2" 
                        opacity="0.5"
                    />
                    // Render the connective label(s)
                    {
                        for render_labels.into_iter().map(|(text, x, y)| {
                            // System-specific font styling
                            let (font_size, font_family, font_weight) = match system_type {
                                "tetrad" => ("16", "Roboto, sans-serif", "bold"),
                                "pentad" => ("16", "Roboto, sans-serif", "bold"),
                                _ => ("15", "Roboto, sans-serif", "bold")
                            };
                            
                            html! {
                                <text
                                    x={x.to_string()}
                                    y={y.to_string()}
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size={font_size}
                                    font-family={font_family}
                                    font-weight={font_weight}
                                    fill="#333"
                                    stroke="white"
                                    stroke-width="1"
                                    paint-order="stroke"
                                    transform={format!("rotate({} {} {})", text_angle, x, y)}
                                >
                                    {text}
                                </text>
                            }
                        })
                    }
                </>
            }
        }).collect();

        html! {
            <>
                {for edge_elements}
            </>
        }
    }



    /// Render positioned input fields as HTML overlays that align with SVG labels
    fn render_overlay_input_fields(&self, ctx: &Context<Self>, nodes: &[Point], labels: &Option<Vec<String>>, user_inputs: &Option<Vec<String>>, svg_size: f64, padding: f64) -> Html {
        if let Some(label_texts) = labels {
            let input_values = user_inputs.as_ref().map(|inputs| inputs.as_slice()).unwrap_or(&[]);
            let svg_center_x = svg_size / 2.0;
            let svg_center_y = svg_size / 2.0;
            
            let positioned_inputs: Vec<Html> = nodes.iter()
                .zip(label_texts.iter())
                .enumerate()
                .map(|(index, (node, label_text))| {
                                         // Use shared positioning logic for perfect alignment with labels
                     let (label_x, label_y) = self.calculate_label_position(node, svg_center_x, svg_center_y);
                     
                     // Convert SVG coordinates to HTML overlay coordinates
                     let input_x = label_x + padding - 40.0; // Center the 80px input field
                     let input_y = label_y + padding + 25.0; // Position below label
                    
                    let current_value = input_values.get(index).cloned().unwrap_or_default();
                    let on_input_change = ctx.props().on_input_change.clone();
                    
                    html! {
                        <input 
                            type="text"
                                                         class="positioned-input-field"
                            style={format!("position: absolute; left: {}px; top: {}px; width: 80px; z-index: 10;", input_x, input_y)}
                            placeholder={format!("{}", label_text.chars().take(6).collect::<String>())}
                            value={current_value}
                            oninput={
                                if let Some(callback) = on_input_change {
                                    callback.reform(move |e: InputEvent| {
                                        let input = e.target_unchecked_into::<HtmlInputElement>();
                                        (index, input.value())
                                    })
                                } else {
                                    Callback::noop()
                                }
                            }
                        />
                    }
                })
                .collect();

            html! {
                <>
                    {for positioned_inputs}
                </>
            }
        } else {
            html! {}
        }
    }
} 