use std::io::{self, Write};

#[derive(Debug)]
pub struct Monad {
    pub name: String,
    pub terms: Vec<String>,
}

impl Monad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Unity in diversity and diversity in unity";

    /// Creates a new monad
    pub fn new(name: &str) -> Self {
        Monad {
            name: name.to_string(),
            terms: Vec::new(),
        }
    }

    /// Interactive creation method - handles all input/output internally
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Monad ---");
        
        // Helper for optional input with default
        let get_optional_input = |prompt: &str, default: &str| -> Result<String, Box<dyn std::error::Error>> {
            let mut input = String::new();
            print!("{}", prompt);
            
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Could not flush output: {}", e);
            }
            
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed = input.trim();
                    
                    if trimmed.is_empty() {
                        Ok(default.to_string())
                    } else {
                        // Validate optional input if provided
                        if trimmed.len() > 100 {
                            return Err("Input is too long (max 100 characters)".into());
                        }
                        
                        if !trimmed.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                            return Err("Input contains invalid characters".into());
                        }
                        
                        Ok(trimmed.to_string())
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}. Using default: {}", e, default);
                    Ok(default.to_string())
                }
            }
        };

        // Helper for term input with validation
        let get_term_input = |prompt: &str| -> Result<Option<String>, Box<dyn std::error::Error>> {
            let mut input = String::new();
            print!("{}", prompt);
            
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Could not flush output: {}", e);
            }
            
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed = input.trim();
                    
                    if trimmed.is_empty() {
                        Ok(None) // Empty input signals end of term entry
                    } else {
                        // Validate term input
                        if trimmed.len() > 100 {
                            return Err("Term is too long (max 100 characters)".into());
                        }
                        
                        if !trimmed.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                            return Err("Term contains invalid characters".into());
                        }
                        
                        Ok(Some(trimmed.to_string()))
                    }
                }
                Err(e) => Err(format!("Error reading term: {}", e).into())
            }
        };

        // Get monad name
        let name = get_optional_input("Enter a name for your Monad (or press Enter for 'Unnamed Monad'): ", "Unnamed Monad")?;

        let mut monad = Monad::new(&name);

        // Get terms for the monad
        println!("\nEnter terms for \"{}\". Press Enter on an empty line when done.", monad.name);
        loop {
            match get_term_input("Term: ") {
                Ok(Some(term)) => {
                    monad.add_term(&term);
                }
                Ok(None) => {
                    break; // Empty input, user is done
                }
                Err(e) => {
                    eprintln!("Error: {}. Please try again.", e);
                    continue;
                }
            }
        }

        // Display the created monad
        monad.display();
        
        Ok(monad)
    }

    /// Adds a term to the monad vector with validation
    pub fn add_term(&mut self, term: &str) {
        if !term.is_empty() {
            self.terms.push(term.to_string());
        }
    }

    /// Retrieves all terms associated with the Monad vector.
    pub fn get_all_terms(&self) -> &Vec<String> {
        &self.terms
    }

    /// Check if monad has any terms defined
    #[allow(dead_code)]
    pub fn has_terms(&self) -> bool {
        !self.terms.is_empty()
    }
    
    /// Get the number of terms in the monad
    #[allow(dead_code)]
    pub fn term_count(&self) -> usize {
        self.terms.len()
    }

    /// Display the monad in a formatted way
    pub fn display(&self) {
        println!("\n--- Monad Details ---");
        println!("Monad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        let terms = self.get_all_terms();
        if !terms.is_empty() {
            println!("User-defined Terms:");
            for term in terms {
                println!("- {}", term);
            }
        } else {
            println!("No user-defined terms were added.");
        }
        println!("---------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monad_creation() {
        let monad = Monad::new("Test Monad");
        
        assert_eq!(monad.name, "Test Monad");
        assert_eq!(monad.terms.len(), 0);
        assert!(!monad.has_terms());
    }

    #[test]
    fn test_monad_creation_with_empty_name() {
        let monad = Monad::new("");
        
        assert_eq!(monad.name, "");
        assert_eq!(monad.terms.len(), 0);
    }

    #[test]
    fn test_add_term() {
        let mut monad = Monad::new("Test");
        
        monad.add_term("Unity");
        monad.add_term("Consciousness");
        
        assert_eq!(monad.terms.len(), 2);
        assert_eq!(monad.terms[0], "Unity");
        assert_eq!(monad.terms[1], "Consciousness");
        assert!(monad.has_terms());
    }

    #[test]
    fn test_add_empty_term() {
        let mut monad = Monad::new("Test");
        
        monad.add_term(""); // Should not add empty terms
        monad.add_term("Valid Term");
        
        assert_eq!(monad.terms.len(), 1);
        assert_eq!(monad.terms[0], "Valid Term");
    }

    #[test]
    fn test_get_all_terms() {
        let mut monad = Monad::new("Test");
        monad.add_term("First");
        monad.add_term("Second");
        
        let terms = monad.get_all_terms();
        assert_eq!(terms.len(), 2);
        assert_eq!(terms[0], "First");
        assert_eq!(terms[1], "Second");
    }

    #[test]
    fn test_has_terms() {
        let mut monad = Monad::new("Test");
        assert!(!monad.has_terms());
        
        monad.add_term("Some term");
        assert!(monad.has_terms());
    }

    #[test]
    fn test_term_count() {
        let mut monad = Monad::new("Test");
        assert_eq!(monad.term_count(), 0);
        
        monad.add_term("First");
        assert_eq!(monad.term_count(), 1);
        
        monad.add_term("Second");
        assert_eq!(monad.term_count(), 2);
    }

    #[test]
    fn test_term_attribute_description() {
        assert_eq!(Monad::TERM_ATTRIBUTE_DESCRIPTION, "Unity in diversity and diversity in unity");
    }

    #[test]
    fn test_monad_with_special_characters() {
        let mut monad = Monad::new("Test-Monad!");
        monad.add_term("Unity (Primary)");
        monad.add_term("Consciousness, Universal");
        
        assert_eq!(monad.name, "Test-Monad!");
        assert_eq!(monad.terms[0], "Unity (Primary)");
        assert_eq!(monad.terms[1], "Consciousness, Universal");
    }

    #[test]
    fn test_monad_with_unicode() {
        let mut monad = Monad::new("测试");
        monad.add_term("统一体");
        monad.add_term("意识");
        
        assert_eq!(monad.name, "测试");
        assert_eq!(monad.terms[0], "统一体");
        assert_eq!(monad.terms[1], "意识");
    }

    #[test]
    fn test_monad_clone_behavior() {
        let mut original = Monad::new("Original");
        original.add_term("Original Term");
        
        let mut cloned = Monad::new(&original.name);
        for term in original.get_all_terms() {
            cloned.add_term(term);
        }
        
        cloned.add_term("New Term");
        
        // Original should be unchanged
        assert_eq!(original.terms.len(), 1);
        assert_eq!(original.terms[0], "Original Term");
        
        // Cloned should have both terms
        assert_eq!(cloned.terms.len(), 2);
        assert_eq!(cloned.terms[0], "Original Term");
        assert_eq!(cloned.terms[1], "New Term");
    }

    #[test]
    fn test_monad_debug_format() {
        let mut monad = Monad::new("Debug Test");
        monad.add_term("Debug Term");
        
        let debug_str = format!("{:?}", monad);
        
        assert!(debug_str.contains("Debug Test"));
        assert!(debug_str.contains("Debug Term"));
    }

    #[test]
    fn test_multiple_term_addition() {
        let mut monad = Monad::new("Multi Term Test");
        
        let terms = vec!["Alpha", "Beta", "Gamma", "Delta"];
        for term in &terms {
            monad.add_term(term);
        }
        
        assert_eq!(monad.term_count(), 4);
        for (i, term) in terms.iter().enumerate() {
            assert_eq!(monad.terms[i], *term);
        }
    }

    #[test]
    fn test_empty_string_handling() {
        let mut monad = Monad::new("");
        monad.add_term("");
        monad.add_term("Valid");
        monad.add_term("");
        
        assert_eq!(monad.name, "");
        assert_eq!(monad.term_count(), 1);
        assert_eq!(monad.terms[0], "Valid");
    }

    #[test]
    fn test_term_order_preservation() {
        let mut monad = Monad::new("Order Test");
        
        monad.add_term("First");
        monad.add_term("Second");
        monad.add_term("Third");
        
        let terms = monad.get_all_terms();
        assert_eq!(terms[0], "First");
        assert_eq!(terms[1], "Second");
        assert_eq!(terms[2], "Third");
    }
}