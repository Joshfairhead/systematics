use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(non_snake_case)] // Connective fields use intentional positional-semantic naming
pub struct Heptad {
    pub name: String,
    // User instances for canonical positions
    pub insight: String,      // Position A
    pub research: String,     // Position B
    pub design: String,       // Position C
    pub synthesis: String,    // Position D
    pub application: String,  // Position E
    pub delivery: String,     // Position F
    pub value: String,        // Position G
    // Connectives stored as HashMap for scalability
    pub connectives: HashMap<(usize, usize), String>,
}

impl Heptad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Generative power";

    /// Creates a new Heptad with user instances for canonical positions
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
        let mut heptad = Heptad {
            name: name.to_string(),
            insight: insight.to_string(),
            research: research.to_string(),
            design: design.to_string(),
            synthesis: synthesis.to_string(),
            application: application.to_string(),
            delivery: delivery.to_string(),
            value: value.to_string(),
            connectives: HashMap::new(),
        };

        // Initialize all 21 connectives with positional-semantic defaults
        let terms = vec![
            "insight", "research", "design", "synthesis", 
            "application", "delivery", "value"
        ];
        
        for i in 0..7 {
            for j in (i + 1)..7 {
                let connective_name = format!("{}_{}_{}_{}", 
                    char::from(b'A' + i as u8),
                    char::from(b'A' + j as u8),
                    terms[i], 
                    terms[j]
                );
                heptad.connectives.insert((i, j), connective_name);
            }
        }

        heptad
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
        println!("\n--- Creating a Heptad ---");
        
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
        let name = get_optional_input("Enter a name for your Heptad (or press Enter for 'Unnamed Heptad'): ", "Unnamed Heptad")?;
        let insight = get_optional_input("Enter the Insight instance (or press Enter for 'Insight'): ", "Insight")?;
        let research = get_optional_input("Enter the Research instance (or press Enter for 'Research'): ", "Research")?;
        let design = get_optional_input("Enter the Design instance (or press Enter for 'Design'): ", "Design")?;
        let synthesis = get_optional_input("Enter the Synthesis instance (or press Enter for 'Synthesis'): ", "Synthesis")?;
        let application = get_optional_input("Enter the Application instance (or press Enter for 'Application'): ", "Application")?;
        let delivery = get_optional_input("Enter the Delivery instance (or press Enter for 'Delivery'): ", "Delivery")?;
        let value = get_optional_input("Enter the Value instance (or press Enter for 'Value'): ", "Value")?;

        let mut heptad = Heptad::new(&name, &insight, &research, &design, &synthesis, &application, &delivery, &value);
        
        // Ask if user wants to modify the default connectives
        let modify_connectives = get_yes_no_input("\nWould you like to modify the default connectives? (y/n): ", "y")?;
        
        if modify_connectives.starts_with('y') {
            println!("\nModifying connectives (press Enter to keep default, or input new value):");
            println!("Note: Heptad has 21 connectives - this will take several moments to review.");
            
            // Get all connective keys and sort them for consistent ordering
            let mut keys: Vec<_> = heptad.connectives.keys().cloned().collect();
            keys.sort();
            
            for (i, j) in keys {
                let current_value = heptad.connectives.get(&(i, j)).unwrap();
                let prompt = format!("Connective {}<>{} ({}): ", 
                    char::from(b'A' + i as u8), 
                    char::from(b'A' + j as u8),
                    current_value
                );
                
                let new_value = get_optional_input(&prompt, current_value)?;
                heptad.set_connective(i, j, new_value);
            }
        }
        
        // Display the created heptad
        heptad.display();
        
        // Show connectives if any were defined
        if heptad.has_connectives() {
            heptad.display_connectives();
        }
        
        Ok(heptad)
    }
    
    /// Check if any connectives are defined
    pub fn has_connectives(&self) -> bool {
        !self.connectives.is_empty()
    }
    
    /// Get canonical term names (hardcoded)
    #[allow(dead_code)]
    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec!["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"]
    }
    
    /// Get all user instances
    #[allow(dead_code)]
    pub fn get_instances(&self) -> Vec<String> {
        vec![
            self.insight.clone(),
            self.research.clone(),
            self.design.clone(),
            self.synthesis.clone(),
            self.application.clone(),
            self.delivery.clone(),
            self.value.clone(),
        ]
    }
    
    /// Display heptad details
    pub fn display(&self) {
        println!("\n--- Heptad Details ---");
        println!("Heptad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("A (Insight): {}", self.insight);
        println!("B (Research): {}", self.research);
        println!("C (Design): {}", self.design);
        println!("D (Synthesis): {}", self.synthesis);
        println!("E (Application): {}", self.application);
        println!("F (Delivery): {}", self.delivery);
        println!("G (Value): {}", self.value);
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
    fn test_heptad_creation() {
        let heptad = Heptad::new(
            "Test Heptad",
            "Core Insight",
            "Deep Research",
            "Creative Design",
            "Smart Synthesis",
            "Practical Application",
            "Effective Delivery",
            "Real Value"
        );
        
        assert_eq!(heptad.name, "Test Heptad");
        assert_eq!(heptad.insight, "Core Insight");
        assert_eq!(heptad.research, "Deep Research");
        assert_eq!(heptad.design, "Creative Design");
        assert_eq!(heptad.synthesis, "Smart Synthesis");
        assert_eq!(heptad.application, "Practical Application");
        assert_eq!(heptad.delivery, "Effective Delivery");
        assert_eq!(heptad.value, "Real Value");
        
        // Should have default connectives
        assert!(heptad.has_connectives());
        assert_eq!(heptad.connectives_count(), 21);
    }

    #[test]
    fn test_canonical_terms() {
        let terms = Heptad::get_canonical_terms();
        assert_eq!(terms, vec!["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"]);
    }

    #[test]
    fn test_get_instances() {
        let heptad = Heptad::new(
            "Test",
            "Core Insight",
            "Deep Research",
            "Creative Design",
            "Smart Synthesis",
            "Practical Application",
            "Effective Delivery",
            "Real Value"
        );
        
        let instances = heptad.get_instances();
        assert_eq!(instances, vec![
            "Core Insight", "Deep Research", "Creative Design", "Smart Synthesis", 
            "Practical Application", "Effective Delivery", "Real Value"
        ]);
    }

    #[test]
    fn test_has_connectives_with_some() {
        let heptad = Heptad::new("Test", "A", "B", "C", "D", "E", "F", "G");
        assert!(heptad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut heptad = Heptad::new("Test", "A", "B", "C", "D", "E", "F", "G");
        heptad.connectives.clear();
        assert!(!heptad.has_connectives());
    }

    #[test]
    fn test_positional_semantic_connectives() {
        let heptad = Heptad::new("Test", "A", "B", "C", "D", "E", "F", "G");
        
        // Should start with positional-semantic connectives
        assert!(heptad.has_connectives());
        assert_eq!(heptad.get_connective(0, 1).unwrap(), "A_B_insight_research");
        assert_eq!(heptad.get_connective(2, 3).unwrap(), "C_D_design_synthesis");
        assert_eq!(heptad.get_connective(4, 5).unwrap(), "E_F_application_delivery");
        assert_eq!(heptad.get_connective(5, 6).unwrap(), "F_G_delivery_value");
    }

    #[test]
    fn test_custom_connectives() {
        let mut heptad = Heptad::new("Test", "A", "B", "C", "D", "E", "F", "G");
        
        // Modify one connective while others keep defaults
        heptad.set_connective(0, 1, "custom connection".to_string());
        
        assert!(heptad.has_connectives());
        assert_eq!(heptad.get_connective(0, 1).unwrap(), "custom connection");
        
        // Other connectives should still have defaults
        assert_eq!(heptad.get_connective(2, 3).unwrap(), "C_D_design_synthesis");
        assert_eq!(heptad.get_connective(4, 5).unwrap(), "E_F_application_delivery");
    }

    #[test]
    fn test_all_connectives_count() {
        let heptad = Heptad::new("Test", "A", "B", "C", "D", "E", "F", "G");
        
        // Should have exactly 21 connectives (7 choose 2 = 21)
        assert_eq!(heptad.connectives_count(), 21);
    }

    #[test]
    fn test_connective_helper_methods() {
        let mut heptad = Heptad::new("Test", "A", "B", "C", "D", "E", "F", "G");
        
        // Test get/set connective methods
        assert!(heptad.get_connective(0, 1).is_some());
        
        heptad.set_connective(0, 1, "new value".to_string());
        assert_eq!(heptad.get_connective(0, 1).unwrap(), "new value");
        
        // Test bidirectional access (should work both ways)
        assert_eq!(heptad.get_connective(1, 0).unwrap(), "new value");
    }
} 