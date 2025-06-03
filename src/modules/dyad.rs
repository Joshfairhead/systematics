use std::io::{self, Write};

#[derive(Debug)]
pub struct Dyad {
    pub name: String,
    pub essence: String,
    pub existence: String,
}

impl Dyad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Complementarity, polarity or force";

    /// Creates a new Dyad.
    pub fn new(name: &str, essence: &str, existence: &str) -> Self {
        Dyad {
            name: name.to_string(),
            essence: essence.to_string(),
            existence: existence.to_string(),
        }
    }
    
    /// Interactive creation method - handles all input/output internally
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Dyad ---");
        
        // Helper for required input - loops until valid input is provided
        let get_required_input = |prompt: &str, field_name: &str| -> Result<String, Box<dyn std::error::Error>> {
            loop {
                let mut input = String::new();
                print!("{}", prompt);
                
                if let Err(e) = io::stdout().flush() {
                    return Err(format!("Could not flush output: {}", e).into());
                }
                
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let trimmed = input.trim();
                        
                        // Check for empty input - prompt again
                        if trimmed.is_empty() {
                            println!("{} is required. Please enter a value.", field_name);
                            continue;
                        }
                        
                        // Check for reasonable length (1-100 characters)
                        if trimmed.len() > 100 {
                            println!("{} is too long (max 100 characters). Please try again.", field_name);
                            continue;
                        }
                        
                        // Check for valid characters (letters, numbers, spaces, basic punctuation)
                        if !trimmed.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                            println!("{} contains invalid characters. Please use only letters, numbers, spaces, and basic punctuation.", field_name);
                            continue;
                        }
                        
                        return Ok(trimmed.to_string());
                    }
                    Err(e) => return Err(format!("Error reading {}: {}", field_name, e).into())
                }
            }
        };
        
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
        
        // Helper for yes/no input
        let _get_yes_no_input = |prompt: &str, default: &str| -> Result<String, Box<dyn std::error::Error>> {
            loop {
                let mut input = String::new();
                print!("{}", prompt);
                
                if let Err(e) = io::stdout().flush() {
                    eprintln!("Warning: Could not flush output: {}", e);
                }
                
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let trimmed = input.trim().to_lowercase();
                        
                        if trimmed.is_empty() {
                            return Ok(default.to_string());
                        }
                        
                        match trimmed.as_str() {
                            "y" | "yes" | "n" | "no" => return Ok(trimmed),
                            _ => {
                                println!("Please enter 'y' or 'n' (or press Enter for default: {})", default);
                                continue;
                            }
                        }
                    }
                    Err(e) => return Err(format!("Error reading input: {}", e).into())
                }
            }
        };
        
        // Get required inputs
        let name = get_optional_input("Enter a name for your Dyad (or press Enter for 'Unnamed Dyad'): ", "Unnamed Dyad")?;
        let essence = get_required_input("Enter the Essence instance: ", "Essence instance")?;
        let existence = get_required_input("Enter the Existence instance: ", "Existence instance")?;

        let dyad = Dyad::new(&name, &essence, &existence);
        
        // Display the created dyad
        dyad.display();
        
        Ok(dyad)
    }
    
    /// Check if dyad has valid terms defined
    #[allow(dead_code)]
    pub fn has_terms(&self) -> bool {
        !self.essence.is_empty() && !self.existence.is_empty()
    }
    
    /// Get canonical terms for Dyad
    #[allow(dead_code)]
    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec!["Essence", "Existence"]
    }
    
    /// Get user instances for all canonical positions
    #[allow(dead_code)]
    pub fn get_instances(&self) -> Vec<String> {
        vec![self.essence.clone(), self.existence.clone()]
    }
    
    /// Display dyad details
    pub fn display(&self) {
        println!("\n--- Dyad Details ---");
        println!("Dyad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("Essence: {}", self.essence);
        println!("Existence: {}", self.existence);
        println!("---------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dyad_creation() {
        let dyad = Dyad::new("Test Dyad", "Being", "Becoming");
        
        assert_eq!(dyad.name, "Test Dyad");
        assert_eq!(dyad.essence, "Being");
        assert_eq!(dyad.existence, "Becoming");
    }

    #[test]
    fn test_dyad_creation_with_empty_name() {
        let dyad = Dyad::new("", "Essence", "Existence");
        
        assert_eq!(dyad.name, "");
        assert_eq!(dyad.essence, "Essence");
        assert_eq!(dyad.existence, "Existence");
    }

    #[test]
    fn test_canonical_terms() {
        let terms = Dyad::get_canonical_terms();
        assert_eq!(terms, vec!["Essence", "Existence"]);
    }

    #[test]
    fn test_get_instances() {
        let dyad = Dyad::new("Test", "Universal Being", "Cosmic Becoming");
        let instances = dyad.get_instances();
        
        assert_eq!(instances.len(), 2);
        assert_eq!(instances[0], "Universal Being");
        assert_eq!(instances[1], "Cosmic Becoming");
    }

    #[test]
    fn test_has_terms_with_content() {
        let dyad = Dyad::new("Test", "Being", "Becoming");
        assert!(dyad.has_terms());
    }

    #[test]
    fn test_has_terms_with_empty_essence() {
        let dyad = Dyad::new("Test", "", "Becoming");
        assert!(!dyad.has_terms());
    }

    #[test]
    fn test_has_terms_with_empty_existence() {
        let dyad = Dyad::new("Test", "Being", "");
        assert!(!dyad.has_terms());
    }

    #[test]
    fn test_has_terms_with_both_empty() {
        let dyad = Dyad::new("Test", "", "");
        assert!(!dyad.has_terms());
    }

    #[test]
    fn test_term_attribute_description() {
        assert_eq!(Dyad::TERM_ATTRIBUTE_DESCRIPTION, "Complementarity, polarity or force");
    }

    #[test]
    fn test_dyad_with_special_characters() {
        let dyad = Dyad::new("Test-Dyad!", "Being (Primary)", "Becoming, Universal");
        
        assert_eq!(dyad.name, "Test-Dyad!");
        assert_eq!(dyad.essence, "Being (Primary)");
        assert_eq!(dyad.existence, "Becoming, Universal");
    }

    #[test]
    fn test_dyad_with_unicode() {
        let dyad = Dyad::new("测试", "存在", "成为");
        
        assert_eq!(dyad.name, "测试");
        assert_eq!(dyad.essence, "存在");
        assert_eq!(dyad.existence, "成为");
    }

    #[test]
    fn test_dyad_clone_and_modify() {
        let original = Dyad::new("Original", "Original Essence", "Original Existence");
        let mut modified = Dyad::new(&original.name, &original.essence, &original.existence);
        
        modified.name = "Modified".to_string();
        modified.essence = "Modified Essence".to_string();
        modified.existence = "Modified Existence".to_string();
        
        // Original should be unchanged
        assert_eq!(original.name, "Original");
        assert_eq!(original.essence, "Original Essence");
        assert_eq!(original.existence, "Original Existence");
        
        // Modified should be changed
        assert_eq!(modified.name, "Modified");
        assert_eq!(modified.essence, "Modified Essence");
        assert_eq!(modified.existence, "Modified Existence");
    }

    #[test]
    fn test_dyad_debug_format() {
        let dyad = Dyad::new("Debug Test", "Debug Essence", "Debug Existence");
        let debug_str = format!("{:?}", dyad);
        
        assert!(debug_str.contains("Debug Test"));
        assert!(debug_str.contains("Debug Essence"));
        assert!(debug_str.contains("Debug Existence"));
    }

    #[test]
    fn test_canonical_terms_immutable() {
        let terms1 = Dyad::get_canonical_terms();
        let terms2 = Dyad::get_canonical_terms();
        
        assert_eq!(terms1, terms2);
        assert_eq!(terms1.len(), 2);
    }

    #[test]
    fn test_get_instances_order() {
        let dyad = Dyad::new("Order Test", "First Term", "Second Term");
        let instances = dyad.get_instances();
        
        assert_eq!(instances.len(), 2);
        assert_eq!(instances[0], "First Term");
        assert_eq!(instances[1], "Second Term");
    }

    #[test]
    fn test_dyad_construction_consistency() {
        let name = "Consistency Test";
        let essence = "Consistent Essence";
        let existence = "Consistent Existence";
        
        let dyad = Dyad::new(name, essence, existence);
        
        assert_eq!(dyad.name, name);
        assert_eq!(dyad.essence, essence);
        assert_eq!(dyad.existence, existence);
        
        let instances = dyad.get_instances();
        assert_eq!(instances[0], essence);
        assert_eq!(instances[1], existence);
    }

    #[test]
    fn test_empty_string_handling() {
        let dyad = Dyad::new("", "", "");
        
        assert_eq!(dyad.name, "");
        assert_eq!(dyad.essence, "");
        assert_eq!(dyad.existence, "");
        assert!(!dyad.has_terms());
        
        let instances = dyad.get_instances();
        assert_eq!(instances.len(), 2);
        assert_eq!(instances[0], "");
        assert_eq!(instances[1], "");
    }

    #[test]
    fn test_dyad_polarity_concepts() {
        let dyad = Dyad::new("Polarity Test", "Light", "Dark");
        
        assert_eq!(dyad.essence, "Light");
        assert_eq!(dyad.existence, "Dark");
        assert!(dyad.has_terms());
        
        let dyad2 = Dyad::new("Force Test", "Action", "Reaction");
        assert_eq!(dyad2.essence, "Action");
        assert_eq!(dyad2.existence, "Reaction");
    }

    #[test]
    fn test_term_attribute_description_consistency() {
        // Test that the description matches Bennett's terminology for dyads
        let description = Dyad::TERM_ATTRIBUTE_DESCRIPTION;
        assert!(description.contains("polarity") || description.contains("force") || description.contains("Complementarity"));
    }
}