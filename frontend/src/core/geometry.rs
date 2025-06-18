use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct GraphLayout {
    pub nodes: Vec<Point>,
    pub edges: Vec<Edge>,
    pub node_radius: f64,
}

pub struct GeometryCalculator;

impl GeometryCalculator {
    pub fn calculate_system_layout(
        system_type: &str,
        center_x: f64,
        center_y: f64,
        size: f64,
    ) -> GraphLayout {
        let node_count = Self::get_node_count(system_type);
        let nodes = Self::calculate_node_positions(node_count, center_x, center_y, size);
        let edges = Self::generate_complete_graph_edges(node_count);
        let node_radius = Self::get_node_radius(node_count);

        GraphLayout {
            nodes,
            edges,
            node_radius,
        }
    }

    fn get_node_count(system_type: &str) -> usize {
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

    fn calculate_node_positions(
        node_count: usize,
        cx: f64,
        cy: f64,
        size: f64,
    ) -> Vec<Point> {
        match node_count {
            1 => vec![Point { x: cx, y: cy }],
            2 => {
                let offset = size * 0.15;
                vec![
                    Point { x: cx - offset, y: cy },
                    Point { x: cx + offset, y: cy },
                ]
            }
            _ => {
                let radius = Self::get_radius_for_system(node_count, size);
                let rotation = Self::get_rotation_for_system(node_count);

                (0..node_count)
                    .map(|i| {
                        let angle = 2.0 * PI * i as f64 / node_count as f64 + rotation;
                        Point {
                            x: cx + radius * angle.cos(),
                            y: cy + radius * angle.sin(),
                        }
                    })
                    .collect()
            }
        }
    }

    fn get_radius_for_system(node_count: usize, size: f64) -> f64 {
        match node_count {
            3..=6 => size * 0.15,
            7..=8 => size * 0.18,
            9..=10 => size * 0.20,
            11..=12 => size * 0.22,
            _ => size * 0.15,
        }
    }

    fn get_rotation_for_system(node_count: usize) -> f64 {
        match node_count {
            3 | 5 | 7 | 9 | 11 => -PI / 2.0,
            4 => PI / 4.0,
            6 => 0.0,
            8 => PI / 8.0,
            10 | 12 => -PI / 2.0,
            _ => 0.0,
        }
    }

    fn generate_complete_graph_edges(node_count: usize) -> Vec<Edge> {
        let mut edges = Vec::new();
        
        for i in 0..node_count {
            for j in (i + 1)..node_count {
                edges.push(Edge { from: i, to: j });
            }
        }
        
        edges
    }

    fn get_node_radius(node_count: usize) -> f64 {
        match node_count {
            1 => 12.0,
            2 => 10.0,
            3..=6 => 8.0,
            7..=9 => 6.0,
            10..=12 => 5.0,
            _ => 6.0,
        }
    }
} 