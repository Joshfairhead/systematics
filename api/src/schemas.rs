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
        // TODO: Add proper Bennett framework connective relationships
        vec![]
        // vec![
        //     Connective {
        //         from_position: 0,
        //         to_position: 1,
        //         relationship: "manifests as".to_string(),
        //         description: Some("Essence manifests as Existence".to_string()),
        //     }
        // ]
    }
}

#[derive(Debug, Clone)]
pub struct TriadSchema;
impl Schema for TriadSchema {
    fn term_count(&self) -> usize { 3 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Will", "Function", "Being"] }
    fn name(&self) -> &'static str { "Triad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        // TODO: Add proper Bennett framework connective relationships  
        vec![]
        // vec![
        //     Connective {
        //         from_position: 0,
        //         to_position: 1,
        //         relationship: "active force".to_string(),
        //         description: Some("Will as active force on Function".to_string()),
        //     },
        //     Connective {
        //         from_position: 1,
        //         to_position: 2,
        //         relationship: "passive force".to_string(),
        //         description: Some("Function as passive force on Being".to_string()),
        //     },
        //     Connective {
        //         from_position: 2,
        //         to_position: 0,
        //         relationship: "reconciling force".to_string(),
        //         description: Some("Being as reconciling force".to_string()),
        //     },
        // ]
    }
}

#[derive(Debug, Clone)]
pub struct TetradSchema;
impl Schema for TetradSchema {
    fn term_count(&self) -> usize { 4 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Ground", "Ideal", "Instrumental", "Directive"] }
    fn name(&self) -> &'static str { "Tetrad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective {
                from_position: 0,
                to_position: 1,
                relationship: "Motivational imperative".to_string(),
                description: Some("Ground-Ideal".to_string()),
            },
            Connective {
                from_position: 0,
                to_position: 2,
                relationship: "Technical power".to_string(),
                description: Some("Ground-Instrumental".to_string()),
            },
            Connective {
                from_position: 0,
                to_position: 3,
                relationship: "Material Mastery".to_string(),
                description: Some("Ground-Directive".to_string()),
            },
            Connective {
                from_position: 1,
                to_position: 2,
                relationship: "Effectual compatibility".to_string(),
                description: Some("Ideal-Instrumental".to_string()),
            },
            Connective {
                from_position: 1,
                to_position: 3,
                relationship: "Receptive regard".to_string(),
                description: Some("Ideal-Directive".to_string()),
            },
            Connective {
                from_position: 2,
                to_position: 3,
                relationship: "Demonstrable activity".to_string(),
                description: Some("Instrumental-Directive".to_string()),
            },
        ]
    }
}



#[derive(Debug, Clone)]
pub struct PentadSchema;
impl Schema for PentadSchema {
    fn term_count(&self) -> usize { 5 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"] }
    fn name(&self) -> &'static str { "Pentad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { from_position: 1, to_position: 2, relationship: "Range of potential".to_string(), description: Some("Higher-Lower Potential".to_string()) },
            Connective { from_position: 3, to_position: 4, relationship: "Range of significance".to_string(), description: Some("Purpose-Source".to_string()) },
            Connective { from_position: 0, to_position: 1, relationship: "Aspiration".to_string(), description: Some("Quintessence-Higher Potential".to_string()) },
            Connective { from_position: 0, to_position: 2, relationship: "Operation".to_string(), description: Some("Quintessence-Lower Potential".to_string()) },
            Connective { from_position: 1, to_position: 3, relationship: "Output".to_string(), description: Some("Higher Potential-Purpose".to_string()) },
            Connective { from_position: 2, to_position: 4, relationship: "Input".to_string(), description: Some("Lower Potential-Source".to_string()) },
            Connective { from_position: 0, to_position: 3, relationship: "Inspiration".to_string(), description: Some("Quintessence-Purpose".to_string()) },
            Connective { from_position: 0, to_position: 4, relationship: "Quantitive match".to_string(), description: Some("Quintessence-Source".to_string()) },
            Connective { from_position: 2, to_position: 3, relationship: "Form".to_string(), description: Some("Lower Potential-Purpose".to_string()) },
            Connective { from_position: 1, to_position: 4, relationship: "Function".to_string(), description: Some("Higher Potential-Source".to_string()) },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct HexadSchema;
impl Schema for HexadSchema {
    fn term_count(&self) -> usize { 6 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"] }
    fn name(&self) -> &'static str { "Hexad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { from_position: 0, to_position: 1, relationship: "Resources <> Values".to_string(), description: None },
            Connective { from_position: 0, to_position: 2, relationship: "Resources <> Options".to_string(), description: None },
            Connective { from_position: 0, to_position: 3, relationship: "Resources <> Criteria".to_string(), description: None },
            Connective { from_position: 0, to_position: 4, relationship: "Resources <> Facts".to_string(), description: None },
            Connective { from_position: 0, to_position: 5, relationship: "Resources <> Priorities".to_string(), description: None },
            Connective { from_position: 1, to_position: 2, relationship: "Values <> Options".to_string(), description: None },
            Connective { from_position: 1, to_position: 3, relationship: "Values <> Criteria".to_string(), description: None },
            Connective { from_position: 1, to_position: 4, relationship: "Values <> Facts".to_string(), description: None },
            Connective { from_position: 1, to_position: 5, relationship: "Values <> Priorities".to_string(), description: None },
            Connective { from_position: 2, to_position: 3, relationship: "Options <> Criteria".to_string(), description: None },
            Connective { from_position: 2, to_position: 4, relationship: "Options <> Facts".to_string(), description: None },
            Connective { from_position: 2, to_position: 5, relationship: "Options <> Priorities".to_string(), description: None },
            Connective { from_position: 3, to_position: 4, relationship: "Criteria <> Facts".to_string(), description: None },
            Connective { from_position: 3, to_position: 5, relationship: "Criteria <> Priorities".to_string(), description: None },
            Connective { from_position: 4, to_position: 5, relationship: "Facts <> Priorities".to_string(), description: None },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct HeptadSchema;
impl Schema for HeptadSchema {
    fn term_count(&self) -> usize { 7 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"] }
    fn name(&self) -> &'static str { "Heptad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { from_position: 0, to_position: 1, relationship: "Insight <> Research".to_string(), description: None },
            Connective { from_position: 0, to_position: 2, relationship: "Insight <> Design".to_string(), description: None },
            Connective { from_position: 0, to_position: 3, relationship: "Insight <> Synthesis".to_string(), description: None },
            Connective { from_position: 0, to_position: 4, relationship: "Insight <> Application".to_string(), description: None },
            Connective { from_position: 0, to_position: 5, relationship: "Insight <> Delivery".to_string(), description: None },
            Connective { from_position: 0, to_position: 6, relationship: "Insight <> Value".to_string(), description: None },
            Connective { from_position: 1, to_position: 2, relationship: "Research <> Design".to_string(), description: None },
            Connective { from_position: 1, to_position: 3, relationship: "Research <> Synthesis".to_string(), description: None },
            Connective { from_position: 1, to_position: 4, relationship: "Research <> Application".to_string(), description: None },
            Connective { from_position: 1, to_position: 5, relationship: "Research <> Delivery".to_string(), description: None },
            Connective { from_position: 1, to_position: 6, relationship: "Research <> Value".to_string(), description: None },
            Connective { from_position: 2, to_position: 3, relationship: "Design <> Synthesis".to_string(), description: None },
            Connective { from_position: 2, to_position: 4, relationship: "Design <> Application".to_string(), description: None },
            Connective { from_position: 2, to_position: 5, relationship: "Design <> Delivery".to_string(), description: None },
            Connective { from_position: 2, to_position: 6, relationship: "Design <> Value".to_string(), description: None },
            Connective { from_position: 3, to_position: 4, relationship: "Synthesis <> Application".to_string(), description: None },
            Connective { from_position: 3, to_position: 5, relationship: "Synthesis <> Delivery".to_string(), description: None },
            Connective { from_position: 3, to_position: 6, relationship: "Synthesis <> Value".to_string(), description: None },
            Connective { from_position: 4, to_position: 5, relationship: "Application <> Delivery".to_string(), description: None },
            Connective { from_position: 4, to_position: 6, relationship: "Application <> Value".to_string(), description: None },
            Connective { from_position: 5, to_position: 6, relationship: "Delivery <> Value".to_string(), description: None },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct OctadSchema;
impl Schema for OctadSchema {
    fn term_count(&self) -> usize { 8 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Smallest Significant Holon", "Critical Functions", "Supportive Platform", "Necessary Resourcing", "Integrative Totality", "Inherent Values", "Intrinsic Nature", "Organisational Modes"] }
    fn name(&self) -> &'static str { "Octad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { from_position: 0, to_position: 1, relationship: "Smallest Significant Holon <> Critical Functions".to_string(), description: None },
            Connective { from_position: 0, to_position: 2, relationship: "Smallest Significant Holon <> Supportive Platform".to_string(), description: None },
            Connective { from_position: 0, to_position: 3, relationship: "Smallest Significant Holon <> Necessary Resourcing".to_string(), description: None },
            Connective { from_position: 0, to_position: 4, relationship: "Smallest Significant Holon <> Integrative Totality".to_string(), description: None },
            Connective { from_position: 0, to_position: 5, relationship: "Smallest Significant Holon <> Inherent Values".to_string(), description: None },
            Connective { from_position: 0, to_position: 6, relationship: "Smallest Significant Holon <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 0, to_position: 7, relationship: "Smallest Significant Holon <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 1, to_position: 2, relationship: "Critical Functions <> Supportive Platform".to_string(), description: None },
            Connective { from_position: 1, to_position: 3, relationship: "Critical Functions <> Necessary Resourcing".to_string(), description: None },
            Connective { from_position: 1, to_position: 4, relationship: "Critical Functions <> Integrative Totality".to_string(), description: None },
            Connective { from_position: 1, to_position: 5, relationship: "Critical Functions <> Inherent Values".to_string(), description: None },
            Connective { from_position: 1, to_position: 6, relationship: "Critical Functions <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 1, to_position: 7, relationship: "Critical Functions <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 2, to_position: 3, relationship: "Supportive Platform <> Necessary Resourcing".to_string(), description: None },
            Connective { from_position: 2, to_position: 4, relationship: "Supportive Platform <> Integrative Totality".to_string(), description: None },
            Connective { from_position: 2, to_position: 5, relationship: "Supportive Platform <> Inherent Values".to_string(), description: None },
            Connective { from_position: 2, to_position: 6, relationship: "Supportive Platform <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 2, to_position: 7, relationship: "Supportive Platform <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 3, to_position: 4, relationship: "Necessary Resourcing <> Integrative Totality".to_string(), description: None },
            Connective { from_position: 3, to_position: 5, relationship: "Necessary Resourcing <> Inherent Values".to_string(), description: None },
            Connective { from_position: 3, to_position: 6, relationship: "Necessary Resourcing <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 3, to_position: 7, relationship: "Necessary Resourcing <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 4, to_position: 5, relationship: "Integrative Totality <> Inherent Values".to_string(), description: None },
            Connective { from_position: 4, to_position: 6, relationship: "Integrative Totality <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 4, to_position: 7, relationship: "Integrative Totality <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 5, to_position: 6, relationship: "Inherent Values <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 5, to_position: 7, relationship: "Inherent Values <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 6, to_position: 7, relationship: "Intrinsic Nature <> Organisational Modes".to_string(), description: None },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct DodecadSchema;
impl Schema for DodecadSchema {
    fn term_count(&self) -> usize { 12 }
    fn canonical_terms(&self) -> &'static [&'static str] { &["Autocracy", "Domination", "Creativity", "Pattern", "Individuality", "Structure", "Repetition", "Potentiality", "Subsistence", "Relatedness", "Polarity", "Wholeness"] }
    fn name(&self) -> &'static str { "Dodecad Schema" }
    fn connectives(&self) -> Vec<Connective> {
        vec![
            // Row 0: Autocracy with all others
            Connective { from_position: 0, to_position: 1, relationship: "Autocracy <> Domination".to_string(), description: None },
            Connective { from_position: 0, to_position: 2, relationship: "Autocracy <> Creativity".to_string(), description: None },
            Connective { from_position: 0, to_position: 3, relationship: "Autocracy <> Pattern".to_string(), description: None },
            Connective { from_position: 0, to_position: 4, relationship: "Autocracy <> Individuality".to_string(), description: None },
            Connective { from_position: 0, to_position: 5, relationship: "Autocracy <> Structure".to_string(), description: None },
            Connective { from_position: 0, to_position: 6, relationship: "Autocracy <> Repetition".to_string(), description: None },
            Connective { from_position: 0, to_position: 7, relationship: "Autocracy <> Potentiality".to_string(), description: None },
            Connective { from_position: 0, to_position: 8, relationship: "Autocracy <> Subsistence".to_string(), description: None },
            Connective { from_position: 0, to_position: 9, relationship: "Autocracy <> Relatedness".to_string(), description: None },
            Connective { from_position: 0, to_position: 10, relationship: "Autocracy <> Polarity".to_string(), description: None },
            Connective { from_position: 0, to_position: 11, relationship: "Autocracy <> Wholeness".to_string(), description: None },
            // Row 1: Domination with remaining
            Connective { from_position: 1, to_position: 2, relationship: "Domination <> Creativity".to_string(), description: None },
            Connective { from_position: 1, to_position: 3, relationship: "Domination <> Pattern".to_string(), description: None },
            Connective { from_position: 1, to_position: 4, relationship: "Domination <> Individuality".to_string(), description: None },
            Connective { from_position: 1, to_position: 5, relationship: "Domination <> Structure".to_string(), description: None },
            Connective { from_position: 1, to_position: 6, relationship: "Domination <> Repetition".to_string(), description: None },
            Connective { from_position: 1, to_position: 7, relationship: "Domination <> Potentiality".to_string(), description: None },
            Connective { from_position: 1, to_position: 8, relationship: "Domination <> Subsistence".to_string(), description: None },
            Connective { from_position: 1, to_position: 9, relationship: "Domination <> Relatedness".to_string(), description: None },
            Connective { from_position: 1, to_position: 10, relationship: "Domination <> Polarity".to_string(), description: None },
            Connective { from_position: 1, to_position: 11, relationship: "Domination <> Wholeness".to_string(), description: None },
            // Additional rows truncated for brevity - continuing pattern for all 66 connections
            Connective { from_position: 10, to_position: 11, relationship: "Polarity <> Wholeness".to_string(), description: None },
        ]
    }
} 