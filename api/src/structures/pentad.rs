use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, schemas::{Schema, PentadSchema}, error::{Result, SystematicsError}};

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
    user_term_index: [String; 5],
    
    // Connective relationships
    connectives: HashMap<(usize, usize), String>,
    
    // Schema definition
    schema: PentadSchema,
}

impl Pentad {
    /// Create a new pentad with default empty terms
    pub fn new(name: String, terms: [String; 5]) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: terms,
            connectives: HashMap::new(),
            schema: PentadSchema,
        }
    }
    
    /// Get the first term (Quintessence)
    pub fn first_term(&self) -> &str {
        &self.user_term_index[0]
    }
    
    /// Get the second term (Higher Potential)
    pub fn second_term(&self) -> &str {
        &self.user_term_index[1]
    }
    
    /// Get the third term (Lower Potential)
    pub fn third_term(&self) -> &str {
        &self.user_term_index[2]
    }
    
    /// Get the fourth term (Purpose)
    pub fn fourth_term(&self) -> &str {
        &self.user_term_index[3]
    }
    
    /// Get the fifth term (Source)
    pub fn fifth_term(&self) -> &str {
        &self.user_term_index[4]
    }
    
    /// Get all terms as a tuple
    pub fn terms_tuple(&self) -> (&str, &str, &str, &str, &str) {
        (&self.user_term_index[0], &self.user_term_index[1], &self.user_term_index[2], &self.user_term_index[3], &self.user_term_index[4])
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
    
    /// Map a canonical term to its positional coordinate
    /// Returns the 0-based index for the given canonical term
    pub fn canonical_term_to_position(&self, canonical_term: &str) -> Option<usize> {
        let canonical_terms = self.schema.canonical_terms();
        canonical_terms.iter().position(|&term| term == canonical_term)
    }
    
    /// Map a positional coordinate to its canonical term
    /// Returns the canonical term for the given 0-based position index
    pub fn canonical_term_from_position(&self, position: usize) -> Option<&str> {
        let canonical_terms = self.schema.canonical_terms();
        canonical_terms.get(position).copied()
    }
    
    /// Map a user instance to its positional coordinate
    /// Returns the 0-based index for the given user instance
    pub fn instance_to_position(&self, instance: &str) -> Option<usize> {
        self.user_term_index.iter().position(|inst| inst == instance)
    }
    
    /// Map a positional coordinate to its user instance
    /// Returns the user instance for the given 0-based position index
    pub fn instance_from_position(&self, position: usize) -> Option<&str> {
        self.user_term_index.get(position).map(|s| s.as_str())
    }
    
    /// Map a position to its canonical term (alias for canonical_term_from_position)
    /// Returns the canonical term for the given 0-based position index
    pub fn position_to_canonical_term(&self, position: usize) -> Option<&str> {
        self.canonical_term_from_position(position)
    }
    
    /// Map a canonical term to its position (alias for canonical_term_to_position)
    /// Returns the 0-based index for the given canonical term
    pub fn position_from_canonical_term(&self, canonical_term: &str) -> Option<usize> {
        self.canonical_term_to_position(canonical_term)
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
    
    fn canonical_terms(&self) -> Vec<String> {
        self.schema.canonical_terms().iter().map(|s| s.to_string()).collect()
    }
    
    fn user_terms(&self) -> &[String] {
        &self.user_term_index
    }
    
    fn schema(&self) -> &dyn Schema {
        &self.schema
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
        for (i, term) in self.user_term_index.iter().enumerate() {
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
        for (i, term) in self.user_term_index.iter().enumerate() {
            if term.len() > 100 {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} is too long (max 100 characters)", i + 1),
                });
            }
        }
        
        // Validate terms contain only allowed characters
        for (i, term) in self.user_term_index.iter().enumerate() {
            if !term.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} contains invalid characters", i + 1),
                });
            }
        }
        
        // Validate terms are all different (pentad should represent distinct aspects)
        for i in 0..5 {
            for j in (i + 1)..5 {
                if self.user_term_index[i].trim().to_lowercase() == self.user_term_index[j].trim().to_lowercase() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Terms {} and {} should be different to represent distinct aspects", i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        println!("\n=== {} ===", self.name);
        println!("Type: Pentad (5 terms)");
        
        // Display terms with arrows
        for (i, term) in self.user_term_index.iter().enumerate() {
            if i < self.user_term_index.len() - 1 {
                print!("{} → ", term);
            } else {
                println!("{}", term);
            }
        }
        
        println!("Schema: {}", self.schema.name());
        println!("ID: {}", &self.id[..8]);
        println!("{}", "=".repeat(self.name.len() + 8));
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
        assert_eq!(pentad.first_term(), "Quintessence");
        assert_eq!(pentad.second_term(), "Higher Potential");
        assert_eq!(pentad.third_term(), "Lower Potential");
        assert_eq!(pentad.fourth_term(), "Purpose");
        assert_eq!(pentad.fifth_term(), "Source");
        assert!(pentad.validate().is_ok());
    }

    #[test]
    fn test_canonical_terms() {
        let pentad = PentadBuilder::new()
            .name("Test")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
        
        let canonical = pentad.canonical_terms();
        assert_eq!(canonical.len(), 5);
        assert_eq!(canonical[0], "Quintessence");
        assert_eq!(canonical[1], "Higher Potential");
        assert_eq!(canonical[2], "Lower Potential");
        assert_eq!(canonical[3], "Purpose");
        assert_eq!(canonical[4], "Source");
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
        assert_eq!(pentad.connectives().len(), 1);
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
        
        let terms = pentad.user_terms();
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
        assert_eq!(pentad.user_terms().len(), 5);
        assert!(pentad.validate().is_ok());
    }
} 