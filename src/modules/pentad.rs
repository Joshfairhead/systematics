#[derive(Debug)]
pub struct Pentad {
    pub name: String,
    pub intrinsiclimit: String,
    pub innerupperlimit: String,
    pub innerlowerlimit: String,
    pub outerupperlimit: String,
    pub outerlowerlimit: String,
}

impl Pentad {
    pub const TERM_ATTRIBUTE_DESCRIPTION: &'static str = "Quintessence or significance";

    /// Creates a new Pentad.
    pub fn new(
        name: &str,
        intrinsiclimit: &str,
        innerupperlimit: &str,
        innerlowerlimit: &str,
        outerupperlimit: &str,
        outerlowerlimit: &str,
    ) -> Self {
        Pentad {
            name: name.to_string(),
            intrinsiclimit: intrinsiclimit.to_string(),
            innerupperlimit: innerupperlimit.to_string(),
            innerlowerlimit: innerlowerlimit.to_string(),
            outerupperlimit: outerupperlimit.to_string(),
            outerlowerlimit: outerlowerlimit.to_string(),
        }
    }
} 