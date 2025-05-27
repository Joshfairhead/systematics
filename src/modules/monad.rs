#[derive(Debug)]
pub struct Monad {
    pub name: String,
    pub terms: Vec<String>,
}

impl Monad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Represents unity in diversity and diversity in unity.";

    /// Creates a new monad
    pub fn new(name: &str) -> Self {
        Monad {
            name: name.to_string(),
            terms: Vec::new(),
        }
    }
    /// Adds a term to the monad vector
    pub fn add_term(&mut self, term: &str) {
        self.terms.push(term.to_string());
    }

    /// Retrieves all terms associated with the Monad vector.
    pub fn get_all_terms(&self) -> &Vec<String> {
        &self.terms
    }
}