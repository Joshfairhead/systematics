use std::io::{self, Write};

/// Represents a single permutation of three terms in sequence
#[derive(Debug, Clone)]
pub struct Permutation {
    pub name: String,
    pub initiating: String,
    pub colouring: String,
    pub outcome: String,
}

impl Permutation {
    pub fn new(name: &str, initiating: &str, colouring: &str, outcome: &str) -> Self {
        Permutation {
            name: name.to_string(),
            initiating: initiating.to_string(),
            colouring: colouring.to_string(),
            outcome: outcome.to_string(),
        }
    }

    /// Display the permutation as "Name: Initiating → Colouring → Outcome"
    pub fn display(&self) -> String {
        format!("{}: {} → {} → {}", self.name, self.initiating, self.colouring, self.outcome)
    }
}

/// Generate all six permutations of three terms
pub fn generate_six_permutations(term_a: &str, term_b: &str, term_c: &str) -> Vec<Permutation> {
    vec![
        Permutation::new("Expansion", term_a, term_b, term_c),     // term_1 > term_2 > term_3 (123)
        Permutation::new("Interaction", term_a, term_c, term_b),   // term_1 > term_3 > term_2 (132)
        Permutation::new("Concentration", term_b, term_a, term_c), // term_2 > term_1 > term_3 (213)
        Permutation::new("Identity", term_b, term_c, term_a),      // term_2 > term_3 > term_1 (231)
        Permutation::new("Order", term_c, term_a, term_b),         // term_3 > term_1 > term_2 (312)
        Permutation::new("Freedom", term_c, term_b, term_a),       // term_3 > term_2 > term_1 (321)
    ]
}

/// Interactive permutation generator - gets three terms from user and displays all permutations
pub fn create_interactive() -> Result<Vec<Permutation>, Box<dyn std::error::Error>> {
    println!("\n--- Six Permutations Generator ---");
    
    // Helper for input with validation
    let get_term_input = |prompt: &str, field_name: &str| -> Result<String, Box<dyn std::error::Error>> {
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
    
    // Get the three terms
    let term_1 = get_term_input("Enter initiating term: ", "Initiating term")?;
    let term_2 = get_term_input("Enter colouring term: ", "Colouring term")?;
    let term_3 = get_term_input("Enter outcome term: ", "Outcome term")?;
    
    // Generate permutations
    let permutations = generate_six_permutations(&term_1, &term_2, &term_3);
    
    // Display results
    println!("\n--- Six Permutations ---");
    println!("For terms: '{}', '{}', '{}'", term_1, term_2, term_3);
    println!();
    
    for (i, perm) in permutations.iter().enumerate() {
        println!("{}. {}", i + 1, perm.display());
    }
    println!("------------------------");
    
    Ok(permutations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_creation() {
        let perm = Permutation::new("Test", "Active", "Passive", "Reconciling");
        
        assert_eq!(perm.name, "Test");
        assert_eq!(perm.initiating, "Active");
        assert_eq!(perm.colouring, "Passive");
        assert_eq!(perm.outcome, "Reconciling");
    }

    #[test]
    fn test_permutation_display() {
        let perm = Permutation::new("Expansion", "A", "B", "C");
        assert_eq!(perm.display(), "Expansion: A → B → C");
    }

    #[test]
    fn test_generate_six_permutations() {
        let perms = generate_six_permutations("A", "B", "C");
        
        assert_eq!(perms.len(), 6);
        
        // Check all permutations are present with their names
        let displays: Vec<String> = perms.iter().map(|p| p.display()).collect();
        
        assert!(displays.contains(&"Expansion: A → B → C".to_string()));
        assert!(displays.contains(&"Interaction: A → C → B".to_string()));
        assert!(displays.contains(&"Concentration: B → A → C".to_string()));
        assert!(displays.contains(&"Identity: B → C → A".to_string()));
        assert!(displays.contains(&"Order: C → A → B".to_string()));
        assert!(displays.contains(&"Freedom: C → B → A".to_string()));
    }

    #[test]
    fn test_permutation_uniqueness() {
        let perms = generate_six_permutations("X", "Y", "Z");
        let displays: Vec<String> = perms.iter().map(|p| p.display()).collect();
        
        // Check that all permutations are unique
        let mut sorted_displays = displays.clone();
        sorted_displays.sort();
        sorted_displays.dedup();
        
        assert_eq!(displays.len(), sorted_displays.len());
    }

    #[test]
    fn test_permutation_with_identical_terms() {
        let perms = generate_six_permutations("Same", "Same", "Same");
        
        assert_eq!(perms.len(), 6);
        
        // Check that names are different even with identical terms
        let names: Vec<String> = perms.iter().map(|p| p.name.clone()).collect();
        assert!(names.contains(&"Expansion".to_string()));
        assert!(names.contains(&"Interaction".to_string()));
        assert!(names.contains(&"Concentration".to_string()));
        assert!(names.contains(&"Identity".to_string()));
        assert!(names.contains(&"Order".to_string()));
        assert!(names.contains(&"Freedom".to_string()));
    }

    #[test]
    fn test_permutation_with_special_characters() {
        let perms = generate_six_permutations("Term-1", "Term (2)", "Term.3");
        
        assert_eq!(perms.len(), 6);
        assert_eq!(perms[0].display(), "Expansion: Term-1 → Term (2) → Term.3");
    }

    #[test]
    fn test_permutation_clone() {
        let original = Permutation::new("Test", "A", "B", "C");
        let cloned = original.clone();
        
        assert_eq!(original.name, cloned.name);
        assert_eq!(original.initiating, cloned.initiating);
        assert_eq!(original.colouring, cloned.colouring);
        assert_eq!(original.outcome, cloned.outcome);
    }

    #[test]
    fn test_permutation_debug_format() {
        let perm = Permutation::new("Debug", "Debug", "Test", "Format");
        let debug_str = format!("{:?}", perm);
        
        assert!(debug_str.contains("Debug"));
        assert!(debug_str.contains("Test"));
        assert!(debug_str.contains("Format"));
    }
} 