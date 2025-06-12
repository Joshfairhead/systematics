use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Hexad {
    pub name: String,
    // User instances for canonical positions
    pub resources: String,     // Position A
    pub values: String,        // Position B
    pub options: String,       // Position C
    pub criteria: String,      // Position D
    pub facts: String,         // Position E
    pub priorities: String,    // Position F
    // Connectives stored as HashMap for scalability
    pub connectives: HashMap<(usize, usize), String>,
}

impl Hexad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Coalescence";

    /// Creates a new Hexad with user instances for canonical positions
    pub fn new(
        name: &str,
        resources: &str,
        values: &str,
        options: &str,
        criteria: &str,
        facts: &str,
        priorities: &str,
    ) -> Self {
        let mut hexad = Hexad {
            name: name.to_string(),
            resources: resources.to_string(),
            values: values.to_string(),
            options: options.to_string(),
            criteria: criteria.to_string(),
            facts: facts.to_string(),
            priorities: priorities.to_string(),
            connectives: HashMap::new(),
        };

        // Initialize all 15 connectives with positional-semantic defaults
        let terms = vec![
            "resources", "values", "options", "criteria", "facts", "priorities"
        ];
        
        for i in 0..6 {
            for j in (i + 1)..6 {
                let connective_name = format!("{}_{}_{}_{}", 
                    char::from(b'A' + i as u8),
                    char::from(b'A' + j as u8),
                    terms[i], 
                    terms[j]
                );
                hexad.connectives.insert((i, j), connective_name);
            }
        }

        hexad
    }

    /// Get a connective by position indices
    pub fn get_connective(&self, i: usize, j: usize) -> Option<&String> {
        let key = if i < j { (i, j) } else { (j, i) };
        self.connectives.get(&key)
    }

    /// Set a connective by position indices
    pub fn set_connective(&mut self, i: usize, j: usize, value: String) {
        let key = if i < j { (i, j) } else { (j, i) };
        self.connectives.insert(key, value);
    }

    /// Get the total number of connectives
    pub fn connectives_count(&self) -> usize {
        self.connectives.len()
    }
    
    /// Interactive creation method - handles all input/output internally
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Hexad ---");
        
        // Helper for required input - loops until valid input is provided
        let _get_required_input = |prompt: &str, field_name: &str| -> Result<String, Box<dyn std::error::Error>> {
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
        let get_yes_no_input = |prompt: &str, default: &str| -> Result<String, Box<dyn std::error::Error>> {
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
        
        // Get inputs using canonical terms
        let name = get_optional_input("Enter a name for your Hexad (or press Enter for 'Unnamed Hexad'): ", "Unnamed Hexad")?;
        let resources = get_optional_input("Enter the Resources instance (or press Enter for 'Resources'): ", "Resources")?;
        let values = get_optional_input("Enter the Values instance (or press Enter for 'Values'): ", "Values")?;
        let options = get_optional_input("Enter the Options instance (or press Enter for 'Options'): ", "Options")?;
        let criteria = get_optional_input("Enter the Criteria instance (or press Enter for 'Criteria'): ", "Criteria")?;
        let facts = get_optional_input("Enter the Facts instance (or press Enter for 'Facts'): ", "Facts")?;
        let priorities = get_optional_input("Enter the Priorities instance (or press Enter for 'Priorities'): ", "Priorities")?;

        let mut hexad = Hexad::new(&name, &resources, &values, &options, &criteria, &facts, &priorities);
        
        // Ask if user wants to modify the default connectives
        let modify_connectives = get_yes_no_input("\nWould you like to modify the default connectives? (y/n): ", "y")?;
        
        if modify_connectives.starts_with('y') {
            println!("\nModifying connectives (press Enter to keep default, or input new value):");
            println!("Note: Hexad has 15 connectives - this will take a moment to review.");
            
            // Get all connective keys and sort them for consistent ordering
            let mut keys: Vec<_> = hexad.connectives.keys().cloned().collect();
            keys.sort();
            
            for (i, j) in keys {
                let current_value = hexad.connectives.get(&(i, j)).unwrap();
                let prompt = format!("Connective {}<>{} ({}): ", 
                    char::from(b'A' + i as u8), 
                    char::from(b'A' + j as u8),
                    current_value
                );
                
                let new_value = get_optional_input(&prompt, current_value)?;
                hexad.set_connective(i, j, new_value);
            }
        }
        
        // Display the created hexad
        hexad.display();
        
        // Show connectives if any were defined
        if hexad.has_connectives() {
            hexad.display_connectives();
        }
        
        Ok(hexad)
    }
    
    /// Check if any connectives are defined
    pub fn has_connectives(&self) -> bool {
        !self.connectives.is_empty()
    }
    
    /// Get canonical term names (hardcoded)
    #[allow(dead_code)]
    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec!["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"]
    }
    
    /// Get all user instances
    #[allow(dead_code)]
    pub fn get_instances(&self) -> Vec<String> {
        vec![
            self.resources.clone(),
            self.values.clone(),
            self.options.clone(),
            self.criteria.clone(),
            self.facts.clone(),
            self.priorities.clone(),
        ]
    }
    
    /// Display hexad details
    pub fn display(&self) {
        println!("\n--- Hexad Details ---");
        println!("Hexad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("A (Resources): {}", self.resources);
        println!("B (Values): {}", self.values);
        println!("C (Options): {}", self.options);
        println!("D (Criteria): {}", self.criteria);
        println!("E (Facts): {}", self.facts);
        println!("F (Priorities): {}", self.priorities);
        println!("---------------------");
    }
    
    /// Display all connectives
    pub fn display_connectives(&self) {
        println!("\nConnectives:");
        let instances = self.get_instances();
        
        // Get all connective keys and sort them for consistent ordering
        let mut keys: Vec<_> = self.connectives.keys().cloned().collect();
        keys.sort();
        
        for (i, j) in keys {
            let connective = self.connectives.get(&(i, j)).unwrap();
            let code = format!("{}<>{}", char::from(b'A' + i as u8), char::from(b'A' + j as u8));
            println!("  {} <--[{}]--> {} ({})", instances[i], connective, instances[j], code);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexad_creation() {
        let hexad = Hexad::new(
            "Test Hexad",
            "Available Resources",
            "Core Values",
            "Possible Options",
            "Selection Criteria",
            "Known Facts",
            "Key Priorities"
        );
        
        assert_eq!(hexad.name, "Test Hexad");
        assert_eq!(hexad.resources, "Available Resources");
        assert_eq!(hexad.values, "Core Values");
        assert_eq!(hexad.options, "Possible Options");
        assert_eq!(hexad.criteria, "Selection Criteria");
        assert_eq!(hexad.facts, "Known Facts");
        assert_eq!(hexad.priorities, "Key Priorities");
        
        // Should have default connectives
        assert!(hexad.has_connectives());
        assert_eq!(hexad.connectives_count(), 15);
    }

    #[test]
    fn test_canonical_terms() {
        let terms = Hexad::get_canonical_terms();
        assert_eq!(terms, vec!["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"]);
    }

    #[test]
    fn test_get_instances() {
        let hexad = Hexad::new(
            "Test",
            "Available Resources",
            "Core Values",
            "Possible Options",
            "Selection Criteria",
            "Known Facts",
            "Key Priorities"
        );
        
        let instances = hexad.get_instances();
        assert_eq!(instances, vec![
            "Available Resources", "Core Values", "Possible Options", 
            "Selection Criteria", "Known Facts", "Key Priorities"
        ]);
    }

    #[test]
    fn test_has_connectives_with_some() {
        let hexad = Hexad::new("Test", "A", "B", "C", "D", "E", "F");
        assert!(hexad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut hexad = Hexad::new("Test", "A", "B", "C", "D", "E", "F");
        hexad.connectives.clear();
        assert!(!hexad.has_connectives());
    }

    #[test]
    fn test_positional_semantic_connectives() {
        let hexad = Hexad::new("Test", "A", "B", "C", "D", "E", "F");
        
        // Should start with positional-semantic connectives
        assert!(hexad.has_connectives());
        assert_eq!(hexad.get_connective(0, 1).unwrap(), "A_B_resources_values");
        assert_eq!(hexad.get_connective(2, 3).unwrap(), "C_D_options_criteria");
        assert_eq!(hexad.get_connective(4, 5).unwrap(), "E_F_facts_priorities");
    }

    #[test]
    fn test_custom_connectives() {
        let mut hexad = Hexad::new("Test", "A", "B", "C", "D", "E", "F");
        
        // Modify one connective while others keep defaults
        hexad.set_connective(0, 1, "custom connection".to_string());
        
        assert!(hexad.has_connectives());
        assert_eq!(hexad.get_connective(0, 1).unwrap(), "custom connection");
        
        // Other connectives should still have defaults
        assert_eq!(hexad.get_connective(2, 3).unwrap(), "C_D_options_criteria");
        assert_eq!(hexad.get_connective(4, 5).unwrap(), "E_F_facts_priorities");
    }

    #[test]
    fn test_all_connectives_count() {
        let hexad = Hexad::new("Test", "A", "B", "C", "D", "E", "F");
        
        // Should have exactly 15 connectives (6 choose 2 = 15)
        assert_eq!(hexad.connectives_count(), 15);
    }

    #[test]
    fn test_connective_helper_methods() {
        let mut hexad = Hexad::new("Test", "A", "B", "C", "D", "E", "F");
        
        // Test get/set connective methods
        assert!(hexad.get_connective(0, 1).is_some());
        
        hexad.set_connective(0, 1, "new value".to_string());
        assert_eq!(hexad.get_connective(0, 1).unwrap(), "new value");
        
        // Test bidirectional access (should work both ways)
        assert_eq!(hexad.get_connective(1, 0).unwrap(), "new value");
    }
} 