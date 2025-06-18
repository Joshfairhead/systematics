use yew::{html, Component, Context, Html, Properties};
use std::f64::consts::PI;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub system_type: String,
    pub size: f64,
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
        
        let viewbox = format!("0 0 {} {}", size, size);
        let center_x = size / 2.0;
        let center_y = size / 2.0;
        
        html! {
            <svg 
                width={size.to_string()} 
                height={size.to_string()} 
                viewBox={viewbox}
                class="geometric-structure"
            >
                {self.render_graph(system_type, center_x, center_y, size)}
            </svg>
        }
    }
}

impl GeometricRenderer {
    fn render_graph(&self, system_type: &str, cx: f64, cy: f64, size: f64) -> Html {
        let node_count = match system_type {
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
            _ => return html! { <circle cx={cx.to_string()} cy={cy.to_string()} r="5" fill="gray" /> },
        };

        // Calculate node positions
        let nodes = self.calculate_node_positions(node_count, cx, cy, size);
        
        html! {
            <>
                // Render all edges first (so they appear behind nodes)
                {self.render_all_edges(&nodes)}
                // Render nodes on top
                {self.render_all_nodes(&nodes)}
            </>
        }
    }

    fn calculate_node_positions(&self, node_count: usize, cx: f64, cy: f64, size: f64) -> Vec<(f64, f64)> {
        match node_count {
            1 => vec![(cx, cy)], // Monad: single point at center
            2 => {
                // Dyad: two points horizontally aligned
                let offset = size * 0.15;
                vec![(cx - offset, cy), (cx + offset, cy)]
            },
            _ => {
                // All other systems: evenly spaced around a circle
                let radius = self.get_radius_for_system(node_count, size);
                let rotation = self.get_rotation_for_system(node_count);
                
                (0..node_count).map(|i| {
                    let angle = 2.0 * PI * i as f64 / node_count as f64 + rotation;
                    let x = cx + radius * angle.cos();
                    let y = cy + radius * angle.sin();
                    (x, y)
                }).collect()
            }
        }
    }

    fn get_radius_for_system(&self, node_count: usize, size: f64) -> f64 {
        match node_count {
            3..=6 => size * 0.15,   // Smaller systems
            7..=8 => size * 0.18,   // Medium systems  
            9..=10 => size * 0.20,  // Larger systems
            11..=12 => size * 0.22, // Largest systems
            _ => size * 0.15,       // Default
        }
    }

    fn get_rotation_for_system(&self, node_count: usize) -> f64 {
        match node_count {
            3 | 5 | 7 | 9 | 11 => -PI/2.0, // Odd systems: point at top
            4 => PI/4.0,                    // Tetrad: diamond orientation
            6 => 0.0,                       // Hexad: flat top
            8 => PI/8.0,                    // Octad: slight rotation
            10 | 12 => -PI/2.0,             // Even high systems: point at top
            _ => 0.0,                       // Default
        }
    }

    fn render_all_edges(&self, nodes: &[(f64, f64)]) -> Html {
        if nodes.len() < 2 {
            return html! {}; // No edges for monad
        }

        let mut edges = Vec::new();
        
        // Generate all possible edges (complete graph)
        for i in 0..nodes.len() {
            for j in (i+1)..nodes.len() {
                let (x1, y1) = nodes[i];
                let (x2, y2) = nodes[j];
                
                edges.push(html! {
                    <line 
                        x1={x1.to_string()} 
                        y1={y1.to_string()} 
                        x2={x2.to_string()} 
                        y2={y2.to_string()}
                        stroke="#667eea" 
                        stroke-width="1" 
                        opacity="0.3"
                    />
                });
            }
        }

        html! {
            <>
                {for edges}
            </>
        }
    }

    fn render_all_nodes(&self, nodes: &[(f64, f64)]) -> Html {
        let node_radius = self.get_node_radius(nodes.len());
        
        html! {
            <>
                {for nodes.iter().map(|(x, y)| {
                    html! {
                        <circle 
                            cx={x.to_string()} 
                            cy={y.to_string()} 
                            r={node_radius.to_string()} 
                            fill="#667eea"
                            stroke="#ffffff"
                            stroke-width="2"
                        />
                    }
                })}
            </>
        }
    }

    fn get_node_radius(&self, node_count: usize) -> f64 {
        match node_count {
            1 => 12.0,      // Monad: larger single node
            2 => 10.0,      // Dyad: medium nodes
            3..=6 => 8.0,   // Small to medium systems
            7..=9 => 6.0,   // Medium to large systems
            10..=12 => 5.0, // Large systems: smaller nodes to avoid crowding
            _ => 6.0,       // Default
        }
    }
} 