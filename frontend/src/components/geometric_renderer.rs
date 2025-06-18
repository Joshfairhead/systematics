use yew::{html, Component, Context, Html, Properties};
use crate::core::geometry::{GeometryCalculator, Point, Edge};

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
        
        // Use framework-agnostic core logic
        let layout = GeometryCalculator::calculate_system_layout(
            system_type, 
            center_x, 
            center_y, 
            size
        );
        
        html! {
            <svg 
                width={size.to_string()} 
                height={size.to_string()} 
                viewBox={viewbox}
                class="geometric-structure"
            >
                // Render edges first (behind nodes)
                {self.render_edges(&layout.edges, &layout.nodes)}
                // Render nodes on top
                {self.render_nodes(&layout.nodes, layout.node_radius)}
            </svg>
        }
    }
}

impl GeometricRenderer {
    /// Framework-specific rendering of edges (Yew HTML)
    /// Core logic is in GeometryCalculator (framework-agnostic)
    fn render_edges(&self, edges: &[Edge], nodes: &[Point]) -> Html {
        if edges.is_empty() {
            return html! {};
        }

        let edge_elements: Vec<Html> = edges.iter().map(|edge| {
            let from_node = &nodes[edge.from];
            let to_node = &nodes[edge.to];
            
            html! {
                <line 
                    x1={from_node.x.to_string()} 
                    y1={from_node.y.to_string()} 
                    x2={to_node.x.to_string()} 
                    y2={to_node.y.to_string()}
                    stroke="#667eea" 
                    stroke-width="1" 
                    opacity="0.3"
                />
            }
        }).collect();

        html! {
            <>
                {for edge_elements}
            </>
        }
    }

    /// Framework-specific rendering of nodes (Yew HTML)
    /// Core logic is in GeometryCalculator (framework-agnostic)
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


} 