#[derive(Debug)]
pub struct Heptad {
    pub name: String,
    pub insight: String,
    pub research: String,
    pub design: String,
    pub synthesis: String,
    pub application: String,
    pub delivery: String,
    pub value: String,
}

impl Heptad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Generative power";

    /// Creates a new Heptad.
    pub fn new(
        name: &str,
        insight: &str,
        research: &str,
        design: &str,
        synthesis: &str,
        application: &str,
        delivery: &str,
        value: &str,
    ) -> Self {
        Heptad {
            name: name.to_string(),
            insight: insight.to_string(),
            research: research.to_string(),
            design: design.to_string(),
            synthesis: synthesis.to_string(),
            application: application.to_string(),
            delivery: delivery.to_string(),
            value: value.to_string(),
        }
    }
} 