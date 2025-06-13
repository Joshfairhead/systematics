use crate::schemas::StructureSchema;

/// Bennett's Pentad Schema - Quintessence, Higher Potential, Lower Potential, Purpose, Source
pub struct BennettPentadSchema;

impl StructureSchema for BennettPentadSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"]
    }
    
    fn get_position_description(&self, pos: usize) -> Option<&'static str> {
        match pos {
            0 => Some("The essential nature or intrinsic limit"),
            1 => Some("The higher capacity or inner upper limit"),
            2 => Some("The lower capacity or inner lower limit"),
            3 => Some("The intended goal or outer upper limit"),
            4 => Some("The origin or outer lower limit"),
            _ => None,
        }
    }
    
    fn get_connective_label(&self, i: usize, j: usize) -> Option<&'static str> {
        match (i, j) {
            (1, 2) => Some("Range of potential"),        // B<>C: Higher-Lower Potential
            (3, 4) => Some("Range of significance"),     // D<>E: Purpose-Source
            (0, 1) => Some("Aspiration"),                // A<>B: Quintessence-Higher Potential
            (0, 2) => Some("Operation"),                 // A<>C: Quintessence-Lower Potential
            (1, 3) => Some("Output"),                    // B<>D: Higher Potential-Purpose
            (2, 4) => Some("Input"),                     // C<>E: Lower Potential-Source
            (0, 3) => Some("Inspiration"),               // A<>D: Quintessence-Purpose
            (0, 4) => Some("Quantitive match"),          // A<>E: Quintessence-Source
            (2, 3) => Some("Form"),                      // C<>D: Lower Potential-Purpose
            (1, 4) => Some("Function"),                  // B<>E: Higher Potential-Source
            _ => None,
        }
    }
    
    fn get_attribute_description(&self) -> &'static str {
        "Quintessence or significance"
    }
    
    fn get_schema_name(&self) -> &'static str {
        "Bennett's Pentad"
    }
    
    fn get_structure_name(&self) -> &'static str {
        "Pentad"
    }
    
    fn get_position_count(&self) -> usize {
        5
    }
} 