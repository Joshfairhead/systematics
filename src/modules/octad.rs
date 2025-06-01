use std::io::{self, Write};

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
    // Positional-semantic connectives (bidirectional relationships)
    pub AB_smallest_critical: Option<String>,           // A<>B
    pub AC_smallest_supportive: Option<String>,         // A<>C
    pub AD_smallest_necessary: Option<String>,          // A<>D
    pub AE_smallest_integrative: Option<String>,        // A<>E
    pub AF_smallest_inherent: Option<String>,           // A<>F
    pub AG_smallest_intrinsic: Option<String>,          // A<>G
    pub AH_smallest_organisational: Option<String>,     // A<>H
    pub BC_critical_supportive: Option<String>,         // B<>C
    pub BD_critical_necessary: Option<String>,          // B<>D
    pub BE_critical_integrative: Option<String>,        // B<>E
    pub BF_critical_inherent: Option<String>,           // B<>F
    pub BG_critical_intrinsic: Option<String>,          // B<>G
    pub BH_critical_organisational: Option<String>,     // B<>H
    pub CD_supportive_necessary: Option<String>,        // C<>D
    pub CE_supportive_integrative: Option<String>,      // C<>E
    pub CF_supportive_inherent: Option<String>,         // C<>F
    pub CG_supportive_intrinsic: Option<String>,        // C<>G
    pub CH_supportive_organisational: Option<String>,   // C<>H
    pub DE_necessary_integrative: Option<String>,       // D<>E
    pub DF_necessary_inherent: Option<String>,          // D<>F
    pub DG_necessary_intrinsic: Option<String>,         // D<>G
    pub DH_necessary_organisational: Option<String>,    // D<>H
    pub EF_integrative_inherent: Option<String>,        // E<>F
    pub EG_integrative_intrinsic: Option<String>,       // E<>G
    pub EH_integrative_organisational: Option<String>,  // E<>H
    pub FG_inherent_intrinsic: Option<String>,          // F<>G
    pub FH_inherent_organisational: Option<String>,     // F<>H
    pub GH_intrinsic_organisational: Option<String>,    // G<>H
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
        Octad {
            name: name.to_string(),
            smallest_significant_holon: smallest_significant_holon.to_string(),
            critical_functions: critical_functions.to_string(),
            supportive_platform: supportive_platform.to_string(),
            necessary_resourcing: necessary_resourcing.to_string(),
            integrative_totality: integrative_totality.to_string(),
            inherent_values: inherent_values.to_string(),
            intrinsic_nature: intrinsic_nature.to_string(),
            organisational_modes: organisational_modes.to_string(),
            // Initialize connectives with positional-semantic defaults
            AB_smallest_critical: Some("AB_smallest_critical".to_string()),
            AC_smallest_supportive: Some("AC_smallest_supportive".to_string()),
            AD_smallest_necessary: Some("AD_smallest_necessary".to_string()),
            AE_smallest_integrative: Some("AE_smallest_integrative".to_string()),
            AF_smallest_inherent: Some("AF_smallest_inherent".to_string()),
            AG_smallest_intrinsic: Some("AG_smallest_intrinsic".to_string()),
            AH_smallest_organisational: Some("AH_smallest_organisational".to_string()),
            BC_critical_supportive: Some("BC_critical_supportive".to_string()),
            BD_critical_necessary: Some("BD_critical_necessary".to_string()),
            BE_critical_integrative: Some("BE_critical_integrative".to_string()),
            BF_critical_inherent: Some("BF_critical_inherent".to_string()),
            BG_critical_intrinsic: Some("BG_critical_intrinsic".to_string()),
            BH_critical_organisational: Some("BH_critical_organisational".to_string()),
            CD_supportive_necessary: Some("CD_supportive_necessary".to_string()),
            CE_supportive_integrative: Some("CE_supportive_integrative".to_string()),
            CF_supportive_inherent: Some("CF_supportive_inherent".to_string()),
            CG_supportive_intrinsic: Some("CG_supportive_intrinsic".to_string()),
            CH_supportive_organisational: Some("CH_supportive_organisational".to_string()),
            DE_necessary_integrative: Some("DE_necessary_integrative".to_string()),
            DF_necessary_inherent: Some("DF_necessary_inherent".to_string()),
            DG_necessary_intrinsic: Some("DG_necessary_intrinsic".to_string()),
            DH_necessary_organisational: Some("DH_necessary_organisational".to_string()),
            EF_integrative_inherent: Some("EF_integrative_inherent".to_string()),
            EG_integrative_intrinsic: Some("EG_integrative_intrinsic".to_string()),
            EH_integrative_organisational: Some("EH_integrative_organisational".to_string()),
            FG_inherent_intrinsic: Some("FG_inherent_intrinsic".to_string()),
            FH_inherent_organisational: Some("FH_inherent_organisational".to_string()),
            GH_intrinsic_organisational: Some("GH_intrinsic_organisational".to_string()),
        }
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
            
            // Helper to handle connective modification
            let modify_connective = |prompt: &str, current: &str| -> Result<Option<String>, Box<dyn std::error::Error>> {
                let input = get_optional_input(prompt, current)?;
                Ok(Some(input))
            };
            
            octad.AB_smallest_critical = modify_connective("AB_smallest_critical: ", "AB_smallest_critical")?;
            octad.AC_smallest_supportive = modify_connective("AC_smallest_supportive: ", "AC_smallest_supportive")?;
            octad.AD_smallest_necessary = modify_connective("AD_smallest_necessary: ", "AD_smallest_necessary")?;
            octad.AE_smallest_integrative = modify_connective("AE_smallest_integrative: ", "AE_smallest_integrative")?;
            octad.AF_smallest_inherent = modify_connective("AF_smallest_inherent: ", "AF_smallest_inherent")?;
            octad.AG_smallest_intrinsic = modify_connective("AG_smallest_intrinsic: ", "AG_smallest_intrinsic")?;
            octad.AH_smallest_organisational = modify_connective("AH_smallest_organisational: ", "AH_smallest_organisational")?;
            octad.BC_critical_supportive = modify_connective("BC_critical_supportive: ", "BC_critical_supportive")?;
            octad.BD_critical_necessary = modify_connective("BD_critical_necessary: ", "BD_critical_necessary")?;
            octad.BE_critical_integrative = modify_connective("BE_critical_integrative: ", "BE_critical_integrative")?;
            octad.BF_critical_inherent = modify_connective("BF_critical_inherent: ", "BF_critical_inherent")?;
            octad.BG_critical_intrinsic = modify_connective("BG_critical_intrinsic: ", "BG_critical_intrinsic")?;
            octad.BH_critical_organisational = modify_connective("BH_critical_organisational: ", "BH_critical_organisational")?;
            octad.CD_supportive_necessary = modify_connective("CD_supportive_necessary: ", "CD_supportive_necessary")?;
            octad.CE_supportive_integrative = modify_connective("CE_supportive_integrative: ", "CE_supportive_integrative")?;
            octad.CF_supportive_inherent = modify_connective("CF_supportive_inherent: ", "CF_supportive_inherent")?;
            octad.CG_supportive_intrinsic = modify_connective("CG_supportive_intrinsic: ", "CG_supportive_intrinsic")?;
            octad.CH_supportive_organisational = modify_connective("CH_supportive_organisational: ", "CH_supportive_organisational")?;
            octad.DE_necessary_integrative = modify_connective("DE_necessary_integrative: ", "DE_necessary_integrative")?;
            octad.DF_necessary_inherent = modify_connective("DF_necessary_inherent: ", "DF_necessary_inherent")?;
            octad.DG_necessary_intrinsic = modify_connective("DG_necessary_intrinsic: ", "DG_necessary_intrinsic")?;
            octad.DH_necessary_organisational = modify_connective("DH_necessary_organisational: ", "DH_necessary_organisational")?;
            octad.EF_integrative_inherent = modify_connective("EF_integrative_inherent: ", "EF_integrative_inherent")?;
            octad.EG_integrative_intrinsic = modify_connective("EG_integrative_intrinsic: ", "EG_integrative_intrinsic")?;
            octad.EH_integrative_organisational = modify_connective("EH_integrative_organisational: ", "EH_integrative_organisational")?;
            octad.FG_inherent_intrinsic = modify_connective("FG_inherent_intrinsic: ", "FG_inherent_intrinsic")?;
            octad.FH_inherent_organisational = modify_connective("FH_inherent_organisational: ", "FH_inherent_organisational")?;
            octad.GH_intrinsic_organisational = modify_connective("GH_intrinsic_organisational: ", "GH_intrinsic_organisational")?;
        } else {
            // Keep the defaults that were initialized (no further questions needed)
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
        self.AB_smallest_critical.is_some() ||
        self.AC_smallest_supportive.is_some() ||
        self.AD_smallest_necessary.is_some() ||
        self.AE_smallest_integrative.is_some() ||
        self.AF_smallest_inherent.is_some() ||
        self.AG_smallest_intrinsic.is_some() ||
        self.AH_smallest_organisational.is_some() ||
        self.BC_critical_supportive.is_some() ||
        self.BD_critical_necessary.is_some() ||
        self.BE_critical_integrative.is_some() ||
        self.BF_critical_inherent.is_some() ||
        self.BG_critical_intrinsic.is_some() ||
        self.BH_critical_organisational.is_some() ||
        self.CD_supportive_necessary.is_some() ||
        self.CE_supportive_integrative.is_some() ||
        self.CF_supportive_inherent.is_some() ||
        self.CG_supportive_intrinsic.is_some() ||
        self.CH_supportive_organisational.is_some() ||
        self.DE_necessary_integrative.is_some() ||
        self.DF_necessary_inherent.is_some() ||
        self.DG_necessary_intrinsic.is_some() ||
        self.DH_necessary_organisational.is_some() ||
        self.EF_integrative_inherent.is_some() ||
        self.EG_integrative_intrinsic.is_some() ||
        self.EH_integrative_organisational.is_some() ||
        self.FG_inherent_intrinsic.is_some() ||
        self.FH_inherent_organisational.is_some() ||
        self.GH_intrinsic_organisational.is_some()
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
        let connectives = [
            (&self.smallest_significant_holon, &self.critical_functions, &self.AB_smallest_critical, "A<>B"),
            (&self.smallest_significant_holon, &self.supportive_platform, &self.AC_smallest_supportive, "A<>C"),
            (&self.smallest_significant_holon, &self.necessary_resourcing, &self.AD_smallest_necessary, "A<>D"),
            (&self.smallest_significant_holon, &self.integrative_totality, &self.AE_smallest_integrative, "A<>E"),
            (&self.smallest_significant_holon, &self.inherent_values, &self.AF_smallest_inherent, "A<>F"),
            (&self.smallest_significant_holon, &self.intrinsic_nature, &self.AG_smallest_intrinsic, "A<>G"),
            (&self.smallest_significant_holon, &self.organisational_modes, &self.AH_smallest_organisational, "A<>H"),
            (&self.critical_functions, &self.supportive_platform, &self.BC_critical_supportive, "B<>C"),
            (&self.critical_functions, &self.necessary_resourcing, &self.BD_critical_necessary, "B<>D"),
            (&self.critical_functions, &self.integrative_totality, &self.BE_critical_integrative, "B<>E"),
            (&self.critical_functions, &self.inherent_values, &self.BF_critical_inherent, "B<>F"),
            (&self.critical_functions, &self.intrinsic_nature, &self.BG_critical_intrinsic, "B<>G"),
            (&self.critical_functions, &self.organisational_modes, &self.BH_critical_organisational, "B<>H"),
            (&self.supportive_platform, &self.necessary_resourcing, &self.CD_supportive_necessary, "C<>D"),
            (&self.supportive_platform, &self.integrative_totality, &self.CE_supportive_integrative, "C<>E"),
            (&self.supportive_platform, &self.inherent_values, &self.CF_supportive_inherent, "C<>F"),
            (&self.supportive_platform, &self.intrinsic_nature, &self.CG_supportive_intrinsic, "C<>G"),
            (&self.supportive_platform, &self.organisational_modes, &self.CH_supportive_organisational, "C<>H"),
            (&self.necessary_resourcing, &self.integrative_totality, &self.DE_necessary_integrative, "D<>E"),
            (&self.necessary_resourcing, &self.inherent_values, &self.DF_necessary_inherent, "D<>F"),
            (&self.necessary_resourcing, &self.intrinsic_nature, &self.DG_necessary_intrinsic, "D<>G"),
            (&self.necessary_resourcing, &self.organisational_modes, &self.DH_necessary_organisational, "D<>H"),
            (&self.integrative_totality, &self.inherent_values, &self.EF_integrative_inherent, "E<>F"),
            (&self.integrative_totality, &self.intrinsic_nature, &self.EG_integrative_intrinsic, "E<>G"),
            (&self.integrative_totality, &self.organisational_modes, &self.EH_integrative_organisational, "E<>H"),
            (&self.inherent_values, &self.intrinsic_nature, &self.FG_inherent_intrinsic, "F<>G"),
            (&self.inherent_values, &self.organisational_modes, &self.FH_inherent_organisational, "F<>H"),
            (&self.intrinsic_nature, &self.organisational_modes, &self.GH_intrinsic_organisational, "G<>H"),
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
        assert_eq!(octad.AB_smallest_critical.unwrap(), "AB_smallest_critical");
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
        let mut octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        octad.AB_smallest_critical = Some("test connection".to_string());
        
        assert!(octad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        // Remove all connectives to test none state
        octad.AB_smallest_critical = None;
        octad.AC_smallest_supportive = None;
        octad.AD_smallest_necessary = None;
        octad.AE_smallest_integrative = None;
        octad.AF_smallest_inherent = None;
        octad.AG_smallest_intrinsic = None;
        octad.AH_smallest_organisational = None;
        octad.BC_critical_supportive = None;
        octad.BD_critical_necessary = None;
        octad.BE_critical_integrative = None;
        octad.BF_critical_inherent = None;
        octad.BG_critical_intrinsic = None;
        octad.BH_critical_organisational = None;
        octad.CD_supportive_necessary = None;
        octad.CE_supportive_integrative = None;
        octad.CF_supportive_inherent = None;
        octad.CG_supportive_intrinsic = None;
        octad.CH_supportive_organisational = None;
        octad.DE_necessary_integrative = None;
        octad.DF_necessary_inherent = None;
        octad.DG_necessary_intrinsic = None;
        octad.DH_necessary_organisational = None;
        octad.EF_integrative_inherent = None;
        octad.EG_integrative_intrinsic = None;
        octad.EH_integrative_organisational = None;
        octad.FG_inherent_intrinsic = None;
        octad.FH_inherent_organisational = None;
        octad.GH_intrinsic_organisational = None;
        
        assert!(!octad.has_connectives());
    }

    #[test]
    fn test_positional_semantic_connectives() {
        let octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        
        // Should start with positional-semantic connectives
        assert!(octad.has_connectives());
        assert_eq!(octad.AB_smallest_critical.unwrap(), "AB_smallest_critical");
        assert_eq!(octad.CD_supportive_necessary.unwrap(), "CD_supportive_necessary");
        assert_eq!(octad.EF_integrative_inherent.unwrap(), "EF_integrative_inherent");
        assert_eq!(octad.GH_intrinsic_organisational.unwrap(), "GH_intrinsic_organisational");
    }

    #[test]
    fn test_custom_connectives() {
        let mut octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        
        // Modify one connective while others keep defaults
        octad.AB_smallest_critical = Some("custom connection".to_string());
        
        assert!(octad.has_connectives());
        assert_eq!(octad.AB_smallest_critical.unwrap(), "custom connection");
        
        // Other connectives should still have defaults
        assert_eq!(octad.CD_supportive_necessary.unwrap(), "CD_supportive_necessary");
        assert_eq!(octad.EF_integrative_inherent.unwrap(), "EF_integrative_inherent");
    }

    #[test]
    fn test_all_connectives_count() {
        let octad = Octad::new("Test", "A", "B", "C", "D", "E", "F", "G", "H");
        
        // Should have exactly 28 connectives (8 choose 2 = 28)
        let connectives_count = [
            octad.AB_smallest_critical.is_some(),
            octad.AC_smallest_supportive.is_some(),
            octad.AD_smallest_necessary.is_some(),
            octad.AE_smallest_integrative.is_some(),
            octad.AF_smallest_inherent.is_some(),
            octad.AG_smallest_intrinsic.is_some(),
            octad.AH_smallest_organisational.is_some(),
            octad.BC_critical_supportive.is_some(),
            octad.BD_critical_necessary.is_some(),
            octad.BE_critical_integrative.is_some(),
            octad.BF_critical_inherent.is_some(),
            octad.BG_critical_intrinsic.is_some(),
            octad.BH_critical_organisational.is_some(),
            octad.CD_supportive_necessary.is_some(),
            octad.CE_supportive_integrative.is_some(),
            octad.CF_supportive_inherent.is_some(),
            octad.CG_supportive_intrinsic.is_some(),
            octad.CH_supportive_organisational.is_some(),
            octad.DE_necessary_integrative.is_some(),
            octad.DF_necessary_inherent.is_some(),
            octad.DG_necessary_intrinsic.is_some(),
            octad.DH_necessary_organisational.is_some(),
            octad.EF_integrative_inherent.is_some(),
            octad.EG_integrative_intrinsic.is_some(),
            octad.EH_integrative_organisational.is_some(),
            octad.FG_inherent_intrinsic.is_some(),
            octad.FH_inherent_organisational.is_some(),
            octad.GH_intrinsic_organisational.is_some(),
        ].iter().filter(|&&x| x).count();
        
        assert_eq!(connectives_count, 28);
    }
} 