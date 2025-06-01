use std::io::{self, Write};

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
    // Positional-semantic connectives (bidirectional relationships)
    pub AB_insight_research: Option<String>,       // A<>B
    pub AC_insight_design: Option<String>,         // A<>C
    pub AD_insight_synthesis: Option<String>,      // A<>D
    pub AE_insight_application: Option<String>,    // A<>E
    pub AF_insight_delivery: Option<String>,       // A<>F
    pub AG_insight_value: Option<String>,          // A<>G
    pub BC_research_design: Option<String>,        // B<>C
    pub BD_research_synthesis: Option<String>,     // B<>D
    pub BE_research_application: Option<String>,   // B<>E
    pub BF_research_delivery: Option<String>,      // B<>F
    pub BG_research_value: Option<String>,         // B<>G
    pub CD_design_synthesis: Option<String>,       // C<>D
    pub CE_design_application: Option<String>,     // C<>E
    pub CF_design_delivery: Option<String>,        // C<>F
    pub CG_design_value: Option<String>,           // C<>G
    pub DE_synthesis_application: Option<String>,  // D<>E
    pub DF_synthesis_delivery: Option<String>,     // D<>F
    pub DG_synthesis_value: Option<String>,        // D<>G
    pub EF_application_delivery: Option<String>,   // E<>F
    pub EG_application_value: Option<String>,      // E<>G
    pub FG_delivery_value: Option<String>,         // F<>G
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
        Heptad {
            name: name.to_string(),
            insight: insight.to_string(),
            research: research.to_string(),
            design: design.to_string(),
            synthesis: synthesis.to_string(),
            application: application.to_string(),
            delivery: delivery.to_string(),
            value: value.to_string(),
            // Initialize connectives with positional-semantic defaults
            AB_insight_research: Some("AB_insight_research".to_string()),
            AC_insight_design: Some("AC_insight_design".to_string()),
            AD_insight_synthesis: Some("AD_insight_synthesis".to_string()),
            AE_insight_application: Some("AE_insight_application".to_string()),
            AF_insight_delivery: Some("AF_insight_delivery".to_string()),
            AG_insight_value: Some("AG_insight_value".to_string()),
            BC_research_design: Some("BC_research_design".to_string()),
            BD_research_synthesis: Some("BD_research_synthesis".to_string()),
            BE_research_application: Some("BE_research_application".to_string()),
            BF_research_delivery: Some("BF_research_delivery".to_string()),
            BG_research_value: Some("BG_research_value".to_string()),
            CD_design_synthesis: Some("CD_design_synthesis".to_string()),
            CE_design_application: Some("CE_design_application".to_string()),
            CF_design_delivery: Some("CF_design_delivery".to_string()),
            CG_design_value: Some("CG_design_value".to_string()),
            DE_synthesis_application: Some("DE_synthesis_application".to_string()),
            DF_synthesis_delivery: Some("DF_synthesis_delivery".to_string()),
            DG_synthesis_value: Some("DG_synthesis_value".to_string()),
            EF_application_delivery: Some("EF_application_delivery".to_string()),
            EG_application_value: Some("EG_application_value".to_string()),
            FG_delivery_value: Some("FG_delivery_value".to_string()),
        }
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
            println!("Note: Heptad has 21 connectives - this will take a moment to review.");
            
            // Helper to handle connective modification
            let modify_connective = |prompt: &str, current: &str| -> Result<Option<String>, Box<dyn std::error::Error>> {
                let input = get_optional_input(prompt, current)?;
                Ok(Some(input))
            };
            
            heptad.AB_insight_research = modify_connective("AB_insight_research: ", "AB_insight_research")?;
            heptad.AC_insight_design = modify_connective("AC_insight_design: ", "AC_insight_design")?;
            heptad.AD_insight_synthesis = modify_connective("AD_insight_synthesis: ", "AD_insight_synthesis")?;
            heptad.AE_insight_application = modify_connective("AE_insight_application: ", "AE_insight_application")?;
            heptad.AF_insight_delivery = modify_connective("AF_insight_delivery: ", "AF_insight_delivery")?;
            heptad.AG_insight_value = modify_connective("AG_insight_value: ", "AG_insight_value")?;
            heptad.BC_research_design = modify_connective("BC_research_design: ", "BC_research_design")?;
            heptad.BD_research_synthesis = modify_connective("BD_research_synthesis: ", "BD_research_synthesis")?;
            heptad.BE_research_application = modify_connective("BE_research_application: ", "BE_research_application")?;
            heptad.BF_research_delivery = modify_connective("BF_research_delivery: ", "BF_research_delivery")?;
            heptad.BG_research_value = modify_connective("BG_research_value: ", "BG_research_value")?;
            heptad.CD_design_synthesis = modify_connective("CD_design_synthesis: ", "CD_design_synthesis")?;
            heptad.CE_design_application = modify_connective("CE_design_application: ", "CE_design_application")?;
            heptad.CF_design_delivery = modify_connective("CF_design_delivery: ", "CF_design_delivery")?;
            heptad.CG_design_value = modify_connective("CG_design_value: ", "CG_design_value")?;
            heptad.DE_synthesis_application = modify_connective("DE_synthesis_application: ", "DE_synthesis_application")?;
            heptad.DF_synthesis_delivery = modify_connective("DF_synthesis_delivery: ", "DF_synthesis_delivery")?;
            heptad.DG_synthesis_value = modify_connective("DG_synthesis_value: ", "DG_synthesis_value")?;
            heptad.EF_application_delivery = modify_connective("EF_application_delivery: ", "EF_application_delivery")?;
            heptad.EG_application_value = modify_connective("EG_application_value: ", "EG_application_value")?;
            heptad.FG_delivery_value = modify_connective("FG_delivery_value: ", "FG_delivery_value")?;
        } else {
            // Keep the defaults that were initialized (no further questions needed)
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
        self.AB_insight_research.is_some() ||
        self.AC_insight_design.is_some() ||
        self.AD_insight_synthesis.is_some() ||
        self.AE_insight_application.is_some() ||
        self.AF_insight_delivery.is_some() ||
        self.AG_insight_value.is_some() ||
        self.BC_research_design.is_some() ||
        self.BD_research_synthesis.is_some() ||
        self.BE_research_application.is_some() ||
        self.BF_research_delivery.is_some() ||
        self.BG_research_value.is_some() ||
        self.CD_design_synthesis.is_some() ||
        self.CE_design_application.is_some() ||
        self.CF_design_delivery.is_some() ||
        self.CG_design_value.is_some() ||
        self.DE_synthesis_application.is_some() ||
        self.DF_synthesis_delivery.is_some() ||
        self.DG_synthesis_value.is_some() ||
        self.EF_application_delivery.is_some() ||
        self.EG_application_value.is_some() ||
        self.FG_delivery_value.is_some()
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
        let connectives = [
            (&self.insight, &self.research, &self.AB_insight_research, "A<>B"),
            (&self.insight, &self.design, &self.AC_insight_design, "A<>C"),
            (&self.insight, &self.synthesis, &self.AD_insight_synthesis, "A<>D"),
            (&self.insight, &self.application, &self.AE_insight_application, "A<>E"),
            (&self.insight, &self.delivery, &self.AF_insight_delivery, "A<>F"),
            (&self.insight, &self.value, &self.AG_insight_value, "A<>G"),
            (&self.research, &self.design, &self.BC_research_design, "B<>C"),
            (&self.research, &self.synthesis, &self.BD_research_synthesis, "B<>D"),
            (&self.research, &self.application, &self.BE_research_application, "B<>E"),
            (&self.research, &self.delivery, &self.BF_research_delivery, "B<>F"),
            (&self.research, &self.value, &self.BG_research_value, "B<>G"),
            (&self.design, &self.synthesis, &self.CD_design_synthesis, "C<>D"),
            (&self.design, &self.application, &self.CE_design_application, "C<>E"),
            (&self.design, &self.delivery, &self.CF_design_delivery, "C<>F"),
            (&self.design, &self.value, &self.CG_design_value, "C<>G"),
            (&self.synthesis, &self.application, &self.DE_synthesis_application, "D<>E"),
            (&self.synthesis, &self.delivery, &self.DF_synthesis_delivery, "D<>F"),
            (&self.synthesis, &self.value, &self.DG_synthesis_value, "D<>G"),
            (&self.application, &self.delivery, &self.EF_application_delivery, "E<>F"),
            (&self.application, &self.value, &self.EG_application_value, "E<>G"),
            (&self.delivery, &self.value, &self.FG_delivery_value, "F<>G"),
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
    fn test_heptad_creation() {
        let heptad = Heptad::new(
            "Test Heptad",
            "Innovation", 
            "Analysis",
            "Planning",
            "Integration",
            "Implementation",
            "Deployment",
            "Worth"
        );
        
        assert_eq!(heptad.name, "Test Heptad");
        assert_eq!(heptad.insight, "Innovation");
        assert_eq!(heptad.research, "Analysis");
        assert_eq!(heptad.design, "Planning");
        assert_eq!(heptad.synthesis, "Integration");
        assert_eq!(heptad.application, "Implementation");
        assert_eq!(heptad.delivery, "Deployment");
        assert_eq!(heptad.value, "Worth");
        
        // Should have default connectives
        assert!(heptad.has_connectives());
        assert_eq!(heptad.AB_insight_research.unwrap(), "AB_insight_research");
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
            "Innovation", 
            "Analysis",
            "Planning",
            "Integration",
            "Implementation",
            "Deployment",
            "Worth"
        );
        
        let instances = heptad.get_instances();
        assert_eq!(instances, vec!["Innovation", "Analysis", "Planning", "Integration", "Implementation", "Deployment", "Worth"]);
    }

    #[test]
    fn test_has_connectives_with_some() {
        let mut heptad = Heptad::new("Test", "I", "R", "D", "S", "A", "De", "V");
        heptad.AB_insight_research = Some("test connection".to_string());
        
        assert!(heptad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut heptad = Heptad::new("Test", "I", "R", "D", "S", "A", "De", "V");
        // Remove all connectives to test none state
        heptad.AB_insight_research = None;
        heptad.AC_insight_design = None;
        heptad.AD_insight_synthesis = None;
        heptad.AE_insight_application = None;
        heptad.AF_insight_delivery = None;
        heptad.AG_insight_value = None;
        heptad.BC_research_design = None;
        heptad.BD_research_synthesis = None;
        heptad.BE_research_application = None;
        heptad.BF_research_delivery = None;
        heptad.BG_research_value = None;
        heptad.CD_design_synthesis = None;
        heptad.CE_design_application = None;
        heptad.CF_design_delivery = None;
        heptad.CG_design_value = None;
        heptad.DE_synthesis_application = None;
        heptad.DF_synthesis_delivery = None;
        heptad.DG_synthesis_value = None;
        heptad.EF_application_delivery = None;
        heptad.EG_application_value = None;
        heptad.FG_delivery_value = None;
        
        assert!(!heptad.has_connectives());
    }

    #[test]
    fn test_positional_semantic_connectives() {
        let heptad = Heptad::new("Test", "I", "R", "D", "S", "A", "De", "V");
        
        // Should start with positional-semantic connectives
        assert!(heptad.has_connectives());
        assert_eq!(heptad.AB_insight_research.unwrap(), "AB_insight_research");
        assert_eq!(heptad.CD_design_synthesis.unwrap(), "CD_design_synthesis");
        assert_eq!(heptad.EF_application_delivery.unwrap(), "EF_application_delivery");
        assert_eq!(heptad.FG_delivery_value.unwrap(), "FG_delivery_value");
    }

    #[test]
    fn test_custom_connectives() {
        let mut heptad = Heptad::new("Test", "I", "R", "D", "S", "A", "De", "V");
        
        // Modify one connective while others keep defaults
        heptad.AB_insight_research = Some("custom connection".to_string());
        
        assert!(heptad.has_connectives());
        assert_eq!(heptad.AB_insight_research.unwrap(), "custom connection");
        
        // Other connectives should still have defaults
        assert_eq!(heptad.CD_design_synthesis.unwrap(), "CD_design_synthesis");
        assert_eq!(heptad.EF_application_delivery.unwrap(), "EF_application_delivery");
    }

    #[test]
    fn test_all_connectives_count() {
        let heptad = Heptad::new("Test", "I", "R", "D", "S", "A", "De", "V");
        
        // Should have exactly 21 connectives (7 choose 2 = 21)
        let connectives_count = [
            heptad.AB_insight_research.is_some(),
            heptad.AC_insight_design.is_some(),
            heptad.AD_insight_synthesis.is_some(),
            heptad.AE_insight_application.is_some(),
            heptad.AF_insight_delivery.is_some(),
            heptad.AG_insight_value.is_some(),
            heptad.BC_research_design.is_some(),
            heptad.BD_research_synthesis.is_some(),
            heptad.BE_research_application.is_some(),
            heptad.BF_research_delivery.is_some(),
            heptad.BG_research_value.is_some(),
            heptad.CD_design_synthesis.is_some(),
            heptad.CE_design_application.is_some(),
            heptad.CF_design_delivery.is_some(),
            heptad.CG_design_value.is_some(),
            heptad.DE_synthesis_application.is_some(),
            heptad.DF_synthesis_delivery.is_some(),
            heptad.DG_synthesis_value.is_some(),
            heptad.EF_application_delivery.is_some(),
            heptad.EG_application_value.is_some(),
            heptad.FG_delivery_value.is_some(),
        ].iter().filter(|&&x| x).count();
        
        assert_eq!(connectives_count, 21);
    }
} 