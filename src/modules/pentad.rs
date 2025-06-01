use std::io::{self, Write};

#[derive(Debug)]
pub struct Pentad {
    pub name: String,
    // User instances for canonical positions (Bennett's authentic terms)
    pub quintessence: String,      // A - was intrinsiclimit
    pub higher_potential: String,  // B - was innerupperlimit
    pub lower_potential: String,   // C - was innerlowerlimit
    pub purpose: String,           // D - was outerupperlimit
    pub source: String,            // E - was outerlowerlimit
    // Canonical connectives (Bennett's authentic relationships)
    pub bc_range_of_potential: Option<String>,     // B<>C: Range of potential
    pub de_range_of_significance: Option<String>,  // D<>E: Range of significance
    pub ab_aspiration: Option<String>,             // A<>B: Aspiration
    pub ac_operation: Option<String>,              // A<>C: Operation
    pub bd_output: Option<String>,                 // B<>D: Output
    pub ce_input: Option<String>,                  // C<>E: Input
    pub ad_inspiration: Option<String>,            // A<>D: Inspiration
    pub ae_quantitive_match: Option<String>,       // A<>E: Quantitive match
    pub cd_form: Option<String>,                   // C<>D: Form
    pub be_function: Option<String>,               // B<>E: Function
}

impl Pentad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Quintessence or significance";

    /// Creates a new Pentad with user instances for canonical positions
    pub fn new(
        name: &str,
        quintessence: &str,
        higher_potential: &str,
        lower_potential: &str,
        purpose: &str,
        source: &str,
    ) -> Self {
        Pentad {
            name: name.to_string(),
            quintessence: quintessence.to_string(),
            higher_potential: higher_potential.to_string(),
            lower_potential: lower_potential.to_string(),
            purpose: purpose.to_string(),
            source: source.to_string(),
            // Initialize connectives with Bennett's canonical defaults
            bc_range_of_potential: Some("Range of potential".to_string()),
            de_range_of_significance: Some("Range of significance".to_string()),
            ab_aspiration: Some("Aspiration".to_string()),
            ac_operation: Some("Operation".to_string()),
            bd_output: Some("Output".to_string()),
            ce_input: Some("Input".to_string()),
            ad_inspiration: Some("Inspiration".to_string()),
            ae_quantitive_match: Some("Quantitive match".to_string()),
            cd_form: Some("Form".to_string()),
            be_function: Some("Function".to_string()),
        }
    }
    
    /// Interactive creation method - handles all input/output internally
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Pentad ---");
        
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
        
        // Get required inputs using Bennett's authentic terms
        let name = get_optional_input("Enter a name for your Pentad (or press Enter for 'Unnamed Pentad'): ", "Unnamed Pentad")?;
        let quintessence = get_optional_input("Enter the Quintessence (A) instance (or press Enter for 'Quintessence'): ", "Quintessence")?;
        let higher_potential = get_optional_input("Enter the Higher Potential (B) instance (or press Enter for 'Higher Potential'): ", "Higher Potential")?;
        let lower_potential = get_optional_input("Enter the Lower Potential (C) instance (or press Enter for 'Lower Potential'): ", "Lower Potential")?;
        let purpose = get_optional_input("Enter the Purpose (D) instance (or press Enter for 'Purpose'): ", "Purpose")?;
        let source = get_optional_input("Enter the Source (E) instance (or press Enter for 'Source'): ", "Source")?;

        let mut pentad = Pentad::new(&name, &quintessence, &higher_potential, &lower_potential, &purpose, &source);
        
        // Ask if user wants to modify the default connectives
        let modify_connectives = get_yes_no_input("\nWould you like to modify the default connectives? (y/n): ", "y")?;
        
        if modify_connectives.starts_with('y') {
            println!("\nModifying connectives (press Enter to keep default, or input new value):");
            
            // Helper to handle connective modification
            let modify_connective = |prompt: &str, current: &str| -> Result<Option<String>, Box<dyn std::error::Error>> {
                let input = get_optional_input(prompt, current)?;
                Ok(Some(input))
            };
            
            pentad.bc_range_of_potential = modify_connective(
                "Range of potential (B<>C): ", 
                "Range of potential"
            )?;
            
            pentad.de_range_of_significance = modify_connective(
                "Range of significance (D<>E): ", 
                "Range of significance"
            )?;
            
            pentad.ab_aspiration = modify_connective(
                "Aspiration (A<>B): ", 
                "Aspiration"
            )?;
            
            pentad.ac_operation = modify_connective(
                "Operation (A<>C): ", 
                "Operation"
            )?;
            
            pentad.bd_output = modify_connective(
                "Output (B<>D): ", 
                "Output"
            )?;
            
            pentad.ce_input = modify_connective(
                "Input (C<>E): ", 
                "Input"
            )?;
            
            pentad.ad_inspiration = modify_connective(
                "Inspiration (A<>D): ", 
                "Inspiration"
            )?;
            
            pentad.ae_quantitive_match = modify_connective(
                "Quantitive match (A<>E): ", 
                "Quantitive match"
            )?;
            
            pentad.cd_form = modify_connective(
                "Form (C<>D): ", 
                "Form"
            )?;
            
            pentad.be_function = modify_connective(
                "Function (B<>E): ", 
                "Function"
            )?;
        } else {
            // Keep the defaults that were initialized (no further questions needed)
        }
        
        // Display the created pentad
        pentad.display();
        
        // Show connectives if any were defined
        if pentad.has_connectives() {
            pentad.display_connectives();
        }
        
        Ok(pentad)
    }
    
    /// Check if any connectives are defined
    pub fn has_connectives(&self) -> bool {
        self.bc_range_of_potential.is_some() ||
        self.de_range_of_significance.is_some() ||
        self.ab_aspiration.is_some() ||
        self.ac_operation.is_some() ||
        self.bd_output.is_some() ||
        self.ce_input.is_some() ||
        self.ad_inspiration.is_some() ||
        self.ae_quantitive_match.is_some() ||
        self.cd_form.is_some() ||
        self.be_function.is_some()
    }
    
    /// Get canonical term names (hardcoded - Bennett's authentic terms)
    #[allow(dead_code)]
    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec!["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"]
    }
    
    /// Get all user instances
    #[allow(dead_code)]
    pub fn get_instances(&self) -> Vec<String> {
        vec![
            self.quintessence.clone(),
            self.higher_potential.clone(),
            self.lower_potential.clone(),
            self.purpose.clone(),
            self.source.clone(),
        ]
    }
    
    /// Display pentad details
    pub fn display(&self) {
        println!("\n--- Pentad Details ---");
        println!("Pentad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("Quintessence (A): {}", self.quintessence);
        println!("Higher Potential (B): {}", self.higher_potential);
        println!("Lower Potential (C): {}", self.lower_potential);
        println!("Purpose (D): {}", self.purpose);
        println!("Source (E): {}", self.source);
        println!("---------------------");
    }
    
    /// Display all connectives
    pub fn display_connectives(&self) {
        println!("\nConnectives:");
        let connectives = [
            (&self.higher_potential, &self.lower_potential, &self.bc_range_of_potential, "B<>C"),
            (&self.purpose, &self.source, &self.de_range_of_significance, "D<>E"),
            (&self.quintessence, &self.higher_potential, &self.ab_aspiration, "A<>B"),
            (&self.quintessence, &self.lower_potential, &self.ac_operation, "A<>C"),
            (&self.higher_potential, &self.purpose, &self.bd_output, "B<>D"),
            (&self.lower_potential, &self.source, &self.ce_input, "C<>E"),
            (&self.quintessence, &self.purpose, &self.ad_inspiration, "A<>D"),
            (&self.quintessence, &self.source, &self.ae_quantitive_match, "A<>E"),
            (&self.lower_potential, &self.purpose, &self.cd_form, "C<>D"),
            (&self.higher_potential, &self.source, &self.be_function, "B<>E"),
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
    fn test_pentad_creation() {
        let pentad = Pentad::new(
            "Test Pentad",
            "Essence", 
            "High Capacity",
            "Low Capacity",
            "Goal",
            "Origin"
        );
        
        assert_eq!(pentad.name, "Test Pentad");
        assert_eq!(pentad.quintessence, "Essence");
        assert_eq!(pentad.higher_potential, "High Capacity");
        assert_eq!(pentad.lower_potential, "Low Capacity");
        assert_eq!(pentad.purpose, "Goal");
        assert_eq!(pentad.source, "Origin");
        
        // Now has default connectives
        assert!(pentad.has_connectives());
        assert_eq!(pentad.bc_range_of_potential.unwrap(), "Range of potential");
    }

    #[test]
    fn test_canonical_terms() {
        let terms = Pentad::get_canonical_terms();
        assert_eq!(terms, vec!["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"]);
    }

    #[test]
    fn test_get_instances() {
        let pentad = Pentad::new(
            "Test",
            "Essence", 
            "High Capacity",
            "Low Capacity",
            "Goal",
            "Origin"
        );
        
        let instances = pentad.get_instances();
        assert_eq!(instances, vec!["Essence", "High Capacity", "Low Capacity", "Goal", "Origin"]);
    }

    #[test]
    fn test_has_connectives_with_some() {
        let mut pentad = Pentad::new("Test", "A", "B", "C", "D", "E");
        pentad.bc_range_of_potential = Some("test connection".to_string());
        
        assert!(pentad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut pentad = Pentad::new("Test", "A", "B", "C", "D", "E");
        // Remove all connectives to test none state
        pentad.bc_range_of_potential = None;
        pentad.de_range_of_significance = None;
        pentad.ab_aspiration = None;
        pentad.ac_operation = None;
        pentad.bd_output = None;
        pentad.ce_input = None;
        pentad.ad_inspiration = None;
        pentad.ae_quantitive_match = None;
        pentad.cd_form = None;
        pentad.be_function = None;
        
        assert!(!pentad.has_connectives());
    }

    #[test]
    fn test_bennett_authentic_connectives() {
        let pentad = Pentad::new("Test", "A", "B", "C", "D", "E");
        
        // Should start with Bennett's authentic connectives
        assert!(pentad.has_connectives());
        assert_eq!(pentad.bc_range_of_potential.unwrap(), "Range of potential");
        assert_eq!(pentad.de_range_of_significance.unwrap(), "Range of significance");
        assert_eq!(pentad.ab_aspiration.unwrap(), "Aspiration");
        assert_eq!(pentad.ac_operation.unwrap(), "Operation");
        assert_eq!(pentad.bd_output.unwrap(), "Output");
        assert_eq!(pentad.ce_input.unwrap(), "Input");
        assert_eq!(pentad.ad_inspiration.unwrap(), "Inspiration");
        assert_eq!(pentad.ae_quantitive_match.unwrap(), "Quantitive match");
        assert_eq!(pentad.cd_form.unwrap(), "Form");
        assert_eq!(pentad.be_function.unwrap(), "Function");
    }

    #[test]
    fn test_custom_connectives() {
        let mut pentad = Pentad::new("Test", "A", "B", "C", "D", "E");
        
        // Modify one connective while others keep defaults
        pentad.bc_range_of_potential = Some("custom connection".to_string());
        
        assert!(pentad.has_connectives());
        assert_eq!(pentad.bc_range_of_potential.unwrap(), "custom connection");
        
        // Other connectives should still have Bennett's defaults
        assert_eq!(pentad.ab_aspiration.unwrap(), "Aspiration");
        assert_eq!(pentad.cd_form.unwrap(), "Form");
    }

    #[test]
    fn test_all_bennett_connectives_count() {
        let pentad = Pentad::new("Test", "A", "B", "C", "D", "E");
        
        // Should have exactly 10 Bennett connectives (5 choose 2 = 10)
        let connectives_count = [
            pentad.bc_range_of_potential.is_some(),
            pentad.de_range_of_significance.is_some(),
            pentad.ab_aspiration.is_some(),
            pentad.ac_operation.is_some(),
            pentad.bd_output.is_some(),
            pentad.ce_input.is_some(),
            pentad.ad_inspiration.is_some(),
            pentad.ae_quantitive_match.is_some(),
            pentad.cd_form.is_some(),
            pentad.be_function.is_some(),
        ].iter().filter(|&&x| x).count();
        
        assert_eq!(connectives_count, 10);
    }
} 