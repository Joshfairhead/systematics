mod modules; 

use crate::modules::monad::Monad;
use crate::modules::dyad::Dyad;
use crate::modules::triad::Triad;
use crate::modules::tetrad::Tetrad;
use crate::modules::pentad::Pentad;
use crate::modules::hexad::Hexad;
use crate::modules::heptad::Heptad;
use crate::modules::octad::Octad;
use crate::modules::dodecad::Dodecad;
use std::io::{self, Write}; // Import for input/output

// Helper macro to reduce repetition for input gathering
macro_rules! get_input {
    ($prompt:expr, $failure_msg:expr, $default_val:expr) => {{
        let mut input_str = String::new();
        print!($prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_str).expect($failure_msg);
        let trimmed = input_str.trim();
        if trimmed.is_empty() { $default_val.to_string() } else { trimmed.to_string() }
    }};
}

fn main() {
    println!("How many terms in your system? (1, 2, 3, 4, 5, 6, 7, 8, or 12)");
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read choice");

    match choice_input.trim().parse::<u32>() {
        Ok(num_terms) => match num_terms {
            1 => create_monad(),
            2 => create_dyad(),
            3 => create_triad(),
            4 => { 
                match Tetrad::create_interactive() {
                    Ok(_tetrad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating tetrad: {}", e),
                }
            }
            5 => create_pentad(),
            6 => create_hexad(),
            7 => create_heptad(),
            8 => create_octad(),
            12 => create_dodecad(),
            _ => println!("Invalid number of terms. Please enter 1, 2, 3, 4, 5, 6, 7, 8, or 12."),
        },
        Err(_) => {
            println!("Invalid input. Please enter a number.");
        }
    }

    // Demo removed - tetrad interactive creation now handles everything
}

fn create_monad() {
    println!("\n--- Creating a Monad ---");

    let monad_name_str: String = get_input!("Enter a name for your Monad: ", "Failed to read name", "Unnamed Monad");

    let mut my_monad = Monad::new(&monad_name_str);

    println!("\nEnter terms for \"{}\". Press Enter on an empty line when done.", my_monad.name);
    loop {
        let mut term_input = String::new();
        print!("Term: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut term_input).expect("Failed to read term");
        let trimmed_term = term_input.trim();
        if trimmed_term.is_empty() {
            break;
        }
        my_monad.add_term(trimmed_term);
    }

    println!("\n--- Monad Details ---");
    println!("Monad Name: {}", my_monad.name);
    println!("Core Attribute: {}", Monad::TERM_ATTRIBUTE_DESCRIPTION);
    let terms = my_monad.get_all_terms();
    if !terms.is_empty() {
        println!("User-defined Terms:");
        for term in terms {
            println!("- {}", term);
        }
    } else {
        println!("No user-defined terms were added.");
    }
    println!("---------------------");
}

fn create_dyad() {
    println!("\n--- Creating a Dyad ---");

    let name_str: String = get_input!("Enter a name for your Dyad: ", "Failed to read Dyad name", "Unnamed Dyad");
    let essence_str: String = get_input!("Enter the Dyad's essence: ", "Failed to read essence", "Default Essence");
    let existence_str: String = get_input!("Enter the Dyad's existence: ", "Failed to read existence", "Default Existence");

    let my_dyad = Dyad::new(
        &name_str,
        &essence_str,
        &existence_str,
    );

    println!("\n--- Dyad Details ---");
    println!("Dyad Name: {}", my_dyad.name);
    println!("Core Attribute: {}", Dyad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Essence: {}", my_dyad.essence);
    println!("Existence: {}", my_dyad.existence);
    println!("---------------------");
}

fn create_triad() {
    println!("\n--- Creating a Triad ---");

    let name_str: String = get_input!("Enter a name for your Triad: ", "Failed to read Triad name", "Unnamed Triad");
    let active_str: String = get_input!("Enter the Triad's active aspect: ", "Failed to read active aspect", "Default Active");
    let passive_str: String = get_input!("Enter the Triad's passive aspect: ", "Failed to read passive aspect", "Default Passive");
    let reconciling_str: String = get_input!("Enter the Triad's reconciling aspect: ", "Failed to read reconciling aspect", "Default Reconciling");

    let my_triad = Triad::new(
        &name_str,
        &active_str,
        &passive_str,
        &reconciling_str,
    );

    println!("\n--- Triad Details ---");
    println!("Triad Name: {}", my_triad.name);
    println!("Core Attribute: {}", Triad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Active Term: {}", my_triad.active);
    println!("Passive Term: {}", my_triad.passive);
    println!("Reconciling Term: {}", my_triad.reconciling);
    println!("---------------------");
}

fn create_pentad() {
    println!("\n--- Creating a Pentad ---");
    let name_str: String = get_input!("Enter a name for your Pentad: ", "Failed to read Pentad name", "Unnamed Pentad");
    let intrinsiclimit_str: String = get_input!("Enter the Pentad's intrinsic limit: ", "Failed to read intrinsic limit", "Default Intrinsic Limit");
    let innerupperlimit_str: String = get_input!("Enter the Pentad's inner upper limit: ", "Failed to read inner upper limit", "Default Inner Upper Limit");
    let innerlowerlimit_str: String = get_input!("Enter the Pentad's inner lower limit: ", "Failed to read inner lower limit", "Default Inner Lower Limit");
    let outerupperlimit_str: String = get_input!("Enter the Pentad's outer upper limit: ", "Failed to read outer upper limit", "Default Outer Upper Limit");
    let outerlowerlimit_str: String = get_input!("Enter the Pentad's outer lower limit: ", "Failed to read outer lower limit", "Default Outer Lower Limit");

    let my_pentad = Pentad::new(
        &name_str,
        &intrinsiclimit_str,
        &innerupperlimit_str,
        &innerlowerlimit_str,
        &outerupperlimit_str,
        &outerlowerlimit_str,
    );

    println!("\n--- Pentad Details ---");
    println!("Pentad Name: {}", my_pentad.name);
    println!("Core Attribute: {}", Pentad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Intrinsic Limit: {}", my_pentad.intrinsiclimit);
    println!("Inner Upper Limit: {}", my_pentad.innerupperlimit);
    println!("Inner Lower Limit: {}", my_pentad.innerlowerlimit);
    println!("Outer Upper Limit: {}", my_pentad.outerupperlimit);
    println!("Outer Lower Limit: {}", my_pentad.outerlowerlimit);
    println!("---------------------");
}

fn create_hexad() {
    println!("\n--- Creating a Hexad ---");
    let name_str: String = get_input!("Enter a name for your Hexad: ", "Failed to read Hexad name", "Unnamed Hexad");
    let resources_str: String = get_input!("Enter the Hexad's resources: ", "Failed to read resources", "Default Resources");
    let values_str: String = get_input!("Enter the Hexad's values: ", "Failed to read values", "Default Values");
    let options_str: String = get_input!("Enter the Hexad's options: ", "Failed to read options", "Default Options");
    let criteria_str: String = get_input!("Enter the Hexad's criteria: ", "Failed to read criteria", "Default Criteria");
    let facts_str: String = get_input!("Enter the Hexad's facts: ", "Failed to read facts", "Default Facts");
    let priorities_str: String = get_input!("Enter the Hexad's priorities: ", "Failed to read priorities", "Default Priorities");

    let my_hexad = Hexad::new(
        &name_str,
        &resources_str,
        &values_str,
        &options_str,
        &criteria_str,
        &facts_str,
        &priorities_str,
    );

    println!("\n--- Hexad Details ---");
    println!("Hexad Name: {}", my_hexad.name);
    println!("Core Attribute: {}", Hexad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Resources: {}", my_hexad.resources);
    println!("Values: {}", my_hexad.values);
    println!("Options: {}", my_hexad.options);
    println!("Criteria: {}", my_hexad.criteria);
    println!("Facts: {}", my_hexad.facts);
    println!("Priorities: {}", my_hexad.priorities);
    println!("---------------------");
}

fn create_heptad() {
    println!("\n--- Creating a Heptad ---");
    let name_str: String = get_input!("Enter a name for your Heptad: ", "Failed to read Heptad name", "Unnamed Heptad");
    let insight_str: String = get_input!("Enter the Heptad's insight: ", "Failed to read insight", "Default Insight");
    let research_str: String = get_input!("Enter the Heptad's research: ", "Failed to read research", "Default Research");
    let design_str: String = get_input!("Enter the Heptad's design: ", "Failed to read design", "Default Design");
    let synthesis_str: String = get_input!("Enter the Heptad's synthesis: ", "Failed to read synthesis", "Default Synthesis");
    let application_str: String = get_input!("Enter the Heptad's application: ", "Failed to read application", "Default Application");
    let delivery_str: String = get_input!("Enter the Heptad's delivery: ", "Failed to read delivery", "Default Delivery");
    let value_str: String = get_input!("Enter the Heptad's value: ", "Failed to read value", "Default Value");

    let my_heptad = Heptad::new(
        &name_str,
        &insight_str,
        &research_str,
        &design_str,
        &synthesis_str,
        &application_str,
        &delivery_str,
        &value_str,
    );

    println!("\n--- Heptad Details ---");
    println!("Heptad Name: {}", my_heptad.name);
    println!("Core Attribute: {}", Heptad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Insight: {}", my_heptad.insight);
    println!("Research: {}", my_heptad.research);
    println!("Design: {}", my_heptad.design);
    println!("Synthesis: {}", my_heptad.synthesis);
    println!("Application: {}", my_heptad.application);
    println!("Delivery: {}", my_heptad.delivery);
    println!("Value: {}", my_heptad.value);
    println!("---------------------");
}

fn create_octad() {
    println!("\n--- Creating an Octad ---");
    let name_str: String = get_input!("Enter a name for your Octad: ", "Failed to read Octad name", "Unnamed Octad");
    let smallest_significant_holon_str: String = get_input!("Enter the Octad's smallest significant holon: ", "Failed to read smallest significant holon", "Default Smallest Significant Holon");
    let critical_functions_str: String = get_input!("Enter the Octad's critical functions: ", "Failed to read critical functions", "Default Critical Functions");
    let supportive_platform_str: String = get_input!("Enter the Octad's supportive platform: ", "Failed to read supportive platform", "Default Supportive Platform");
    let necessary_resourcing_str: String = get_input!("Enter the Octad's necessary resourcing: ", "Failed to read necessary resourcing", "Default Necessary Resourcing");
    let integrative_totality_str: String = get_input!("Enter the Octad's integrative totality: ", "Failed to read integrative totality", "Default Integrative Totality");
    let inherent_values_str: String = get_input!("Enter the Octad's inherent values: ", "Failed to read inherent values", "Default Inherent Values");
    let intrinsic_nature_str: String = get_input!("Enter the Octad's intrinsic nature: ", "Failed to read intrinsic nature", "Default Intrinsic Nature");
    let organisational_modes_str: String = get_input!("Enter the Octad's organisational modes: ", "Failed to read organisational modes", "Default Organisational Modes");

    let my_octad = Octad::new(
        &name_str,
        &smallest_significant_holon_str,
        &critical_functions_str,
        &supportive_platform_str,
        &necessary_resourcing_str,
        &integrative_totality_str,
        &inherent_values_str,
        &intrinsic_nature_str,
        &organisational_modes_str,
    );

    println!("\n--- Octad Details ---");
    println!("Octad Name: {}", my_octad.name);
    println!("Core Attribute: {}", Octad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Smallest Significant Holon: {}", my_octad.smallest_significant_holon);
    println!("Critical Functions: {}", my_octad.critical_functions);
    println!("Supportive Platform: {}", my_octad.supportive_platform);
    println!("Necessary Resourcing: {}", my_octad.necessary_resourcing);
    println!("Integrative Totality: {}", my_octad.integrative_totality);
    println!("Inherent Values: {}", my_octad.inherent_values);
    println!("Intrinsic Nature: {}", my_octad.intrinsic_nature);
    println!("Organisational Modes: {}", my_octad.organisational_modes);
    println!("---------------------");
}

fn create_dodecad() {
    println!("\n--- Creating a Dodecad ---");
    let mut name_input = String::new();
    print!("Enter a name for your Dodecad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).expect("Failed to read Dodecad name");
    let name = name_input.trim();

    let autocracy: String = get_input!("Enter Dodecad's autocracy: ", "Failed to read autocracy", "Default Autocracy");
    let domination: String = get_input!("Enter Dodecad's domination: ", "Failed to read domination", "Default Domination");
    let creativity: String = get_input!("Enter Dodecad's creativity: ", "Failed to read creativity", "Default Creativity");
    let pattern: String = get_input!("Enter Dodecad's pattern: ", "Failed to read pattern", "Default Pattern");
    let individuality: String = get_input!("Enter Dodecad's individuality: ", "Failed to read individuality", "Default Individuality");
    let structure: String = get_input!("Enter Dodecad's structure: ", "Failed to read structure", "Default Structure");
    let repetition: String = get_input!("Enter Dodecad's repetition: ", "Failed to read repetition", "Default Repetition");
    let potentiality: String = get_input!("Enter Dodecad's potentiality: ", "Failed to read potentiality", "Default Potentiality");
    let subsistence: String = get_input!("Enter Dodecad's subsistence: ", "Failed to read subsistence", "Default Subsistence");
    let relatedness: String = get_input!("Enter Dodecad's relatedness: ", "Failed to read relatedness", "Default Relatedness");
    let polarity: String = get_input!("Enter Dodecad's polarity: ", "Failed to read polarity", "Default Polarity");
    let wholeness: String = get_input!("Enter Dodecad's wholeness: ", "Failed to read wholeness", "Default Wholeness");

    let my_dodecad = Dodecad::new(
        if name.is_empty() { "Unnamed Dodecad" } else { name },
        &autocracy,
        &domination,
        &creativity,
        &pattern,
        &individuality,
        &structure,
        &repetition,
        &potentiality,
        &subsistence,
        &relatedness,
        &polarity,
        &wholeness,
    );

    println!("\n--- Dodecad Details ---");
    println!("Dodecad Name: {}", my_dodecad.name);
    println!("Core Attribute: {}", Dodecad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Autocracy: {}", my_dodecad.autocracy);
    println!("Domination: {}", my_dodecad.domination);
    println!("Creativity: {}", my_dodecad.creativity);
    println!("Pattern: {}", my_dodecad.pattern);
    println!("Individuality: {}", my_dodecad.individuality);
    println!("Structure: {}", my_dodecad.structure);
    println!("Repetition: {}", my_dodecad.repetition);
    println!("Potentiality: {}", my_dodecad.potentiality);
    println!("Subsistence: {}", my_dodecad.subsistence);
    println!("Relatedness: {}", my_dodecad.relatedness);
    println!("Polarity: {}", my_dodecad.polarity);
    println!("Wholeness: {}", my_dodecad.wholeness);
    println!("---------------------");
}
