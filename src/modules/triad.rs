#[derive(Debug)]
pub struct Triad {
    pub name: String,
    pub active: String,
    pub passive: String,
    pub reconciling: String,
}

impl Triad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "dynamism, relation, will";

    /// Creates a new Triad.
    pub fn new(name: &str, active: &str, passive: &str, reconciling: &str) -> Self {
        Triad {
            name: name.to_string(),
            active: active.to_string(),
            passive: passive.to_string(),
            reconciling: reconciling.to_string(),
        }
    }
}
