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
    pub symbolic_circle: Option<SymbolicCircle>,
    pub symbolic_circles: Vec<SymbolicCircle>,
    pub symbolic_triangle: Option<SymbolicTriangle>,
}

#[derive(Debug, Clone)]
pub struct SymbolicCircle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct SymbolicTriangle {
    pub vertices: [Point; 3],
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
        let symbolic_circle = Self::get_symbolic_circle(system_type, center_x, center_y, size);
        let symbolic_circles = Self::get_symbolic_circles(system_type, center_x, center_y, size);
        let symbolic_triangle = Self::get_symbolic_triangle(system_type, center_x, center_y, size);

        GraphLayout {
            nodes,
            edges,
            node_radius,
            symbolic_circle,
            symbolic_circles,
            symbolic_triangle,
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
                let radius = size * 0.3;     // Reduced from 0.4 to prevent truncation
                let offset = radius / 2.0;   // Match symbolic circle offset
                vec![
                    Point { x: cx - offset, y: cy },
                    Point { x: cx + offset, y: cy },
                ]
            }
            3 => {
                // Equilateral triangle on its side: two nodes left, one right
                // API canonical order: [Will, Being, Function]
                // Map API indices to visual positions:
                // API Index 0 (Will) → Top-left
                // API Index 1 (Being) → Right
                // API Index 2 (Function) → Bottom-left
                let side_length = size * 0.65;  // Slightly bigger triangle
                let height = side_length * (3.0_f64.sqrt() / 2.0);  // Height of equilateral triangle
                let half_side = side_length / 2.0;
                let right_offset = height / 2.0 + size * 0.05;  // Move Being further right
                
                vec![
                    // Index 0: Will → Top-left
                    Point { x: cx - height / 2.0, y: cy - half_side },
                    // Index 1: Being → Right (moved further right)
                    Point { x: cx + right_offset, y: cy },
                    // Index 2: Function → Bottom-left
                    Point { x: cx - height / 2.0, y: cy + half_side },
                ]
            }
            4 => {
                // Diamond shape for tetrad
                // API canonical order: [Ideal, Directive, Instrumental, Ground]
                // Map API indices to visual positions:
                // API Index 0 (Ideal) → Top
                // API Index 1 (Directive) → Right
                // API Index 2 (Instrumental) → Left
                // API Index 3 (Ground) → Bottom
                let diamond_size = size * 0.35;  // Larger diamond for better visibility
                
                vec![
                    // Index 0: Ideal → Top
                    Point { x: cx, y: cy - diamond_size },
                    // Index 1: Directive → Right
                    Point { x: cx + diamond_size, y: cy },
                    // Index 2: Instrumental → Left
                    Point { x: cx - diamond_size, y: cy },
                    // Index 3: Ground → Bottom
                    Point { x: cx, y: cy + diamond_size },
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
            1 => 12.0,      // Monad
            2 => 12.0,      // Dyad: same size as monad
            3 => 12.0,      // Triad: same size as monad
            4..=6 => 12.0,  // Tetrad-Hexad: same size as monad
            7..=9 => 12.0,  // Heptad-Ennead: same size as monad
            10..=12 => 12.0, // Decad-Dodecad: same size as monad
            _ => 12.0,
        }
    }

    fn get_symbolic_circle(system_type: &str, center_x: f64, center_y: f64, size: f64) -> Option<SymbolicCircle> {
        match system_type {
            "monad" => Some(SymbolicCircle {
                center: Point { x: center_x, y: center_y },
                radius: size * 0.45,  // Large circle for user attributes
            }),
            _ => None,
        }
    }

    fn get_symbolic_circles(system_type: &str, center_x: f64, center_y: f64, size: f64) -> Vec<SymbolicCircle> {
        match system_type {
            "dyad" => {
                let radius = size * 0.3;     // Reduced to match node positioning
                let offset = radius / 2.0;   // Distance = half radius so circles pass through centers
                vec![
                    SymbolicCircle {
                        center: Point { x: center_x - offset, y: center_y },
                        radius,
                    },
                    SymbolicCircle {
                        center: Point { x: center_x + offset, y: center_y },
                        radius,
                    },
                ]
            },
            _ => vec![],
        }
    }

    fn get_symbolic_triangle(system_type: &str, _center_x: f64, _center_y: f64, _size: f64) -> Option<SymbolicTriangle> {
        // Triad uses edges between nodes to form triangle, no separate symbolic triangle needed
        None
    }
} 