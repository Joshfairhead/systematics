use crate::{
    SystematicStructure, 
    schemas::{Schema, MonadSchema}, 
    error::{Result, SystematicsError}
};
use uuid::Uuid;

/// A monadic structure - the simplest systematic structure with one term
#[derive(Debug, Clone)]
pub struct Monad {
    id: String,
    name: String,
    term: String,
    attributes: Vec<String>,
    schema: MonadSchema,
}

impl Monad {
    pub fn new(name: String, term: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            term,
            attributes: Vec::new(),
            schema: MonadSchema,
        }
    }
    
    pub fn with_attributes(mut self, attributes: Vec<String>) -> Self {
        self.attributes = attributes;
        self
    }
    
    pub fn add_attribute(&mut self, attribute: String) {
        self.attributes.push(attribute);
    }
    
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }
}

impl SystematicStructure for Monad {
    const TERM_COUNT: usize = 1;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn terms(&self) -> &[String] {
        std::slice::from_ref(&self.term)
    }
    
    fn schema(&self) -> &dyn Schema {
        &self.schema
    }
    
    fn validate(&self) -> Result<()> {
        if self.term.trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Monad term cannot be empty".to_string(),
            });
        }
        Ok(())
    }
    
    #[cfg(feature = "serde_support")]
    fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| SystematicsError::Serialization(e.to_string()))
    }
    
    #[cfg(feature = "serde_support")]
    fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| SystematicsError::Deserialization(e.to_string()))
    }
}

/// Builder for creating Monad structures
pub struct MonadBuilder {
    name: Option<String>,
    term: Option<String>,
    attributes: Vec<String>,
}

impl MonadBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            term: None,
            attributes: Vec::new(),
        }
    }
    
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn term<S: Into<String>>(mut self, term: S) -> Self {
        self.term = Some(term.into());
        self
    }
    
    pub fn attribute<S: Into<String>>(mut self, attribute: S) -> Self {
        self.attributes.push(attribute.into());
        self
    }
    
    pub fn attributes<I, S>(mut self, attributes: I) -> Self 
    where 
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.attributes.extend(attributes.into_iter().map(|s| s.into()));
        self
    }
    
    pub fn build(self) -> Result<Monad> {
        let name = self.name.unwrap_or_else(|| "Untitled Monad".to_string());
        let term = self.term.ok_or_else(|| SystematicsError::Builder {
            reason: "Monad requires a term".to_string(),
        })?;
        
        let monad = Monad::new(name, term).with_attributes(self.attributes);
        monad.validate()?;
        Ok(monad)
    }
}

impl Default for MonadBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monad_creation() {
        let monad = MonadBuilder::new()
            .name("Test Monad")
            .term("Unity")
            .attribute("infinite")
            .attribute("eternal")
            .build()
            .unwrap();
            
        assert_eq!(monad.name(), "Test Monad");
        assert_eq!(monad.terms()[0], "Unity");
        assert_eq!(monad.attributes().len(), 2);
        assert!(monad.validate().is_ok());
    }
    
    #[test]
    fn test_monad_validation() {
        let result = MonadBuilder::new()
            .name("Invalid Monad")
            .term("")
            .build();
            
        assert!(result.is_err());
    }
} 