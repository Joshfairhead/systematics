#[derive(Debug)]
pub struct Tetrad {
    pub name: String,
    pub ground: String,
    pub ideal: String,
    pub instrumental: String,
    pub directive: String,
}

impl Tetrad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "A Field of Action";

    /// Creates a new Tetrad.
    pub fn new(name: &str, ground: &str, ideal: &str, instrumental: &str, directive: &str) -> Self {
        Tetrad {
            name: name.to_string(),
            ground: ground.to_string(),
            ideal: ideal.to_string(),
            instrumental: instrumental.to_string(),
            directive: directive.to_string(),
        }
    }
}