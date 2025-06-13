use crate::error::{Result, SystematicsError};

/// Core trait for schemas that define structure templates
pub trait Schema: Send + Sync {
    /// Number of terms this schema supports
    fn term_count(&self) -> usize;
    
    /// Get the canonical term names for this schema
    fn canonical_terms(&self) -> &'static [&'static str];
    
    /// Get the schema name/description
    fn name(&self) -> &'static str;
    
    /// Get connective relationships between terms
    fn connectives(&self) -> Vec<Connective>;
    
    /// Validate that given terms fit this schema
    fn validate_terms(&self, terms: &[String]) -> Result<()> {
        if terms.len() != self.term_count() {
            return Err(SystematicsError::InvalidTermCount {
                expected: self.term_count(),
                actual: terms.len(),
            });
        }
        Ok(())
    }
}

/// A relationship/connective between terms in a structure
#[derive(Debug, Clone)]
pub struct Connective {
    pub from_position: usize,
    pub to_position: usize,
    pub relationship: String,
    pub description: Option<String>,
}

/// Provides access to schemas for different structure types
pub trait SchemaProvider {
    fn get_schema(&self, term_count: usize) -> Option<Box<dyn Schema>>;
}

/// Bennett's canonical schemas
pub struct BennettSchemas;

impl SchemaProvider for BennettSchemas {
    fn get_schema(&self, term_count: usize) -> Option<Box<dyn Schema>> {
        match term_count {
            1 => Some(Box::new(MonadSchema)),
            2 => Some(Box::new(DyadSchema)),
            3 => Some(Box::new(TriadSchema)),
            4 => Some(Box::new(TetradSchema)),
            5 => Some(Box::new(PentadSchema)),
            6 => Some(Box::new(HexadSchema)),
            7 => Some(Box::new(HeptadSchema)),
            8 => Some(Box::new(OctadSchema)),
            12 => Some(Box::new(DodecadSchema)),
            _ => None,
        }
    }
}

// Schema implementations
#[derive(Debug, Clone)]
pub struct MonadSchema;
impl Schema for MonadSchema {
    fn term_count(&self) -> usize { 1 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Unity"] }
    fn name(&self) -> &'static str { "Monad Schema" }
    fn connectives(&self) -> Vec<Connective> { vec![] }
}

#[derive(Debug, Clone)]
pub struct DyadSchema;
impl Schema for DyadSchema {
    fn term_count(&self) -> usize { 2 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Essence", "Existence"] }
    fn name(&self) -> &'static str { "Dyad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective {
                from_position: 0,
                to_position: 1,
                relationship: "manifests as".to_string(),
                description: Some("Essence manifests as Existence".to_string()),
            }
        ]
    }
}

#[derive(Debug, Clone)]
pub struct TriadSchema;
impl Schema for TriadSchema {
    fn term_count(&self) -> usize { 3 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Will", "Function", "Being"] }
    fn name(&self) -> &'static str { "Triad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective {
                from_position: 0,
                to_position: 1,
                relationship: "active force".to_string(),
                description: Some("Will as active force on Function".to_string()),
            },
            Connective {
                from_position: 1,
                to_position: 2,
                relationship: "passive force".to_string(),
                description: Some("Function as passive force on Being".to_string()),
            },
            Connective {
                from_position: 2,
                to_position: 0,
                relationship: "reconciling force".to_string(),
                description: Some("Being as reconciling force".to_string()),
            },
        ]
    }
}

// Placeholder schemas for higher-order structures
macro_rules! create_schema {
    ($name:ident, $count:expr, $canonical:expr, $schema_name:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name;
        impl Schema for $name {
            fn term_count(&self) -> usize { $count }
            fn canonical_terms(&self) -> &'static [&'static str] { $canonical }
            fn name(&self) -> &'static str { $schema_name }
            fn connectives(&self) -> Vec<Connective> { 
                // TODO: Implement specific connectives for each schema
                vec![]
            }
        }
    };
}

create_schema!(TetradSchema, 4, &["Source", "Courses", "Impulse", "Control"], "Tetrad Schema");
create_schema!(PentadSchema, 5, &["Potential", "Receptivity", "Sensitivity", "Creativity", "Polarity"], "Pentad Schema");
create_schema!(HexadSchema, 6, &["Freedom", "Harmony", "Structure", "Uniqueness", "Dominance", "Submission"], "Hexad Schema");
create_schema!(HeptadSchema, 7, &["Insight", "Reason", "Knowledge", "Understanding", "Wisdom", "Compassion", "Value"], "Heptad Schema");
create_schema!(OctadSchema, 8, &["Significant Holon", "Structural Modes", "Organisational Modes", "Exchange Modes", "Constitutional Modes", "Creative Modes", "Unitive Modes", "Transcendent Modes"], "Octad Schema");
create_schema!(DodecadSchema, 12, &["Autocracy", "Domination", "Authority", "Leadership", "Responsibility", "Justice", "Creativity", "Love", "Wisdom", "Compassion", "Hope", "Wholeness"], "Dodecad Schema"); 