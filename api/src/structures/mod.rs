pub mod monad;
pub mod dyad;
pub mod triad;
pub mod tetrad;
pub mod pentad;
pub mod hexad;
pub mod heptad;
pub mod octad;
pub mod ennead;
pub mod decad;
pub mod undecad;
pub mod dodecad;

// Re-export all structure types
pub use monad::{Monad, MonadBuilder};
pub use dyad::{Dyad, DyadBuilder};
pub use triad::{Triad, TriadBuilder};
pub use tetrad::{Tetrad, TetradBuilder};
pub use pentad::{Pentad, PentadBuilder};
pub use hexad::{Hexad, HexadBuilder};
pub use heptad::{Heptad, HeptadBuilder};
pub use octad::{Octad, OctadBuilder};
pub use ennead::{Ennead, EnneadBuilder};
pub use decad::{Decad, DecadBuilder};
pub use undecad::{Undecad, UndecadBuilder};
pub use dodecad::{Dodecad, DodecadBuilder}; 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SystematicStructure;

    #[test]
    fn test_structure_refactoring_new_methods() {
        // Test Monad
        let monad = MonadBuilder::new()
            .name("Test Monad")
            .term("Unity")
            .build()
            .unwrap();
        
        assert_eq!(monad.coherence_attribute(), "Universality");
        assert_eq!(monad.term_designation(), "Totality");
        assert_eq!(monad.first_order_connectives_name(), "Connectionless unity");

        // Test Dyad
        let dyad = DyadBuilder::new()
            .name("Test Dyad")
            .terms("A".to_string(), "B".to_string())
            .build()
            .unwrap();
            
        assert_eq!(dyad.coherence_attribute(), "Complimentarity");
        assert_eq!(dyad.term_designation(), "Poles");
        assert_eq!(dyad.first_order_connectives_name(), "Force");

        // Test Triad
        let triad = TriadBuilder::new()
            .name("Test Triad")
            .terms("A".to_string(), "B".to_string(), "C".to_string())
            .build()
            .unwrap();
            
        assert_eq!(triad.coherence_attribute(), "Dynamism");
        assert_eq!(triad.term_designation(), "Impulses");
        assert_eq!(triad.first_order_connectives_name(), "Acts");

        // Test Tetrad
        let tetrad = TetradBuilder::new()
            .name("Test Tetrad")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string())
            .build()
            .unwrap();
            
        assert_eq!(tetrad.coherence_attribute(), "Activity Field");
        assert_eq!(tetrad.term_designation(), "Sources");
        assert_eq!(tetrad.first_order_connectives_name(), "Interplays");

        // Test Pentad
        let pentad = PentadBuilder::new()
            .name("Test Pentad")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
            
        assert_eq!(pentad.coherence_attribute(), "Significance and Potential");
        assert_eq!(pentad.term_designation(), "Limits");
        assert_eq!(pentad.first_order_connectives_name(), "Mutualities");

        // Test Hexad
        let hexad = HexadBuilder::new()
            .name("Test Hexad")
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string())
            .build()
            .unwrap();
            
        assert_eq!(hexad.coherence_attribute(), "Coalescence");
        assert_eq!(hexad.term_designation(), "Laws");
        assert_eq!(hexad.first_order_connectives_name(), "Connectives");

        // Test Heptad
        let heptad = HeptadBuilder::new()
            .name("Test Heptad".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string())
            .build()
            .unwrap();
            
        assert_eq!(heptad.coherence_attribute(), "Transformation");
        assert_eq!(heptad.term_designation(), "States");
        assert_eq!(heptad.first_order_connectives_name(), "Connectives");

        // Test Octad
        let octad = OctadBuilder::new()
            .name("Test Octad".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string())
            .build()
            .unwrap();
            
        assert_eq!(octad.coherence_attribute(), "Self-Sufficiency");
        assert_eq!(octad.term_designation(), "Elements");
        assert_eq!(octad.first_order_connectives_name(), "Connectives");

        // Test Ennead
        let ennead = EnneadBuilder::new()
            .name("Test Ennead".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string(), "I".to_string())
            .build()
            .unwrap();
            
        assert_eq!(ennead.coherence_attribute(), "Transformation");
        assert_eq!(ennead.term_designation(), "Elements");
        assert_eq!(ennead.first_order_connectives_name(), "Connectives");

        // Test Decad
        let decad = DecadBuilder::new()
            .name("Test Decad".to_string())
            .terms([
                "A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), 
                "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string(), 
                "I".to_string(), "J".to_string()
            ])
            .build()
            .unwrap();
            
        assert_eq!(decad.coherence_attribute(), "Intrinsic Harmony");
        assert_eq!(decad.term_designation(), "Elements");
        assert_eq!(decad.first_order_connectives_name(), "Connectives");

        // Test Undecad
        let undecad = UndecadBuilder::new()
            .name("Test Undecad".to_string())
            .terms([
                "A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), 
                "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string(), 
                "I".to_string(), "J".to_string(), "K".to_string()
            ])
            .build()
            .unwrap();
            
        assert_eq!(undecad.coherence_attribute(), "Articulate Symmetry");
        assert_eq!(undecad.term_designation(), "Elements");
        assert_eq!(undecad.first_order_connectives_name(), "Connectives");

        // Test Dodecad
        let dodecad = DodecadBuilder::new()
            .name("Test Dodecad".to_string())
            .terms([
                "A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), 
                "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string(), 
                "I".to_string(), "J".to_string(), "K".to_string(), "L".to_string()
            ])
            .build()
            .unwrap();
            
        assert_eq!(dodecad.coherence_attribute(), "Harmony");
        assert_eq!(dodecad.term_designation(), "Tones");
        assert_eq!(dodecad.first_order_connectives_name(), "Connectives");
        
        // Verify new methods return non-empty strings for all tested structures
        assert!(!monad.coherence_attribute().is_empty());
        assert!(!monad.term_designation().is_empty());
        assert!(!monad.first_order_connectives_name().is_empty());
        
        assert!(!dyad.coherence_attribute().is_empty());
        assert!(!dyad.term_designation().is_empty());
        assert!(!dyad.first_order_connectives_name().is_empty());
        
        assert!(!triad.coherence_attribute().is_empty());
        assert!(!triad.term_designation().is_empty());
        assert!(!triad.first_order_connectives_name().is_empty());
        
        assert!(!tetrad.coherence_attribute().is_empty());
        assert!(!tetrad.term_designation().is_empty());
        assert!(!tetrad.first_order_connectives_name().is_empty());
        
        assert!(!pentad.coherence_attribute().is_empty());
        assert!(!pentad.term_designation().is_empty());
        assert!(!pentad.first_order_connectives_name().is_empty());
        
        assert!(!hexad.coherence_attribute().is_empty());
        assert!(!hexad.term_designation().is_empty());
        assert!(!hexad.first_order_connectives_name().is_empty());
        
        assert!(!heptad.coherence_attribute().is_empty());
        assert!(!heptad.term_designation().is_empty());
        assert!(!heptad.first_order_connectives_name().is_empty());
        
        assert!(!octad.coherence_attribute().is_empty());
        assert!(!octad.term_designation().is_empty());
        assert!(!octad.first_order_connectives_name().is_empty());
        
        assert!(!ennead.coherence_attribute().is_empty());
        assert!(!ennead.term_designation().is_empty());
        assert!(!ennead.first_order_connectives_name().is_empty());
        
        assert!(!decad.coherence_attribute().is_empty());
        assert!(!decad.term_designation().is_empty());
        assert!(!decad.first_order_connectives_name().is_empty());
        
        assert!(!undecad.coherence_attribute().is_empty());
        assert!(!undecad.term_designation().is_empty());
        assert!(!undecad.first_order_connectives_name().is_empty());
        
        assert!(!dodecad.coherence_attribute().is_empty());
        assert!(!dodecad.term_designation().is_empty());
        assert!(!dodecad.first_order_connectives_name().is_empty());
    }
} 