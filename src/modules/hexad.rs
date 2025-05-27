#[derive(Debug)]
pub struct Hexad {
    pub name: String,
    pub resources: String,
    pub values: String,
    pub options: String,
    pub criteria: String,
    pub facts: String,
    pub priorities: String,
}

impl Hexad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Coalescence";

    /// Creates a new Hexad.
    pub fn new(
        name: &str,
        resources: &str,
        values: &str,
        options: &str,
        criteria: &str,
        facts: &str,
        priorities: &str,
    ) -> Self {
        Hexad {
            name: name.to_string(),
            resources: resources.to_string(),
            values: values.to_string(),
            options: options.to_string(),
            criteria: criteria.to_string(),
            facts: facts.to_string(),
            priorities: priorities.to_string(),
        }
    }
} 