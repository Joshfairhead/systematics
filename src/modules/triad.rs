use std::io::{self, Write};

#[derive(Debug)]
pub struct Triad {
    pub name: String,
    pub active: String,
    pub passive: String,
    pub reconciling: String,
}

impl Triad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "dynamism, relation, will";

    /// Creates a new Triad.
    pub fn new(name: &str, active: &str, passive: &str, reconciling: &str) -> Self {
        Triad {
            name: name.to_string(),
            active: active.to_string(),
            passive: passive.to_string(),
            reconciling: reconciling.to_string(),
        }
    }
    
    /// Interactive creation method - handles all input/output internally
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Triad ---");
        
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
        
        // Get inputs with canonical defaults
        let name = get_optional_input("Enter a name for your Triad (or press Enter for 'Unnamed Triad'): ", "Unnamed Triad")?;
        let active = get_optional_input("Enter the Active instance (or press Enter for 'Active'): ", "Active")?;
        let passive = get_optional_input("Enter the Passive instance (or press Enter for 'Passive'): ", "Passive")?;
        let reconciling = get_optional_input("Enter the Reconciling instance (or press Enter for 'Reconciling'): ", "Reconciling")?;

        let triad = Triad::new(&name, &active, &passive, &reconciling);
        
        // Display the created triad
        triad.display();
        
        Ok(triad)
    }
    
    /// Check if triad has valid terms defined
    #[allow(dead_code)]
    pub fn has_terms(&self) -> bool {
        !self.active.is_empty() && !self.passive.is_empty() && !self.reconciling.is_empty()
    }
    
    /// Get canonical terms for Triad
    #[allow(dead_code)]
    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec!["Active", "Passive", "Reconciling"]
    }
    
    /// Get user instances for all canonical positions
    #[allow(dead_code)]
    pub fn get_instances(&self) -> Vec<String> {
        vec![self.active.clone(), self.passive.clone(), self.reconciling.clone()]
    }
    
    /// Display triad details
    pub fn display(&self) {
        println!("\n--- Triad Details ---");
        println!("Triad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("Active: {}", self.active);
        println!("Passive: {}", self.passive);
        println!("Reconciling: {}", self.reconciling);
        println!("---------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad_creation() {
        let triad = Triad::new(
            "Test Triad",
            "Action", 
            "Reflection",
            "Balance"
        );
        
        assert_eq!(triad.name, "Test Triad");
        assert_eq!(triad.active, "Action");
        assert_eq!(triad.passive, "Reflection");
        assert_eq!(triad.reconciling, "Balance");
    }

    #[test]
    fn test_triad_creation_with_empty_name() {
        let triad = Triad::new("", "Active", "Passive", "Reconciling");
        
        assert_eq!(triad.name, "");
        assert_eq!(triad.active, "Active");
        assert_eq!(triad.passive, "Passive");
        assert_eq!(triad.reconciling, "Reconciling");
    }

    #[test]
    fn test_canonical_terms() {
        let terms = Triad::get_canonical_terms();
        assert_eq!(terms, vec!["Active", "Passive", "Reconciling"]);
    }

    #[test]
    fn test_get_instances() {
        let triad = Triad::new("Test", "Force", "Resistance", "Balance");
        let instances = triad.get_instances();
        
        assert_eq!(instances.len(), 3);
        assert_eq!(instances[0], "Force");
        assert_eq!(instances[1], "Resistance");
        assert_eq!(instances[2], "Balance");
    }

    #[test]
    fn test_has_terms_with_content() {
        let triad = Triad::new("Test", "Active", "Passive", "Reconciling");
        assert!(triad.has_terms());
    }

    #[test]
    fn test_has_terms_with_empty_active() {
        let triad = Triad::new("Test", "", "Passive", "Reconciling");
        assert!(!triad.has_terms());
    }

    #[test]
    fn test_has_terms_with_empty_passive() {
        let triad = Triad::new("Test", "Active", "", "Reconciling");
        assert!(!triad.has_terms());
    }

    #[test]
    fn test_has_terms_with_empty_reconciling() {
        let triad = Triad::new("Test", "Active", "Passive", "");
        assert!(!triad.has_terms());
    }

    #[test]
    fn test_has_terms_with_all_empty() {
        let triad = Triad::new("Test", "", "", "");
        assert!(!triad.has_terms());
    }

    #[test]
    fn test_term_attribute_description() {
        assert_eq!(Triad::TERM_ATTRIBUTE_DESCRIPTION, "dynamism, relation, will");
    }

    #[test]
    fn test_triad_with_special_characters() {
        let triad = Triad::new("Test-Triad!", "Force (Primary)", "Resistance, Universal", "Balance!");
        
        assert_eq!(triad.name, "Test-Triad!");
        assert_eq!(triad.active, "Force (Primary)");
        assert_eq!(triad.passive, "Resistance, Universal");
        assert_eq!(triad.reconciling, "Balance!");
    }

    #[test]
    fn test_triad_with_unicode() {
        let triad = Triad::new("测试", "主动", "被动", "调和");
        
        assert_eq!(triad.name, "测试");
        assert_eq!(triad.active, "主动");
        assert_eq!(triad.passive, "被动");
        assert_eq!(triad.reconciling, "调和");
    }

    #[test]
    fn test_triad_clone_and_modify() {
        let original = Triad::new("Original", "Original Active", "Original Passive", "Original Reconciling");
        let mut modified = Triad::new(&original.name, &original.active, &original.passive, &original.reconciling);
        
        modified.name = "Modified".to_string();
        modified.active = "Modified Active".to_string();
        modified.passive = "Modified Passive".to_string();
        modified.reconciling = "Modified Reconciling".to_string();
        
        // Original should be unchanged
        assert_eq!(original.name, "Original");
        assert_eq!(original.active, "Original Active");
        assert_eq!(original.passive, "Original Passive");
        assert_eq!(original.reconciling, "Original Reconciling");
        
        // Modified should be changed
        assert_eq!(modified.name, "Modified");
        assert_eq!(modified.active, "Modified Active");
        assert_eq!(modified.passive, "Modified Passive");
        assert_eq!(modified.reconciling, "Modified Reconciling");
    }

    #[test]
    fn test_triad_debug_format() {
        let triad = Triad::new("Debug Test", "Debug Active", "Debug Passive", "Debug Reconciling");
        let debug_str = format!("{:?}", triad);
        
        assert!(debug_str.contains("Debug Test"));
        assert!(debug_str.contains("Debug Active"));
        assert!(debug_str.contains("Debug Passive"));
        assert!(debug_str.contains("Debug Reconciling"));
    }

    #[test]
    fn test_canonical_terms_immutable() {
        let terms1 = Triad::get_canonical_terms();
        let terms2 = Triad::get_canonical_terms();
        
        assert_eq!(terms1, terms2);
        assert_eq!(terms1.len(), 3);
    }

    #[test]
    fn test_get_instances_order() {
        let triad = Triad::new("Order Test", "First Term", "Second Term", "Third Term");
        let instances = triad.get_instances();
        
        assert_eq!(instances.len(), 3);
        assert_eq!(instances[0], "First Term");
        assert_eq!(instances[1], "Second Term");
        assert_eq!(instances[2], "Third Term");
    }

    #[test]
    fn test_triad_construction_consistency() {
        let name = "Consistency Test";
        let active = "Consistent Active";
        let passive = "Consistent Passive";
        let reconciling = "Consistent Reconciling";
        
        let triad = Triad::new(name, active, passive, reconciling);
        
        assert_eq!(triad.name, name);
        assert_eq!(triad.active, active);
        assert_eq!(triad.passive, passive);
        assert_eq!(triad.reconciling, reconciling);
        
        let instances = triad.get_instances();
        assert_eq!(instances[0], active);
        assert_eq!(instances[1], passive);
        assert_eq!(instances[2], reconciling);
    }

    #[test]
    fn test_empty_string_handling() {
        let triad = Triad::new("", "", "", "");
        
        assert_eq!(triad.name, "");
        assert_eq!(triad.active, "");
        assert_eq!(triad.passive, "");
        assert_eq!(triad.reconciling, "");
        assert!(!triad.has_terms());
        
        let instances = triad.get_instances();
        assert_eq!(instances.len(), 3);
        assert_eq!(instances[0], "");
        assert_eq!(instances[1], "");
        assert_eq!(instances[2], "");
    }

    #[test]
    fn test_triad_dynamism_concepts() {
        let triad = Triad::new("Force Test", "Action", "Reaction", "Equilibrium");
        
        assert_eq!(triad.active, "Action");
        assert_eq!(triad.passive, "Reaction");
        assert_eq!(triad.reconciling, "Equilibrium");
        assert!(triad.has_terms());
        
        let triad2 = Triad::new("Will Test", "Intention", "Resistance", "Resolution");
        assert_eq!(triad2.active, "Intention");
        assert_eq!(triad2.passive, "Resistance");
        assert_eq!(triad2.reconciling, "Resolution");
    }

    #[test]
    fn test_term_attribute_description_consistency() {
        // Test that the description matches Bennett's terminology for triads
        let description = Triad::TERM_ATTRIBUTE_DESCRIPTION;
        assert!(description.contains("dynamism") || description.contains("relation") || description.contains("will"));
    }

    #[test]
    fn test_canonical_defaults_behavior() {
        // Test that canonical terms can be used as defaults
        let triad = Triad::new("Default Test", "Active", "Passive", "Reconciling");
        
        assert_eq!(triad.active, "Active");
        assert_eq!(triad.passive, "Passive");
        assert_eq!(triad.reconciling, "Reconciling");
        
        // Verify these match the canonical terms
        let canonical = Triad::get_canonical_terms();
        assert_eq!(triad.active, canonical[0]);
        assert_eq!(triad.passive, canonical[1]);
        assert_eq!(triad.reconciling, canonical[2]);
    }
}
