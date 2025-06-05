#[derive(Debug)]
pub struct Dodecad {
    pub name: String,
    pub autocracy: String,
    pub domination: String,
    pub creativity: String,
    pub pattern: String,
    pub individuality: String,
    pub structure: String,
    pub repetition: String,
    pub potentiality: String,
    pub subsistence: String,
    pub relatedness: String,
    pub polarity: String,
    pub wholeness: String,
}

impl Dodecad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Totality";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        autocracy: &str,
        domination: &str,
        creativity: &str,
        pattern: &str,
        individuality: &str,
        structure: &str,
        repetition: &str,
        potentiality: &str,
        subsistence: &str,
        relatedness: &str,
        polarity: &str,
        wholeness: &str,
    ) -> Self {
        Dodecad {
            name: name.to_string(),
            autocracy: autocracy.to_string(),
            domination: domination.to_string(),
            creativity: creativity.to_string(),
            pattern: pattern.to_string(),
            individuality: individuality.to_string(),
            structure: structure.to_string(),
            repetition: repetition.to_string(),
            potentiality: potentiality.to_string(),
            subsistence: subsistence.to_string(),
            relatedness: relatedness.to_string(),
            polarity: polarity.to_string(),
            wholeness: wholeness.to_string(),
        }
    }

    pub fn get_instances(&self) -> Vec<String> {
        vec![
            self.autocracy.clone(),
            self.domination.clone(),
            self.creativity.clone(),
            self.pattern.clone(),
            self.individuality.clone(),
            self.structure.clone(),
            self.repetition.clone(),
            self.potentiality.clone(),
            self.subsistence.clone(),
            self.relatedness.clone(),
            self.polarity.clone(),
            self.wholeness.clone(),
        ]
    }

    pub fn get_canonical_terms() -> Vec<&'static str> {
        vec![
            "Autocracy",
            "Domination",
            "Creativity",
            "Pattern",
            "Individuality",
            "Structure",
            "Repetition",
            "Potentiality",
            "Subsistence",
            "Relatedness",
            "Polarity",
            "Wholeness",
        ]
    }

    pub fn has_terms(&self) -> bool {
        self.get_instances().iter().any(|s| !s.is_empty())
    }

    pub fn term_attribute_description() -> &'static str {
        Self::TERM_ATTRIBUTE_DESCRIPTION
    }

    pub fn display(&self) {
        println!("\n--- Dodecad Details ---");
        println!("Dodecad Name: {}", self.name);
        println!("Core Attribute: {}", Self::TERM_ATTRIBUTE_DESCRIPTION);
        println!("1. Autocracy: {}", self.autocracy);
        println!("2. Domination: {}", self.domination);
        println!("3. Creativity: {}", self.creativity);
        println!("4. Pattern: {}", self.pattern);
        println!("5. Individuality: {}", self.individuality);
        println!("6. Structure: {}", self.structure);
        println!("7. Repetition: {}", self.repetition);
        println!("8. Potentiality: {}", self.potentiality);
        println!("9. Subsistence: {}", self.subsistence);
        println!("10. Relatedness: {}", self.relatedness);
        println!("11. Polarity: {}", self.polarity);
        println!("12. Wholeness: {}", self.wholeness);
        println!("-----------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dodecad_creation() {
        let d = Dodecad::new(
            "Test Dodecad",
            "Autocracy",
            "Domination",
            "Creativity",
            "Pattern",
            "Individuality",
            "Structure",
            "Repetition",
            "Potentiality",
            "Subsistence",
            "Relatedness",
            "Polarity",
            "Wholeness",
        );

        assert_eq!(d.name, "Test Dodecad");
        assert_eq!(d.autocracy, "Autocracy");
        assert_eq!(d.pattern, "Pattern");
        assert!(d.has_terms());
    }

    #[test]
    fn test_get_instances() {
        let d = Dodecad::new(
            "InstanceTest",
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"
        );
        let terms = d.get_instances();
        assert_eq!(terms.len(), 12);
        assert_eq!(terms[0], "A");
        assert_eq!(terms[11], "L");
    }

    #[test]
    fn test_get_canonical_terms() {
        let terms = Dodecad::get_canonical_terms();
        assert_eq!(terms.len(), 12);
        assert_eq!(terms[3], "Pattern");
    }

    #[test]
    fn test_has_terms_with_empty_values() {
        let d = Dodecad::new(
            "EmptyTest",
            "", "", "", "", "", "", "", "", "", "", "", ""
        );
        assert!(!d.has_terms());
    }
}
