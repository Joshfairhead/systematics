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

    /// Creates a new Dodecad.
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
} 