use std::io::{self, Write};

#[derive(Debug)]
#[allow(non_snake_case)] // Connective fields use intentional positional-semantic naming
pub struct Hexad {
    pub name: String,
    // User instances for canonical positions
    pub resources: String,     // Position A
    pub values: String,        // Position B
    pub options: String,       // Position C
    pub criteria: String,      // Position D
    pub facts: String,         // Position E
    pub priorities: String,    // Position F
    // Positional-semantic connectives (bidirectional relationships)
    pub AB_resources_values: Option<String>,      // A<>B
    pub AC_resources_options: Option<String>,     // A<>C
    pub AD_resources_criteria: Option<String>,    // A<>D
    pub AE_resources_facts: Option<String>,       // A<>E
    pub AF_resources_priorities: Option<String>,  // A<>F
    pub BC_values_options: Option<String>,        // B<>C
    pub BD_values_criteria: Option<String>,       // B<>D
    pub BE_values_facts: Option<String>,          // B<>E
    pub BF_values_priorities: Option<String>,     // B<>F
    pub CD_options_criteria: Option<String>,      // C<>D
    pub CE_options_facts: Option<String>,         // C<>E
    pub CF_options_priorities: Option<String>,    // C<>F
    pub DE_criteria_facts: Option<String>,        // D<>E
    pub DF_criteria_priorities: Option<String>,   // D<>F
    pub EF_facts_priorities: Option<String>,      // E<>F
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
        Hexad {
            name: name.to_string(),
            resources: resources.to_string(),
            values: values.to_string(),
            options: options.to_string(),
            criteria: criteria.to_string(),
            facts: facts.to_string(),
            priorities: priorities.to_string(),
            // Initialize connectives with positional-semantic defaults
            AB_resources_values: Some("AB_resources_values".to_string()),
            AC_resources_options: Some("AC_resources_options".to_string()),
            AD_resources_criteria: Some("AD_resources_criteria".to_string()),
            AE_resources_facts: Some("AE_resources_facts".to_string()),
            AF_resources_priorities: Some("AF_resources_priorities".to_string()),
            BC_values_options: Some("BC_values_options".to_string()),
            BD_values_criteria: Some("BD_values_criteria".to_string()),
            BE_values_facts: Some("BE_values_facts".to_string()),
            BF_values_priorities: Some("BF_values_priorities".to_string()),
            CD_options_criteria: Some("CD_options_criteria".to_string()),
            CE_options_facts: Some("CE_options_facts".to_string()),
            CF_options_priorities: Some("CF_options_priorities".to_string()),
            DE_criteria_facts: Some("DE_criteria_facts".to_string()),
            DF_criteria_priorities: Some("DF_criteria_priorities".to_string()),
            EF_facts_priorities: Some("EF_facts_priorities".to_string()),
        }
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
            println!("Note: Hexad has 15 connectives - this may take a moment to review.");
            
            // Helper to handle connective modification
            let modify_connective = |prompt: &str, current: &str| -> Result<Option<String>, Box<dyn std::error::Error>> {
                let input = get_optional_input(prompt, current)?;
                Ok(Some(input))
            };
            
            hexad.AB_resources_values = modify_connective("AB_resources_values: ", "AB_resources_values")?;
            hexad.AC_resources_options = modify_connective("AC_resources_options: ", "AC_resources_options")?;
            hexad.AD_resources_criteria = modify_connective("AD_resources_criteria: ", "AD_resources_criteria")?;
            hexad.AE_resources_facts = modify_connective("AE_resources_facts: ", "AE_resources_facts")?;
            hexad.AF_resources_priorities = modify_connective("AF_resources_priorities: ", "AF_resources_priorities")?;
            hexad.BC_values_options = modify_connective("BC_values_options: ", "BC_values_options")?;
            hexad.BD_values_criteria = modify_connective("BD_values_criteria: ", "BD_values_criteria")?;
            hexad.BE_values_facts = modify_connective("BE_values_facts: ", "BE_values_facts")?;
            hexad.BF_values_priorities = modify_connective("BF_values_priorities: ", "BF_values_priorities")?;
            hexad.CD_options_criteria = modify_connective("CD_options_criteria: ", "CD_options_criteria")?;
            hexad.CE_options_facts = modify_connective("CE_options_facts: ", "CE_options_facts")?;
            hexad.CF_options_priorities = modify_connective("CF_options_priorities: ", "CF_options_priorities")?;
            hexad.DE_criteria_facts = modify_connective("DE_criteria_facts: ", "DE_criteria_facts")?;
            hexad.DF_criteria_priorities = modify_connective("DF_criteria_priorities: ", "DF_criteria_priorities")?;
            hexad.EF_facts_priorities = modify_connective("EF_facts_priorities: ", "EF_facts_priorities")?;
        } else {
            // Keep the defaults that were initialized (no further questions needed)
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
        self.AB_resources_values.is_some() ||
        self.AC_resources_options.is_some() ||
        self.AD_resources_criteria.is_some() ||
        self.AE_resources_facts.is_some() ||
        self.AF_resources_priorities.is_some() ||
        self.BC_values_options.is_some() ||
        self.BD_values_criteria.is_some() ||
        self.BE_values_facts.is_some() ||
        self.BF_values_priorities.is_some() ||
        self.CD_options_criteria.is_some() ||
        self.CE_options_facts.is_some() ||
        self.CF_options_priorities.is_some() ||
        self.DE_criteria_facts.is_some() ||
        self.DF_criteria_priorities.is_some() ||
        self.EF_facts_priorities.is_some()
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
        let connectives = [
            (&self.resources, &self.values, &self.AB_resources_values, "A<>B"),
            (&self.resources, &self.options, &self.AC_resources_options, "A<>C"),
            (&self.resources, &self.criteria, &self.AD_resources_criteria, "A<>D"),
            (&self.resources, &self.facts, &self.AE_resources_facts, "A<>E"),
            (&self.resources, &self.priorities, &self.AF_resources_priorities, "A<>F"),
            (&self.values, &self.options, &self.BC_values_options, "B<>C"),
            (&self.values, &self.criteria, &self.BD_values_criteria, "B<>D"),
            (&self.values, &self.facts, &self.BE_values_facts, "B<>E"),
            (&self.values, &self.priorities, &self.BF_values_priorities, "B<>F"),
            (&self.options, &self.criteria, &self.CD_options_criteria, "C<>D"),
            (&self.options, &self.facts, &self.CE_options_facts, "C<>E"),
            (&self.options, &self.priorities, &self.CF_options_priorities, "C<>F"),
            (&self.criteria, &self.facts, &self.DE_criteria_facts, "D<>E"),
            (&self.criteria, &self.priorities, &self.DF_criteria_priorities, "D<>F"),
            (&self.facts, &self.priorities, &self.EF_facts_priorities, "E<>F"),
        ];
        
        for (from, to, connective, code) in connectives {
            match connective {
                Some(conn) => println!("  {} <--[{}]--> {} ({})", from, conn, to, code),
                None => println!("  {} <--> {} (no connective defined) ({})", from, to, code),
            }
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
            "Money", 
            "Quality",
            "Choices",
            "Standards",
            "Data",
            "Goals"
        );
        
        assert_eq!(hexad.name, "Test Hexad");
        assert_eq!(hexad.resources, "Money");
        assert_eq!(hexad.values, "Quality");
        assert_eq!(hexad.options, "Choices");
        assert_eq!(hexad.criteria, "Standards");
        assert_eq!(hexad.facts, "Data");
        assert_eq!(hexad.priorities, "Goals");
        
        // Should have default connectives
        assert!(hexad.has_connectives());
        assert_eq!(hexad.AB_resources_values.unwrap(), "AB_resources_values");
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
            "Money", 
            "Quality",
            "Choices",
            "Standards",
            "Data",
            "Goals"
        );
        
        let instances = hexad.get_instances();
        assert_eq!(instances, vec!["Money", "Quality", "Choices", "Standards", "Data", "Goals"]);
    }

    #[test]
    fn test_has_connectives_with_some() {
        let mut hexad = Hexad::new("Test", "R", "V", "O", "C", "F", "P");
        hexad.AB_resources_values = Some("test connection".to_string());
        
        assert!(hexad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut hexad = Hexad::new("Test", "R", "V", "O", "C", "F", "P");
        // Remove all connectives to test none state
        hexad.AB_resources_values = None;
        hexad.AC_resources_options = None;
        hexad.AD_resources_criteria = None;
        hexad.AE_resources_facts = None;
        hexad.AF_resources_priorities = None;
        hexad.BC_values_options = None;
        hexad.BD_values_criteria = None;
        hexad.BE_values_facts = None;
        hexad.BF_values_priorities = None;
        hexad.CD_options_criteria = None;
        hexad.CE_options_facts = None;
        hexad.CF_options_priorities = None;
        hexad.DE_criteria_facts = None;
        hexad.DF_criteria_priorities = None;
        hexad.EF_facts_priorities = None;
        
        assert!(!hexad.has_connectives());
    }

    #[test]
    fn test_positional_semantic_connectives() {
        let hexad = Hexad::new("Test", "R", "V", "O", "C", "F", "P");
        
        // Should start with positional-semantic connectives
        assert!(hexad.has_connectives());
        assert_eq!(hexad.AB_resources_values.unwrap(), "AB_resources_values");
        assert_eq!(hexad.BC_values_options.unwrap(), "BC_values_options");
        assert_eq!(hexad.DE_criteria_facts.unwrap(), "DE_criteria_facts");
        assert_eq!(hexad.EF_facts_priorities.unwrap(), "EF_facts_priorities");
    }

    #[test]
    fn test_custom_connectives() {
        let mut hexad = Hexad::new("Test", "R", "V", "O", "C", "F", "P");
        
        // Modify one connective while others keep defaults
        hexad.AB_resources_values = Some("custom connection".to_string());
        
        assert!(hexad.has_connectives());
        assert_eq!(hexad.AB_resources_values.unwrap(), "custom connection");
        
        // Other connectives should still have defaults
        assert_eq!(hexad.BC_values_options.unwrap(), "BC_values_options");
        assert_eq!(hexad.DE_criteria_facts.unwrap(), "DE_criteria_facts");
    }

    #[test]
    fn test_all_connectives_count() {
        let hexad = Hexad::new("Test", "R", "V", "O", "C", "F", "P");
        
        // Should have exactly 15 connectives (6 choose 2 = 15)
        let connectives_count = [
            hexad.AB_resources_values.is_some(),
            hexad.AC_resources_options.is_some(),
            hexad.AD_resources_criteria.is_some(),
            hexad.AE_resources_facts.is_some(),
            hexad.AF_resources_priorities.is_some(),
            hexad.BC_values_options.is_some(),
            hexad.BD_values_criteria.is_some(),
            hexad.BE_values_facts.is_some(),
            hexad.BF_values_priorities.is_some(),
            hexad.CD_options_criteria.is_some(),
            hexad.CE_options_facts.is_some(),
            hexad.CF_options_priorities.is_some(),
            hexad.DE_criteria_facts.is_some(),
            hexad.DF_criteria_priorities.is_some(),
            hexad.EF_facts_priorities.is_some(),
        ].iter().filter(|&&x| x).count();
        
        assert_eq!(connectives_count, 15);
    }
} 