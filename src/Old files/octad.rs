use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(non_snake_case)] // Connective fields use intentional positional-semantic naming
pub struct Octad {
    pub name: String,
    // User instances for canonical positions
    pub smallest_significant_holon: String, // Position A
    pub critical_functions: String,         // Position B
    pub supportive_platform: String,        // Position C
    pub necessary_resourcing: String,       // Position D
    pub integrative_totality: String,       // Position E
    pub inherent_values: String,            // Position F
    pub intrinsic_nature: String,           // Position G
    pub organisational_modes: String,       // Position H
    // Connectives stored as HashMap for scalability
    pub connectives: HashMap<(usize, usize), String>,
}

impl Octad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Completedness";

    /// Creates a new Octad with user instances for canonical positions
    pub fn new(
        name: &str,
        smallest_significant_holon: &str,
        critical_functions: &str,
        supportive_platform: &str,
        necessary_resourcing: &str,
        integrative_totality: &str,
        inherent_values: &str,
        intrinsic_nature: &str,
        organisational_modes: &str,
    ) -> Self {
        let mut octad = Octad {
            name: name.to_string(),
            smallest_significant_holon: smallest_significant_holon.to_string(),
            critical_functions: critical_functions.to_string(),
            supportive_platform: supportive_platform.to_string(),
            necessary_resourcing: necessary_resourcing.to_string(),
            integrative_totality: integrative_totality.to_string(),
            inherent_values: inherent_values.to_string(),
            intrinsic_nature: intrinsic_nature.to_string(),
            organisational_modes: organisational_modes.to_string(),
            connectives: HashMap::new(),
        };

        // Initialize all 28 connectives with positional-semantic defaults
        let terms = vec![
            "smallest", "critical", "supportive", "necessary", 
            "integrative", "inherent", "intrinsic", "organisational"
        ];
        
        for i in 0..8 {
            for j in (i + 1)..8 {
                let connective_name = format!("{}_{}_{}_{}", 
                    if i < j { char::from(b'A' + i as u8) } else { char::from(b'A' + j as u8) },
                    if i < j { char::from(b'A' + j as u8) } else { char::from(b'A' + i as u8) },
                    terms[i], 
                    terms[j]
                );
                octad.connectives.insert((i, j), connective_name);
            }
        }

        octad
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
        println!("\n--- Creating an Octad ---");
        
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
        let name = get_optional_input("Enter a name for your Octad (or press Enter for 'Unnamed Octad'): ", "Unnamed Octad")?;
        let smallest_significant_holon = get_optional_input("Enter the Smallest Significant Holon instance (or press Enter for 'Smallest Significant Holon'): ", "Smallest Significant Holon")?;
        let critical_functions = get_optional_input("Enter the Critical Functions instance (or press Enter for 'Critical Functions'): ", "Critical Functions")?;
        let supportive_platform = get_optional_input("Enter the Supportive Platform instance (or press Enter for 'Supportive Platform'): ", "Supportive Platform")?;
        let necessary_resourcing = get_optional_input("Enter the Necessary Resourcing instance (or press Enter for 'Necessary Resourcing'): ", "Necessary Resourcing")?;
        let integrative_totality = get_optional_input("Enter the Integrative Totality instance (or press Enter for 'Integrative Totality'): ", "Integrative Totality")?;
        let inherent_values = get_optional_input("Enter the Inherent Values instance (or press Enter for 'Inherent Values'): ", "Inherent Values")?;
        let intrinsic_nature = get_optional_input("Enter the Intrinsic Nature instance (or press Enter for 'Intrinsic Nature'): ", "Intrinsic Nature")?;
        let organisational_modes = get_optional_input("Enter the Organisational Modes instance (or press Enter for 'Organisational Modes'): ", "Organisational Modes")?;

        let mut octad = Octad::new(&name, &smallest_significant_holon, &critical_functions, &supportive_platform, &necessary_resourcing, &integrative_totality, &inherent_values, &intrinsic_nature, &organisational_modes);
        
        // Ask if user wants to modify the default connectives
        let modify_connectives = get_yes_no_input("\nWould you like to modify the default connectives? (y/n): ", "y")?;
        
        if modify_connectives.starts_with('y') {
            println!("\nModifying connectives (press Enter to keep default, or input new value):");
            println!("Note: Octad has 28 connectives - this will take several moments to review.");
            
            // Get all connective keys and sort them for consistent ordering
            let mut keys: Vec<_> = octad.connectives.keys().cloned().collect();
            keys.sort();
            
            for (i, j) in keys {
                let current_value = octad.connectives.get(&(i, j)).unwrap();
                let prompt = format!("Connective {}<>{} ({}): ", 
                    char::from(b'A' + i as u8), 
                    char::from(b'A' + j as u8),
                    current_value
                );
                
                let new_value = get_optional_input(&prompt, current_value)?;
                octad.set_connective(i, j, new_value);
            }
        }
        
        // Display the created octad
        octad.display();
        
        // Show connectives if any were defined
        if octad.has_connectives() {
            octad.display_connectives();
        }
        
        Ok(octad)
    }
    
    /// Check if any connectives are defined
    pub fn has_connectives(&self) -> bool {
        !self.connectives.is_empty()
    }
    
    /// Get canonical term names (hardcoded)
    #[allow(dead_code)]
    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec![
            "Smallest Significant Holon", 
            "Critical Functions", 
            "Supportive Platform", 
            "Necessary Resourcing", 
            "Integrative Totality", 
            "Inherent Values", 
            "Intrinsic Nature", 
            "Organisational Modes"
        ]
    }
    
    /// Get all user instances
    #[allow(dead_code)]
    pub fn get_instances(&self) -> Vec<String> {
        vec![
            self.smallest_significant_holon.clone(),
            self.critical_functions.clone(),
            self.supportive_platform.clone(),
            self.necessary_resourcing.clone(),
            self.integrative_totality.clone(),
            self.inherent_values.clone(),
            self.intrinsic_nature.clone(),
            self.organisational_modes.clone(),
        ]
    }
    
    /// Display octad details
    pub fn display(&self) {
        println!("\n--- Octad Details ---");
        println!("Octad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("A (Smallest Significant Holon): {}", self.smallest_significant_holon);
        println!("B (Critical Functions): {}", self.critical_functions);
        println!("C (Supportive Platform): {}", self.supportive_platform);
        println!("D (Necessary Resourcing): {}", self.necessary_resourcing);
        println!("E (Integrative Totality): {}", self.integrative_totality);
        println!("F (Inherent Values): {}", self.inherent_values);
        println!("G (Intrinsic Nature): {}", self.intrinsic_nature);
        println!("H (Organisational Modes): {}", self.organisational_modes);
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
    fn test_octad_creation() {
        let octad = Octad::new(
            "Test Octad",
            "Core Element", 
            "Key Processes",
            "Foundation",
            "Resources",
            "Whole System",
            "Core Values",
            "Essential Nature",
            "Operating Modes"
        );
        
        assert_eq!(octad.name, "Test Octad");
        assert_eq!(octad.smallest_significant_holon, "Core Element");
        assert_eq!(octad.critical_functions, "Key Processes");
        assert_eq!(octad.supportive_platform, "Foundation");
        assert_eq!(octad.necessary_resourcing, "Resources");
        assert_eq!(octad.integrative_totality, "Whole System");
        assert_eq!(octad.inherent_values, "Core Values");
        assert_eq!(octad.intrinsic_nature, "Essential Nature");
        assert_eq!(octad.organisational_modes, "Operating Modes");
        
        // Should have default connectives
        assert!(octad.has_connectives());
        assert_eq!(octad.connectives_count(), 28);
    }

    #[test]
    fn test_canonical_terms() {
        let terms = Octad::get_canonical_terms();
        assert_eq!(terms, vec![
            "Smallest Significant Holon", 
            "Critical Functions", 
            "Supportive Platform", 
            "Necessary Resourcing", 
            "Integrative Totality", 
            "Inherent Values", 
            "Intrinsic Nature", 
            "Organisational Modes"
        ]);
    }

    #[test]
    fn test_get_instances() {
        let octad = Octad::new(
            "Test",
            "Core Element", 
            "Key Processes",
            "Foundation",
            "Resources",
            "Whole System",
            "Core Values",
            "Essential Nature",
            "Operating Modes"
        );
        
        let instances = octad.get_instances();
        assert_eq!(instances, vec![
            "Core Element", "Key Processes", "Foundation", "Resources", 
            "Whole System", "Core Values", "Essential Nature", "Operating Modes"
        ]);
    }

    #[test]
    fn test_has_connectives_with_some() {
        let octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        assert!(octad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        octad.connectives.clear();
        assert!(!octad.has_connectives());
    }

    #[test]
    fn test_positional_semantic_connectives() {
        let octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        
        // Should start with positional-semantic connectives
        assert!(octad.has_connectives());
        assert_eq!(octad.get_connective(0, 1).unwrap(), "A_B_smallest_critical");
        assert_eq!(octad.get_connective(2, 3).unwrap(), "C_D_supportive_necessary");
        assert_eq!(octad.get_connective(4, 5).unwrap(), "E_F_integrative_inherent");
        assert_eq!(octad.get_connective(6, 7).unwrap(), "G_H_intrinsic_organisational");
    }

    #[test]
    fn test_custom_connectives() {
        let mut octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        
        // Modify one connective while others keep defaults
        octad.set_connective(0, 1, "custom connection".to_string());
        
        assert!(octad.has_connectives());
        assert_eq!(octad.get_connective(0, 1).unwrap(), "custom connection");
        
        // Other connectives should still have defaults
        assert_eq!(octad.get_connective(2, 3).unwrap(), "C_D_supportive_necessary");
        assert_eq!(octad.get_connective(4, 5).unwrap(), "E_F_integrative_inherent");
    }

    #[test]
    fn test_all_connectives_count() {
        let octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        
        // Should have exactly 28 connectives (8 choose 2 = 28)
        assert_eq!(octad.connectives_count(), 28);
    }

    #[test]
    fn test_connective_helper_methods() {
        let mut octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        
        // Test get/set connective methods
        assert!(octad.get_connective(0, 1).is_some());
        
        octad.set_connective(0, 1, "new value".to_string());
        assert_eq!(octad.get_connective(0, 1).unwrap(), "new value");
        
        // Test bidirectional access (should work both ways)
        assert_eq!(octad.get_connective(1, 0).unwrap(), "new value");
    }
} 