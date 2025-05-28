#[derive(Debug)]
pub struct Octad {
    pub name: String,
    pub smallest_significant_holon: String,
    pub critical_functions: String,
    pub supportive_platform: String,
    pub necessary_resourcing: String,
    pub integrative_totality: String,
    pub inherent_values: String,
    pub intrinsic_nature: String,
    pub organisational_modes: String,
}

impl Octad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Completedness"; // Or "Completeness"

    /// Creates a new Octad.
    pub fn new(
        name: &str,
        smallest_significant_holon: &str,
        critical_functions: &str,
        supportive_platform: &str,
        necessary_resourcing: &str,
        integrative_totality: &str,
        inherent_values: &str,
        intrinsic_nature: &str,
        organisational_modes: &str,
    ) -> Self {
        Octad {
            name: name.to_string(),
            smallest_significant_holon: smallest_significant_holon.to_string(),
            critical_functions: critical_functions.to_string(),
            supportive_platform: supportive_platform.to_string(),
            necessary_resourcing: necessary_resourcing.to_string(),
            integrative_totality: integrative_totality.to_string(),
            inherent_values: inherent_values.to_string(),
            intrinsic_nature: intrinsic_nature.to_string(),
            organisational_modes: organisational_modes.to_string(),
        }
    }
} 