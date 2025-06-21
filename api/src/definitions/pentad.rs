use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, PentadicSystem};

/// Pentad: Five-term systematic structure representing pentadic relationships
/// 
/// Based on Bennett's systematic structures, the pentad represents five-fold
/// relationships with canonical terms: Quintessence, Higher Potential, Lower Potential, Purpose, Source
#[derive(Debug, Clone)]
pub struct Pentad {
    // Core identity
    id: String,
    name: String,
    
    // User's terms for each index position (pentad has 5 term indices)
    instances: [String; 5],
    
    // Connective relationships
    connectives: HashMap<(usize, usize), String>,
    
    // System definition
    system: PentadicSystem,
}

impl Pentad {
    /// Create a new pentad with default empty terms
    pub fn new(name: String, terms: [String; 5]) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            instances: terms,
            connectives: HashMap::new(),
            system: PentadicSystem,
        }
    }
    
    /// Get user instance at position 0 (maps to term character "Quintessence")
    pub fn first_user_instance(&self) -> &str {
        &self.instances[0]
    }
    
    /// Get user instance at position 1 (maps to term character "Higher Potential")
    pub fn second_user_instance(&self) -> &str {
        &self.instances[1]
    }
    
    /// Get user instance at position 2 (maps to term character "Lower Potential")
    pub fn third_user_instance(&self) -> &str {
        &self.instances[2]
    }
    
    /// Get user instance at position 3 (maps to term character "Purpose")
    pub fn fourth_user_instance(&self) -> &str {
        &self.instances[3]
    }
    
    /// Get user instance at position 4 (maps to term character "Source")
    pub fn fifth_user_instance(&self) -> &str {
        &self.instances[4]
    }
    
    /// Get all terms as a tuple
    pub fn terms_tuple(&self) -> (&str, &str, &str, &str, &str) {
        (&self.instances[0], &self.instances[1], &self.instances[2], &self.instances[3], &self.instances[4])
    }
    
    /// Get connective relationship between two terms
    pub fn get_connective(&self, from_index: usize, to_index: usize) -> Option<&String> {
        self.connectives.get(&(from_index, to_index))
    }
    
    /// Set connective relationship between two terms
    pub fn set_connective(&mut self, from_index: usize, to_index: usize, relationship: String) {
        if from_index < 5 && to_index < 5 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        5
    }
    
        /// Map a term character to its positional coordinate
    /// Returns the 0-based index for the given term character
    pub fn term_character_to_position(&self, term_character: &str) -> Option<usize> {
        let term_characters = self.system.term_characters();
        term_characters.iter().position(|&term| term == term_character)
    }

    /// Map a positional coordinate to its term character
    /// Returns the term character for the given 0-based position index
    pub fn term_character_from_position(&self, position: usize) -> Option<&str> {
        let term_characters = self.system.term_characters();
        term_characters.get(position).copied()
    }
    
    /// Map a user instance to its positional coordinate
    /// Returns the 0-based index for the given user instance
    pub fn instance_to_position(&self, instance: &str) -> Option<usize> {
        self.instances.iter().position(|inst| inst == instance)
    }
    
    /// Map a positional coordinate to its user instance
    /// Returns the user instance for the given 0-based position index
    pub fn instance_from_position(&self, position: usize) -> Option<&str> {
        self.instances.get(position).map(|s| s.as_str())
    }
    
        /// Map a position to its term character (alias for term_character_from_position)
    /// Returns the term character for the given 0-based position index
    pub fn position_to_term_character(&self, position: usize) -> Option<&str> {
        self.term_character_from_position(position)
    }

    /// Map a term character to its position (alias for term_character_to_position)
    /// Returns the 0-based index for the given term character
    pub fn position_from_term_character(&self, term_character: &str) -> Option<usize> {
        self.term_character_to_position(term_character)
    }
    
    /// Map a position to its user term (alias for instance_from_position)
    /// Returns the user instance for the given 0-based position index
    pub fn position_to_user_term(&self, position: usize) -> Option<&str> {
        self.instance_from_position(position)
    }
    
    /// Map a user term to its position (alias for instance_to_position)
    /// Returns the 0-based index for the given user instance
    pub fn position_from_user_term(&self, user_term: &str) -> Option<usize> {
        self.instance_to_position(user_term)
    }
}

impl SystematicStructure for Pentad {
    const TERM_COUNT: usize = 5;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn structure_type(&self) -> &str {
        "pentad"
    }
    
    fn coherence_attribute(&self) -> &str {
        self.system.coherence_attribute()
    }
    
    fn term_designation(&self) -> &str {
        self.system.term_designation()
    }
    
    fn source(&self) -> &str {
        self.system.source()
    }
    
    fn term_characters(&self) -> Vec<String> {
        self.system.term_characters().iter().map(|s| s.to_string()).collect()
    }
    
    fn user_instance_index(&self) -> &[String] {
        &self.instances
    }
    
    fn first_order_connectives_type(&self) -> &str {
        self.system.first_order_connectives_type()
    }
    
    fn connectives_traits(&self) -> &std::collections::HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    fn system(&self) -> &dyn systematics_library::System {
        &self.system
    }
    
    fn validate(&self) -> Result<()> {
        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Pentad name cannot be empty".to_string(),
            });
        }
        
        // Validate all five terms are not empty
        let term_names = ["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"];
        for (i, term) in self.instances.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("{} term ({}) cannot be empty", 
                        match i {
                            0 => "First",
                            1 => "Second", 
                            2 => "Third",
                            3 => "Fourth",
                            4 => "Fifth",
                            _ => "Unknown"
                        }, 
                        term_names[i]
                    ),
                });
            }
        }
        
        // Validate term lengths
        for (i, term) in self.instances.iter().enumerate() {
            if term.len() > 100 {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} is too long (max 100 characters)", i + 1),
                });
            }
        }
        
        // Validate terms contain only allowed characters
        for (i, term) in self.instances.iter().enumerate() {
            if !term.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} contains invalid characters", i + 1),
                });
            }
        }
        
        // Validate terms are all different (pentad should represent distinct aspects)
        for i in 0..5 {
            for j in (i + 1)..5 {
                if self.instances[i].trim().to_lowercase() == self.instances[j].trim().to_lowercase() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Terms {} and {} should be different to represent distinct aspects", i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        let header = "=== Pentadic Structure ===";
        println!("\n{}", header);
        println!("Name: {}", self.name());
        println!("{}:", self.term_designation());
        println!("  - {}", self.instances[0]);
        println!("  - {}", self.instances[1]);
        println!("  - {}", self.instances[2]);
        println!("  - {}", self.instances[3]);
        println!("  - {}", self.instances[4]);
        
        // Show connectives if they exist
        if !self.connectives.is_empty() {
            println!("{}:", self.first_order_connectives_type());
            let mut shown_pairs = std::collections::HashSet::new();
            let mut display_items = Vec::new();
            
            // Collect all unique pairs first
            for ((from, to), relationship) in &self.connectives {
                let pair = if from < to { (*from, *to) } else { (*to, *from) };
                if !shown_pairs.contains(&pair) {
                    shown_pairs.insert(pair);
                    let left_term = &self.instances[pair.0];
                    let right_term = &self.instances[pair.1];
                    display_items.push((left_term, relationship, right_term));
                }
            }
            
            // Calculate column widths
            let max_left_len = display_items.iter().map(|(left, _, _)| left.len()).max().unwrap_or(0);
            let max_rel_len = display_items.iter().map(|(_, rel, _)| rel.len()).max().unwrap_or(0);
            let max_right_len = display_items.iter().map(|(_, _, right)| right.len()).max().unwrap_or(0);
            
            // Display with proper column alignment
            for (left_term, relationship, right_term) in display_items {
                println!("  {:^left_width$} <---[{:^rel_width$}]---> {:^right_width$}", 
                    left_term,
                    relationship,
                    right_term,
                    left_width = max_left_len,
                    rel_width = max_rel_len,
                    right_width = max_right_len);
            }
        }
        
        println!();
        println!("Metadata");
        println!("ID: {}", &self.id[..8]); // Short ID for readability
        println!("{}", "=".repeat(header.len()));
    }
    

}

/// Builder for creating Pentad instances
pub struct PentadBuilder {
    name: Option<String>,
    terms: Option<[String; 5]>,
}

impl PentadBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            terms: None,
        }
    }
    
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn terms<S1: Into<String>, S2: Into<String>, S3: Into<String>, S4: Into<String>, S5: Into<String>>(
        mut self, 
        first: S1, 
        second: S2, 
        third: S3, 
        fourth: S4, 
        fifth: S5
    ) -> Self {
        self.terms = Some([
            first.into(),
            second.into(),
            third.into(),
            fourth.into(),
            fifth.into(),
        ]);
        self
    }
    
    pub fn build(self) -> Result<Pentad> {
        let name = self.name.unwrap_or_else(|| "Unnamed Pentad".to_string());
        let terms = self.terms.ok_or_else(|| SystematicsError::Builder {
            reason: "Pentad requires 5 terms".to_string(),
        })?;
        
        let pentad = Pentad::new(name, terms);
        
        pentad.validate()?;
        Ok(pentad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pentad_creation() {
        let pentad = PentadBuilder::new()
            .name("Test Pentad")
            .terms(
                "Quintessence",
                "Higher Potential",
                "Lower Potential",
                "Purpose",
                "Source"
            )
            .build()
            .unwrap();
        
        assert_eq!(pentad.name(), "Test Pentad");
        assert_eq!(pentad.first_user_instance(), "Quintessence");
        assert_eq!(pentad.second_user_instance(), "Higher Potential");
        assert_eq!(pentad.third_user_instance(), "Lower Potential");
        assert_eq!(pentad.fourth_user_instance(), "Purpose");
        assert_eq!(pentad.fifth_user_instance(), "Source");
        assert!(pentad.validate().is_ok());
    }

    #[test]
    fn test_term_characters() {
        let pentad = PentadBuilder::new()
            .name("Test")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
        
        let characters = pentad.term_characters();
        assert_eq!(characters.len(), 5);
        assert_eq!(characters[0], "Purpose");
        assert_eq!(characters[1], "Higher Potential");
        assert_eq!(characters[2], "Quintessence");
        assert_eq!(characters[3], "Lower Potential");
        assert_eq!(characters[4], "Source");
    }

    #[test]
    fn test_connective_relationships() {
        let mut pentad = PentadBuilder::new()
            .name("Test")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
        
        pentad.set_connective(0, 1, "test relationship".to_string());
        assert_eq!(pentad.get_connective(0, 1), Some(&"test relationship".to_string()));
        assert_eq!(pentad.connectives_traits().len(), 1);
    }

    #[test]
    fn test_pentad_validation() {
        // Test empty name
        let result = PentadBuilder::new()
            .name("")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build();
        assert!(result.is_err());
        
        // Test empty term
        let result = PentadBuilder::new()
            .name("Test")
            .terms("".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build();
        assert!(result.is_err());
        
        // Test duplicate terms
        let result = PentadBuilder::new()
            .name("Test")
            .terms("A".to_string(), "A".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build();
        assert!(result.is_err());
        
        // Test valid pentad
        let result = PentadBuilder::new()
            .name("Valid")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pentad_terms_method() {
        let pentad = PentadBuilder::new()
            .name("Test")
            .terms("First".to_string(), "Second".to_string(), "Third".to_string(), "Fourth".to_string(), "Fifth".to_string())
            .build()
            .unwrap();
        
        let terms = pentad.user_instance_index();
        assert_eq!(terms.len(), 5);
        assert_eq!(terms[0], "First");
        assert_eq!(terms[1], "Second");
        assert_eq!(terms[2], "Third");
        assert_eq!(terms[3], "Fourth");
        assert_eq!(terms[4], "Fifth");
        
        let tuple = pentad.terms_tuple();
        assert_eq!(tuple, ("First", "Second", "Third", "Fourth", "Fifth"));
    }

    #[test]
    fn test_trait_compliance() {
        let pentad = PentadBuilder::new()
            .name("Test")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
        
        // Test SystematicStructure trait
        assert_eq!(Pentad::TERM_COUNT, 5);
        assert!(!pentad.id().is_empty());
        assert_eq!(pentad.name(), "Test");
        assert_eq!(pentad.user_instance_index().len(), 5);
        assert!(pentad.validate().is_ok());
    }
} 