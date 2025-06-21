use yew::{html, Component, Context, Html, Properties};
use crate::core::geometry::{GeometryCalculator, Point, Edge, SymbolicCircle, SymbolicTriangle};
use crate::services::api::ConnectiveInfo;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub system_type: String,
    pub size: f64,
    #[prop_or_default]
    pub connectives: Option<Vec<ConnectiveInfo>>,
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
        
        // Fix viewBox - use simpler coordinates that match the actual size
        let viewbox = format!("0 0 {} {}", size, size);
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
            <svg 
                class="geometric-renderer" 
                width={size.to_string()}
                height={size.to_string()}
                viewBox={viewbox}
                xmlns="http://www.w3.org/2000/svg"
            >
                {self.render_symbolic_elements(&layout)}
                {self.render_edges_with_connectives(&layout.edges, &layout.nodes, &props.connectives)}
                {self.render_nodes(&layout.nodes, layout.node_radius)}
            </svg>
        }
    }
}

impl GeometricRenderer {
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
    fn render_edges_with_connectives(&self, edges: &[Edge], nodes: &[Point], connectives: &Option<Vec<ConnectiveInfo>>) -> Html {
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
                            html! {
                                <text
                                    x={x.to_string()}
                                    y={y.to_string()}
                                    text-anchor="middle"
                                    dominant-baseline="middle"
                                    font-size="12"
                                    font-family="Arial, sans-serif"
                                    fill="#333"
                                    stroke="white"
                                    stroke-width="2"
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
} 