use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(non_snake_case)] // Connective fields use intentional positional-semantic naming
pub struct Dodecad {
    pub name: String,
    // User instances for canonical positions
    pub autocracy: String,      // Position A
    pub domination: String,     // Position B
    pub creativity: String,     // Position C
    pub pattern: String,        // Position D
    pub individuality: String,  // Position E
    pub structure: String,      // Position F
    pub repetition: String,     // Position G
    pub potentiality: String,   // Position H
    pub subsistence: String,    // Position I
    pub relatedness: String,    // Position J
    pub polarity: String,       // Position K
    pub wholeness: String,      // Position L
    // Connectives stored as a HashMap of (from_pos, to_pos) -> connective_name
    pub connectives: HashMap<(usize, usize), String>,
}

impl Dodecad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Totality";

    /// Creates a new Dodecad.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        autocracy: &str,
        domination: &str,
        creativity: &str,
        pattern: &str,
        individuality: &str,
        structure: &str,
        repetition: &str,
        potentiality: &str,
        subsistence: &str,
        relatedness: &str,
        polarity: &str,
        wholeness: &str,
    ) -> Self {
        let positions = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
        
        // Generate all connectives automatically using just position letters
        let mut connectives = HashMap::new();
        for i in 0..12 {
            for j in (i + 1)..12 {
                let connective_name = format!("{}{}", positions[i], positions[j]);
                connectives.insert((i, j), connective_name);
            }
        }

        Dodecad {
            name: name.to_string(),
            autocracy: autocracy.to_string(),
            domination: domination.to_string(),
            creativity: creativity.to_string(),
            pattern: pattern.to_string(),
            individuality: individuality.to_string(),
            structure: structure.to_string(),
            repetition: repetition.to_string(),
            potentiality: potentiality.to_string(),
            subsistence: subsistence.to_string(),
            relatedness: relatedness.to_string(),
            polarity: polarity.to_string(),
            wholeness: wholeness.to_string(),
            connectives,
        }
    }

    /// Interactive creation method - handles all input/output internally
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Dodecad ---");
        
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
        let name = get_optional_input("Enter a name for your Dodecad (or press Enter for 'Unnamed Dodecad'): ", "Unnamed Dodecad")?;
        let autocracy = get_optional_input("Enter the Autocracy instance (or press Enter for 'Autocracy'): ", "Autocracy")?;
        let domination = get_optional_input("Enter the Domination instance (or press Enter for 'Domination'): ", "Domination")?;
        let creativity = get_optional_input("Enter the Creativity instance (or press Enter for 'Creativity'): ", "Creativity")?;
        let pattern = get_optional_input("Enter the Pattern instance (or press Enter for 'Pattern'): ", "Pattern")?;
        let individuality = get_optional_input("Enter the Individuality instance (or press Enter for 'Individuality'): ", "Individuality")?;
        let structure = get_optional_input("Enter the Structure instance (or press Enter for 'Structure'): ", "Structure")?;
        let repetition = get_optional_input("Enter the Repetition instance (or press Enter for 'Repetition'): ", "Repetition")?;
        let potentiality = get_optional_input("Enter the Potentiality instance (or press Enter for 'Potentiality'): ", "Potentiality")?;
        let subsistence = get_optional_input("Enter the Subsistence instance (or press Enter for 'Subsistence'): ", "Subsistence")?;
        let relatedness = get_optional_input("Enter the Relatedness instance (or press Enter for 'Relatedness'): ", "Relatedness")?;
        let polarity = get_optional_input("Enter the Polarity instance (or press Enter for 'Polarity'): ", "Polarity")?;
        let wholeness = get_optional_input("Enter the Wholeness instance (or press Enter for 'Wholeness'): ", "Wholeness")?;

        let mut dodecad = Dodecad::new(
            &name, &autocracy, &domination, &creativity, &pattern, &individuality, &structure,
            &repetition, &potentiality, &subsistence, &relatedness, &polarity, &wholeness
        );
        
        // Ask if user wants to modify the default connectives
        let modify_connectives = get_yes_no_input("\nWould you like to modify the default connectives? (y/n): ", "y")?;
        
        if modify_connectives.starts_with('y') {
            println!("\nModifying connectives (press Enter to keep default, or input new value):");
            println!("Note: Dodecad has 66 connectives - this will take several moments to review.");
            
            // Iterate through all connectives and allow modification
            let positions = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"];
            let terms = ["autocracy", "domination", "creativity", "pattern", "individuality", "structure", 
                        "repetition", "potentiality", "subsistence", "relatedness", "polarity", "wholeness"];
            
            for i in 0..12 {
                for j in (i + 1)..12 {
                    let current_name = format!("{}{}", positions[i], positions[j]);
                    let prompt = format!("{}: ", current_name);
                    let new_value = get_optional_input(&prompt, &current_name)?;
                    dodecad.connectives.insert((i, j), new_value);
                }
            }
        } else {
            // Keep the defaults that were initialized (no further questions needed)
        }
        
        // Display the created dodecad
        dodecad.display();
        
        // Show connectives if any were defined
        if dodecad.has_connectives() {
            dodecad.display_connectives();
        }
        
        Ok(dodecad)
    }
    
    /// Check if any connectives are defined
    pub fn has_connectives(&self) -> bool {
        !self.connectives.is_empty()
    }
    
    /// Get a connective by position indices
    pub fn get_connective(&self, i: usize, j: usize) -> Option<&String> {
        // Ensure i < j for consistent ordering
        if i < j {
            self.connectives.get(&(i, j))
        } else if j < i {
            self.connectives.get(&(j, i))
        } else {
            None // Same position
        }
    }
    
    /// Set a connective by position indices
    pub fn set_connective(&mut self, i: usize, j: usize, value: String) {
        // Ensure i < j for consistent ordering
        if i < j {
            self.connectives.insert((i, j), value);
        } else if j < i {
            self.connectives.insert((j, i), value);
        }
        // Ignore if i == j (same position)
    }
    
    /// Get the number of connectives
    pub fn connectives_count(&self) -> usize {
        self.connectives.len()
    }
    
    /// Get canonical term names (hardcoded)
    #[allow(dead_code)]
    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec![
            "Autocracy", 
            "Domination", 
            "Creativity", 
            "Pattern", 
            "Individuality", 
            "Structure", 
            "Repetition", 
            "Potentiality", 
            "Subsistence", 
            "Relatedness", 
            "Polarity", 
            "Wholeness"
        ]
    }
    
    /// Get all user instances
    #[allow(dead_code)]
    pub fn get_instances(&self) -> Vec<String> {
        vec![
            self.autocracy.clone(),
            self.domination.clone(),
            self.creativity.clone(),
            self.pattern.clone(),
            self.individuality.clone(),
            self.structure.clone(),
            self.repetition.clone(),
            self.potentiality.clone(),
            self.subsistence.clone(),
            self.relatedness.clone(),
            self.polarity.clone(),
            self.wholeness.clone(),
        ]
    }
    
    /// Display dodecad details
    pub fn display(&self) {
        println!("\n--- Dodecad Details ---");
        println!("Dodecad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("1. Autocracy: {}", self.autocracy);
        println!("2. Domination: {}", self.domination);
        println!("3. Creativity: {}", self.creativity);
        println!("4. Pattern: {}", self.pattern);
        println!("5. Individuality: {}", self.individuality);
        println!("6. Structure: {}", self.structure);
        println!("7. Repetition: {}", self.repetition);
        println!("8. Potentiality: {}", self.potentiality);
        println!("9. Subsistence: {}", self.subsistence);
        println!("10. Relatedness: {}", self.relatedness);
        println!("11. Polarity: {}", self.polarity);
        println!("12. Wholeness: {}", self.wholeness);
        println!("----------------------");
    }
    
    /// Display all connectives
    pub fn display_connectives(&self) {
        println!("\nConnectives:");
        for ((i, j), connective) in &self.connectives {
            let from = ["Autocracy", "Domination", "Creativity", "Pattern", "Individuality", "Structure", "Repetition", "Potentiality", "Subsistence", "Relatedness", "Polarity", "Wholeness"][*i];
            let to = ["Autocracy", "Domination", "Creativity", "Pattern", "Individuality", "Structure", "Repetition", "Potentiality", "Subsistence", "Relatedness", "Polarity", "Wholeness"][*j];
            println!("  {} <--[{}]--> {}", from, connective, to);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dodecad_creation() {
        let dodecad = Dodecad::new(
            "Test Dodecad",
            "Self-Rule", 
            "Control",
            "Innovation",
            "Design",
            "Uniqueness",
            "Framework",
            "Recurrence",
            "Possibility",
            "Existence",
            "Connection",
            "Opposition",
            "Completeness"
        );
        
        assert_eq!(dodecad.name, "Test Dodecad");
        assert_eq!(dodecad.autocracy, "Self-Rule");
        assert_eq!(dodecad.domination, "Control");
        assert_eq!(dodecad.creativity, "Innovation");
        assert_eq!(dodecad.pattern, "Design");
        assert_eq!(dodecad.individuality, "Uniqueness");
        assert_eq!(dodecad.structure, "Framework");
        assert_eq!(dodecad.repetition, "Recurrence");
        assert_eq!(dodecad.potentiality, "Possibility");
        assert_eq!(dodecad.subsistence, "Existence");
        assert_eq!(dodecad.relatedness, "Connection");
        assert_eq!(dodecad.polarity, "Opposition");
        assert_eq!(dodecad.wholeness, "Completeness");
        
        // Should have default connectives
        assert!(dodecad.has_connectives());
    }

    #[test]
    fn test_canonical_terms() {
        let terms = Dodecad::get_canonical_terms();
        assert_eq!(terms, vec![
            "Autocracy", 
            "Domination", 
            "Creativity", 
            "Pattern", 
            "Individuality", 
            "Structure", 
            "Repetition", 
            "Potentiality", 
            "Subsistence", 
            "Relatedness", 
            "Polarity", 
            "Wholeness"
        ]);
        assert_eq!(terms.len(), 12);
    }

    #[test]
    fn test_get_instances() {
        let dodecad = Dodecad::new(
            "Test",
            "Self-Rule", "Control", "Innovation", "Design", "Uniqueness", "Framework",
            "Recurrence", "Possibility", "Existence", "Connection", "Opposition", "Completeness"
        );
        
        let instances = dodecad.get_instances();
        assert_eq!(instances, vec![
            "Self-Rule", "Control", "Innovation", "Design", "Uniqueness", "Framework",
            "Recurrence", "Possibility", "Existence", "Connection", "Opposition", "Completeness"
        ]);
        assert_eq!(instances.len(), 12);
    }

    #[test]
    fn test_term_attribute_description() {
        assert_eq!(Dodecad::TERM_ATTRIBUTE_DESCRIPTION, "Totality");
    }

    #[test]
    fn test_dodecad_with_canonical_defaults() {
        let dodecad = Dodecad::new(
            "Canonical Test",
            "Autocracy", "Domination", "Creativity", "Pattern", "Individuality", "Structure",
            "Repetition", "Potentiality", "Subsistence", "Relatedness", "Polarity", "Wholeness"
        );
        
        let canonical_terms = Dodecad::get_canonical_terms();
        let instances = dodecad.get_instances();
        
        // When using canonical terms, instances should match canonical terms
        for (canonical, instance) in canonical_terms.iter().zip(instances.iter()) {
            assert_eq!(canonical, instance);
        }
    }

    #[test]
    fn test_all_twelve_terms_present() {
        let dodecad = Dodecad::new(
            "Complete Test",
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"
        );
        
        let instances = dodecad.get_instances();
        assert_eq!(instances.len(), 12);
        
        // Verify each position is correctly assigned
        assert_eq!(instances[0], "A");   // Autocracy
        assert_eq!(instances[1], "B");   // Domination
        assert_eq!(instances[2], "C");   // Creativity
        assert_eq!(instances[3], "D");   // Pattern
        assert_eq!(instances[4], "E");   // Individuality
        assert_eq!(instances[5], "F");   // Structure
        assert_eq!(instances[6], "G");   // Repetition
        assert_eq!(instances[7], "H");   // Potentiality
        assert_eq!(instances[8], "I");   // Subsistence
        assert_eq!(instances[9], "J");   // Relatedness
        assert_eq!(instances[10], "K");  // Polarity
        assert_eq!(instances[11], "L");  // Wholeness
    }

    #[test]
    fn test_has_connectives_with_some() {
        let mut dodecad = Dodecad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L");
        dodecad.connectives.insert((0, 1), "custom connection".to_string());
        
        assert!(dodecad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut dodecad = Dodecad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L");
        // Remove all connectives to test none state
        dodecad.connectives.clear();
        
        assert!(!dodecad.has_connectives());
    }

    #[test]
    fn test_positional_semantic_connectives() {
        let dodecad = Dodecad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L");
        
        // Should start with positional-semantic connectives
        assert!(dodecad.has_connectives());
        assert_eq!(dodecad.connectives.get(&(0, 1)).unwrap(), "AB");
        assert_eq!(dodecad.connectives.get(&(2, 3)).unwrap(), "CD");
        assert_eq!(dodecad.connectives.get(&(4, 5)).unwrap(), "EF");
        assert_eq!(dodecad.connectives.get(&(10, 11)).unwrap(), "KL");
    }

    #[test]
    fn test_custom_connectives() {
        let mut dodecad = Dodecad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L");
        
        // Modify one connective while others keep defaults
        dodecad.connectives.insert((0, 1), "custom connection".to_string());
        
        assert!(dodecad.has_connectives());
        assert_eq!(dodecad.connectives.get(&(0, 1)).unwrap(), "custom connection");
        
        // Other connectives should still have defaults
        assert_eq!(dodecad.connectives.get(&(2, 3)).unwrap(), "CD");
        assert_eq!(dodecad.connectives.get(&(4, 5)).unwrap(), "EF");
    }

    #[test]
    fn test_all_connectives_count() {
        let dodecad = Dodecad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L");
        
        // Should have exactly 66 connectives (12 choose 2 = 66)
        let connectives_count = dodecad.connectives.len();
        
        assert_eq!(connectives_count, 66);
    }
} 