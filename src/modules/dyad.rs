#[derive(Debug)]
pub struct Dyad {
    pub name: String,
    pub essence: String,
    pub existence: String,
}

impl Dyad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Complimentarity, polarity or force";

    /// Creates a new Dyad.
    pub fn new(name: &str, essence: &str, existence: &str) -> Self {
        Dyad {
            name: name.to_string(),
            essence: essence.to_string(),
            existence: existence.to_string(),
        }
    }
}