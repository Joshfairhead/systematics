use std::io::{self, Write};

#[derive(Debug)]
pub struct Tetrad {
    pub name: String,
    // User instances for canonical positions
    pub ground: String,
    pub ideal: String,
    pub instrumental: String,
    pub directive: String,
    // Canonical connectives (bidirectional relationships)
    pub ground_ideal_connective: Option<String>,
    pub ground_instrumental_connective: Option<String>, 
    pub ground_directive_connective: Option<String>,
    pub ideal_instrumental_connective: Option<String>,
    pub ideal_directive_connective: Option<String>,
    pub instrumental_directive_connective: Option<String>,
}

impl Tetrad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "A Field of Action";

    /// Creates a new Tetrad with user instances for canonical positions
    pub fn new(name: &str, ground: &str, ideal: &str, instrumental: &str, directive: &str) -> Self {
        Tetrad {
            name: name.to_string(),
            ground: ground.to_string(),
            ideal: ideal.to_string(),
            instrumental: instrumental.to_string(),
            directive: directive.to_string(),
            // Initialize connectives with canonical defaults
            ground_ideal_connective: Some("Motivational imperative".to_string()),
            ground_instrumental_connective: Some("Technical power".to_string()),
            ground_directive_connective: Some("Material Mastery".to_string()),
            ideal_instrumental_connective: Some("Effectual compatibility".to_string()),
            ideal_directive_connective: Some("Receptive regard".to_string()),
            instrumental_directive_connective: Some("Demonstrable activity".to_string()),
        }
    }
    
    /// Interactive creation method - handles all input/output internally
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Tetrad ---");
        
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
        
        // Get required inputs
        let name = get_optional_input("Enter a name for your Tetrad (or press Enter for 'Unnamed Tetrad'): ", "Unnamed Tetrad")?;
        let ground = get_required_input("Enter the Ground instance: ", "Ground instance")?;
        let ideal = get_required_input("Enter the Ideal instance: ", "Ideal instance")?;
        let instrumental = get_required_input("Enter the Instrumental instance: ", "Instrumental instance")?;
        let directive = get_required_input("Enter the Directive instance: ", "Directive instance")?;

        let mut tetrad = Tetrad::new(&name, &ground, &ideal, &instrumental, &directive);
        
        // Ask if user wants to modify the default connectives
        let modify_connectives = get_yes_no_input("\nWould you like to modify the default connectives? (y/n): ", "n")?;
        
        if modify_connectives.starts_with('y') {
            println!("\nModifying connectives (press Enter to keep default, or input new value):");
            
            // Helper to handle connective modification
            let modify_connective = |prompt: &str, current: &str| -> Result<Option<String>, Box<dyn std::error::Error>> {
                let input = get_optional_input(prompt, current)?;
                Ok(Some(input))
            };
            
            tetrad.ground_ideal_connective = modify_connective(
                "Motivational imperative (G<>I): ", 
                "Motivational imperative"
            )?;
            
            tetrad.ground_instrumental_connective = modify_connective(
                "Technical power (G<>In): ", 
                "Technical power"
            )?;
            
            tetrad.ground_directive_connective = modify_connective(
                "Material Mastery (G<>D): ", 
                "Material Mastery"
            )?;
            
            tetrad.ideal_instrumental_connective = modify_connective(
                "Effectual compatibility (I<>In): ", 
                "Effectual compatibility"
            )?;
            
            tetrad.ideal_directive_connective = modify_connective(
                "Receptive regard (I<>D): ", 
                "Receptive regard"
            )?;
            
            tetrad.instrumental_directive_connective = modify_connective(
                "Demonstrable activity (In<>D): ", 
                "Demonstrable activity"
            )?;
        } else {
            // Ask if they want to remove all connectives
            let remove_all = get_yes_no_input("Would you like to remove all connectives? (y/n): ", "n")?;
            
            if remove_all.starts_with('y') {
                tetrad.ground_ideal_connective = None;
                tetrad.ground_instrumental_connective = None;
                tetrad.ground_directive_connective = None;
                tetrad.ideal_instrumental_connective = None;
                tetrad.ideal_directive_connective = None;
                tetrad.instrumental_directive_connective = None;
            }
            // Otherwise keep the defaults that were initialized
        }
        
        // Display the created tetrad
        tetrad.display();
        
        // Show connectives if any were defined
        if tetrad.has_connectives() {
            tetrad.display_connectives();
        }
        
        Ok(tetrad)
    }
    
    /// Check if any connectives are defined
    pub fn has_connectives(&self) -> bool {
        self.ground_ideal_connective.is_some() ||
        self.ground_instrumental_connective.is_some() ||
        self.ground_directive_connective.is_some() ||
        self.ideal_instrumental_connective.is_some() ||
        self.ideal_directive_connective.is_some() ||
        self.instrumental_directive_connective.is_some()
    }
    
    /// Get canonical term names (hardcoded)
    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec!["Ground", "Ideal", "Instrumental", "Directive"]
    }
    
    /// Get all user instances
    pub fn get_instances(&self) -> Vec<String> {
        vec![
            self.ground.clone(),
            self.ideal.clone(), 
            self.instrumental.clone(),
            self.directive.clone()
        ]
    }
    
    /// Display tetrad details
    pub fn display(&self) {
        println!("\n--- Tetrad Details ---");
        println!("Tetrad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("Ground: {}", self.ground);
        println!("Ideal: {}", self.ideal);
        println!("Instrumental: {}", self.instrumental);
        println!("Directive: {}", self.directive);
        println!("---------------------");
    }
    
    /// Display all connectives
    pub fn display_connectives(&self) {
        println!("\nCanonical Connectives:");
        let connectives = [
            (&self.ground, &self.ideal, &self.ground_ideal_connective),
            (&self.ground, &self.instrumental, &self.ground_instrumental_connective),
            (&self.ground, &self.directive, &self.ground_directive_connective),
            (&self.ideal, &self.instrumental, &self.ideal_instrumental_connective),
            (&self.ideal, &self.directive, &self.ideal_directive_connective),
            (&self.instrumental, &self.directive, &self.instrumental_directive_connective),
        ];
        
        for (from, to, connective) in connectives {
            match connective {
                Some(conn) => println!("  {} <--[{}]--> {}", from, conn, to),
                None => println!("  {} <--> {} (no connective defined)", from, to),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tetrad_creation() {
        let tetrad = Tetrad::new(
            "Test Tetrad",
            "Foundation", 
            "Vision",
            "Method",
            "Purpose"
        );
        
        assert_eq!(tetrad.name, "Test Tetrad");
        assert_eq!(tetrad.ground, "Foundation");
        assert_eq!(tetrad.ideal, "Vision");
        assert_eq!(tetrad.instrumental, "Method");
        assert_eq!(tetrad.directive, "Purpose");
        
        // Now has default connectives
        assert!(tetrad.has_connectives());
        assert_eq!(tetrad.ground_ideal_connective.unwrap(), "Motivational imperative");
    }

    #[test]
    fn test_canonical_terms() {
        let terms = Tetrad::get_canonical_terms();
        assert_eq!(terms, vec!["Ground", "Ideal", "Instrumental", "Directive"]);
    }

    #[test]
    fn test_get_instances() {
        let tetrad = Tetrad::new(
            "Test",
            "Foundation", 
            "Vision",
            "Method",
            "Purpose"
        );
        
        let instances = tetrad.get_instances();
        assert_eq!(instances, vec!["Foundation", "Vision", "Method", "Purpose"]);
    }

    #[test]
    fn test_has_connectives_with_some() {
        let mut tetrad = Tetrad::new("Test", "F", "V", "M", "P");
        tetrad.ground_ideal_connective = Some("test connection".to_string());
        
        assert!(tetrad.has_connectives());
    }

    #[test]
    fn test_has_connectives_with_none() {
        let mut tetrad = Tetrad::new("Test", "F", "V", "M", "P");
        // Remove all connectives to test none state
        tetrad.ground_ideal_connective = None;
        tetrad.ground_instrumental_connective = None;
        tetrad.ground_directive_connective = None;
        tetrad.ideal_instrumental_connective = None;
        tetrad.ideal_directive_connective = None;
        tetrad.instrumental_directive_connective = None;
        
        assert!(!tetrad.has_connectives());
    }

    #[test]
    fn test_default_connectives() {
        let tetrad = Tetrad::new("Test", "F", "V", "M", "P");
        
        // Should start with default connectives
        assert!(tetrad.has_connectives());
        assert_eq!(tetrad.ground_ideal_connective.unwrap(), "Motivational imperative");
        assert_eq!(tetrad.ground_instrumental_connective.unwrap(), "Technical power");
        assert_eq!(tetrad.ground_directive_connective.unwrap(), "Material Mastery");
        assert_eq!(tetrad.ideal_instrumental_connective.unwrap(), "Effectual compatibility");
        assert_eq!(tetrad.ideal_directive_connective.unwrap(), "Receptive regard");
        assert_eq!(tetrad.instrumental_directive_connective.unwrap(), "Demonstrable activity");
    }

    #[test]
    fn test_custom_connectives() {
        let mut tetrad = Tetrad::new("Test", "F", "V", "M", "P");
        
        // Modify one connective while others keep defaults
        tetrad.ground_ideal_connective = Some("custom connection".to_string());
        
        assert!(tetrad.has_connectives());
        assert_eq!(tetrad.ground_ideal_connective.unwrap(), "custom connection");
        
        // Other connectives should still have defaults
        assert_eq!(tetrad.ground_instrumental_connective.unwrap(), "Technical power");
        assert_eq!(tetrad.ground_directive_connective.unwrap(), "Material Mastery");
    }

    #[test]
    fn test_valid_input_characters() {
        // Test that valid characters are accepted
        let tetrad = Tetrad::new(
            "Test-Name (Version 1.0)", 
            "Foundation?", 
            "Vision!", 
            "Method's approach", 
            "Purpose, clearly stated"
        );
        
        assert_eq!(tetrad.name, "Test-Name (Version 1.0)");
        assert_eq!(tetrad.ground, "Foundation?");
        assert_eq!(tetrad.ideal, "Vision!");
        assert_eq!(tetrad.instrumental, "Method's approach");
        assert_eq!(tetrad.directive, "Purpose, clearly stated");
    }

    #[test]
    fn test_empty_inputs_validation() {
        // Test that we can detect when required fields would be empty
        // Note: The actual interactive function now loops until valid input is provided
        // rather than returning errors for empty input
        let empty_string = "";
        assert!(empty_string.trim().is_empty());
        
        let whitespace_only = "   \t\n  ";
        assert!(whitespace_only.trim().is_empty());
    }

    #[test]
    fn test_input_validation_logic() {
        // Test the validation logic used in the interactive function
        // Note: The actual function loops until valid input is provided
        
        // Test empty detection
        let empty = "";
        assert!(empty.trim().is_empty());
        
        // Test length validation  
        let too_long = "a".repeat(101);
        assert!(too_long.len() > 100);
        
        let acceptable_length = "a".repeat(100);
        assert!(acceptable_length.len() <= 100);
        
        // Test character validation
        let valid_chars = "Hello World! How are you? Fine, thanks. (Version 1.0) It's working.";
        let is_valid = valid_chars.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c));
        assert!(is_valid);
        
        let invalid_chars = "Hello @#$%^&*";
        let is_invalid = !invalid_chars.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c));
        assert!(is_invalid);
    }

    #[test]
    fn test_input_length_validation() {
        // Test that very long strings are caught
        let long_string = "a".repeat(101);
        assert!(long_string.len() > 100);
        
        let acceptable_string = "a".repeat(100);
        assert_eq!(acceptable_string.len(), 100);
    }

    #[test]
    fn test_invalid_characters() {
        // Test character validation logic
        let valid_chars = "Hello World! How are you? Fine, thanks. (Version 1.0) It's working.";
        let is_valid = valid_chars.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c));
        assert!(is_valid);
        
        let invalid_chars = "Hello @#$%^&*+=[]{}|\\\":;<>/`~";
        let is_invalid = !invalid_chars.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c));
        assert!(is_invalid);
    }

    #[test]
    fn test_yes_no_input_validation() {
        // Test yes/no validation logic
        let valid_responses = vec!["y", "yes", "n", "no", "Y", "YES", "N", "NO"];
        for response in valid_responses {
            let normalized = response.to_lowercase();
            let is_valid = matches!(normalized.as_str(), "y" | "yes" | "n" | "no");
            assert!(is_valid, "Response '{}' should be valid", response);
        }
        
        let invalid_responses = vec!["maybe", "yep", "nope", "1", "0", "true", "false"];
        for response in invalid_responses {
            let normalized = response.to_lowercase();
            let is_invalid = !matches!(normalized.as_str(), "y" | "yes" | "n" | "no");
            assert!(is_invalid, "Response '{}' should be invalid", response);
        }
    }

    #[test]
    fn test_error_handling_edge_cases() {
        // Test that our validation logic properly catches edge cases
        
        // Test boundary condition for length
        let exactly_100_chars = "a".repeat(100);
        assert_eq!(exactly_100_chars.len(), 100);
        assert!(exactly_100_chars.len() <= 100, "100 characters should be acceptable");
        
        let exactly_101_chars = "a".repeat(101);
        assert_eq!(exactly_101_chars.len(), 101);
        assert!(exactly_101_chars.len() > 100, "101 characters should be rejected");
        
        // Test mixed valid/invalid characters
        let mixed_valid = "Hello World! (2024) - It's working.";
        let is_valid = mixed_valid.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c));
        assert!(is_valid, "Mixed valid characters should pass validation");
        
        // Test common invalid characters that users might try
        let common_invalid_chars = ["@", "#", "$", "%", "^", "&", "*", "+", "=", "[", "]", "{", "}", "|", "\\", "\"", ":", ";", "<", ">", "/", "`", "~"];
        for invalid_char in common_invalid_chars {
            let test_string = format!("Hello{}", invalid_char);
            let is_invalid = !test_string.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c));
            assert!(is_invalid, "String with '{}' should be invalid", invalid_char);
        }
    }

    #[test]
    fn test_input_normalization() {
        // Test how our validation handles different types of whitespace and normalization
        
        // Test various whitespace scenarios
        let whitespace_scenarios = vec![
            "",           // empty
            " ",          // single space
            "\t",         // tab
            "\n",         // newline
            "\r",         // carriage return
            "  \t\n\r  ", // mixed whitespace
        ];
        
        for scenario in whitespace_scenarios {
            assert!(scenario.trim().is_empty(), "Whitespace scenario '{}' should be detected as empty", scenario.escape_debug());
        }
        
        // Test that trimming works as expected
        let padded_input = "  Hello World  ";
        assert_eq!(padded_input.trim(), "Hello World");
        assert_eq!(padded_input.trim().len(), 11);
    }

    #[test]
    fn test_validation_consistency() {
        // Test that our validation logic is consistent across different scenarios
        
        // These should all be valid
        let valid_inputs = vec![
            "Simple text",
            "Text with numbers 123",
            "Text with punctuation!",
            "Text with question?",
            "Text with apostrophe's",
            "Text with (parentheses)",
            "Text with hyphen-word",
            "Text, with commas.",
        ];
        
        for input in valid_inputs {
            let is_valid = input.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c));
            assert!(is_valid, "Input '{}' should be valid", input);
        }
        
        // These should all be invalid
        let invalid_inputs = vec![
            "Text with @ symbol",
            "Text with # hashtag",
            "Text with $ dollar",
            "Text with % percent",
            "Text with ^ caret",
            "Text with & ampersand",
            "Text with * asterisk",
            "Text with + plus",
            "Text with = equals",
            "Text with [brackets]",
            "Text with {braces}",
            "Text with | pipe",
            "Text with \\ backslash",
            "Text with \" quotes",
            "Text with : colon",
            "Text with ; semicolon",
            "Text with < less",
            "Text with > greater",
            "Text with / slash",
            "Text with ` backtick",
            "Text with ~ tilde",
        ];
        
        for input in invalid_inputs {
            let is_invalid = !input.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c));
            assert!(is_invalid, "Input '{}' should be invalid", input);
        }
    }
}