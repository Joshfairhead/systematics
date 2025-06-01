use std::io::{self, Write};

#[derive(Debug)]
pub struct Dyad {
    pub name: String,
    pub essence: String,
    pub existence: String,
}

impl Dyad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Complimentarity, polarity or force";

    /// Creates a new Dyad.
    pub fn new(name: &str, essence: &str, existence: &str) -> Self {
        Dyad {
            name: name.to_string(),
            essence: essence.to_string(),
            existence: existence.to_string(),
        }
    }
    
    /// Interactive creation method - handles all input/output internally
    #[allow(dead_code)]
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Dyad ---");
        
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
        let _get_yes_no_input = |prompt: &str, default: &str| -> Result<String, Box<dyn std::error::Error>> {
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
        let name = get_optional_input("Enter a name for your Dyad (or press Enter for 'Unnamed Dyad'): ", "Unnamed Dyad")?;
        let essence = get_required_input("Enter the Essence instance: ", "Essence instance")?;
        let existence = get_required_input("Enter the Existence instance: ", "Existence instance")?;

        let dyad = Dyad::new(&name, &essence, &existence);
        
        // Display the created dyad
        dyad.display();
        
        Ok(dyad)
    }
    
    /// Display dyad details
    #[allow(dead_code)]
    pub fn display(&self) {
        println!("\n--- Dyad Details ---");
        println!("Dyad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("Essence: {}", self.essence);
        println!("Existence: {}", self.existence);
        println!("---------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dyad_creation() {
        let dyad = Dyad::new(
            "Test Dyad",
            "Being", 
            "Becoming"
        );
        
        assert_eq!(dyad.name, "Test Dyad");
        assert_eq!(dyad.essence, "Being");
        assert_eq!(dyad.existence, "Becoming");
    }
}