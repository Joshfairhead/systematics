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

fn main() {
    println!("Create a new entity: Monad (m), Dyad (d), Triad (t), Tetrad (e), Pentad (p), Hexad (h), Heptad (s), Octad (o), or Dodecad (w)?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");

    match choice.trim().to_lowercase().as_str() {
        "m" | "monad" => {
            create_monad();
        }
        "d" | "dyad" => {
            create_dyad();
        }
        "t" | "triad" => {
            create_triad();
        }
        "e" | "tetrad" => {
            create_tetrad();
        }
        "p" | "pentad" => {
            create_pentad();
        }
        "h" | "hexad" => {
            create_hexad();
        }
        "s" | "heptad" => {
            create_heptad();
        }
        "o" | "octad" => {
            create_octad();
        }
        "w" | "dodecad" => {
            create_dodecad();
        }
        _ => {
            println!("Invalid choice. Exiting.");
        }
    }
}

fn create_monad() {
    println!("\n--- Creating a Monad ---");

    let mut monad_name_input = String::new();
    print!("Enter a name for your Monad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut monad_name_input).expect("Failed to read name");
    let monad_name = monad_name_input.trim();

    let mut my_monad = Monad::new(if monad_name.is_empty() { "Unnamed Monad" } else { monad_name });

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

    let mut dyad_name_input = String::new();
    print!("Enter a name for your Dyad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dyad_name_input).expect("Failed to read Dyad name");
    let dyad_name = dyad_name_input.trim();

    let mut essence_input = String::new();
    print!("Enter the Dyad's essence: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut essence_input).expect("Failed to read essence");
    let essence = essence_input.trim();

    let mut existence_input = String::new();
    print!("Enter the Dyad's existence: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut existence_input).expect("Failed to read existence");
    let existence = existence_input.trim();

    let my_dyad = Dyad::new(
        if dyad_name.is_empty() { "Unnamed Dyad" } else { dyad_name },
        if essence.is_empty() { "Default Essence" } else { essence },
        if existence.is_empty() { "Default Existence" } else { existence },
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

    let mut name_input = String::new();
    print!("Enter a name for your Triad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).expect("Failed to read Triad name");
    let name = name_input.trim();

    let mut active_input = String::new();
    print!("Enter the Triad's active aspect: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut active_input).expect("Failed to read active aspect");
    let active = active_input.trim();

    let mut passive_input = String::new();
    print!("Enter the Triad's passive aspect: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut passive_input).expect("Failed to read passive aspect");
    let passive = passive_input.trim();

    let mut reconciling_input = String::new();
    print!("Enter the Triad's reconciling aspect: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut reconciling_input).expect("Failed to read reconciling aspect");
    let reconciling = reconciling_input.trim();

    let my_triad = Triad::new(
        if name.is_empty() { "Unnamed Triad" } else { name },
        if active.is_empty() { "Default Active" } else { active },
        if passive.is_empty() { "Default Passive" } else { passive },
        if reconciling.is_empty() { "Default Reconciling" } else { reconciling },
    );

    println!("\n--- Triad Details ---");
    println!("Triad Name: {}", my_triad.name);
    println!("Core Attribute: {}", Triad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Active Term: {}", my_triad.active);
    println!("Passive Term: {}", my_triad.passive);
    println!("Reconciling Term: {}", my_triad.reconciling);
    println!("---------------------");
}

fn create_tetrad() {
    println!("\n--- Creating a Tetrad ---");
    let mut name_input = String::new();
    print!("Enter a name for your Tetrad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).expect("Failed to read Tetrad name");
    let name = name_input.trim();

    let mut ground_input = String::new();
    print!("Enter the Tetrad's ground aspect: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ground_input).expect("Failed to read ground aspect");
    let ground = ground_input.trim();

    let mut ideal_input = String::new();
    print!("Enter the Tetrad's ideal aspect: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ideal_input).expect("Failed to read ideal aspect");
    let ideal = ideal_input.trim();

    let mut instrumental_input = String::new();
    print!("Enter the Tetrad's instrumental aspect: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut instrumental_input).expect("Failed to read instrumental aspect");
    let instrumental = instrumental_input.trim();

    let mut directive_input = String::new();
    print!("Enter the Tetrad's directive aspect: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut directive_input).expect("Failed to read directive aspect");
    let directive = directive_input.trim();

    let my_tetrad = Tetrad::new(
        if name.is_empty() { "Unnamed Tetrad" } else { name },
        if ground.is_empty() { "Default Ground" } else { ground },
        if ideal.is_empty() { "Default Ideal" } else { ideal },
        if instrumental.is_empty() { "Default Instrumental" } else { instrumental },
        if directive.is_empty() { "Default Directive" } else { directive },
    );

    println!("\n--- Tetrad Details ---");
    println!("Tetrad Name: {}", my_tetrad.name);
    println!("Core Attribute: {}", Tetrad::TERM_ATTRIBUTE_DESCRIPTION);
    println!("Ground Term: {}", my_tetrad.ground);
    println!("Ideal Term: {}", my_tetrad.ideal);
    println!("Instrumental Term: {}", my_tetrad.instrumental);
    println!("Directive Term: {}", my_tetrad.directive);
    println!("---------------------");
}

fn create_pentad() {
    println!("\n--- Creating a Pentad ---");
    let mut name_input = String::new();
    print!("Enter a name for your Pentad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).expect("Failed to read Pentad name");
    let name = name_input.trim();

    let mut intrinsic_input = String::new();
    print!("Enter the Pentad's intrinsic limit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut intrinsic_input).expect("Failed to read intrinsic limit");
    let intrinsiclimit = intrinsic_input.trim();

    let mut innerupper_input = String::new();
    print!("Enter the Pentad's inner upper limit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut innerupper_input).expect("Failed to read inner upper limit");
    let innerupperlimit = innerupper_input.trim();

    let mut innerlower_input = String::new();
    print!("Enter the Pentad's inner lower limit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut innerlower_input).expect("Failed to read inner lower limit");
    let innerlowerlimit = innerlower_input.trim();

    let mut outerupper_input = String::new();
    print!("Enter the Pentad's outer upper limit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut outerupper_input).expect("Failed to read outer upper limit");
    let outerupperlimit = outerupper_input.trim();

    let mut outerlower_input = String::new();
    print!("Enter the Pentad's outer lower limit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut outerlower_input).expect("Failed to read outer lower limit");
    let outerlowerlimit = outerlower_input.trim();

    let my_pentad = Pentad::new(
        if name.is_empty() { "Unnamed Pentad" } else { name },
        if intrinsiclimit.is_empty() { "Default Intrinsic Limit" } else { intrinsiclimit },
        if innerupperlimit.is_empty() { "Default Inner Upper Limit" } else { innerupperlimit },
        if innerlowerlimit.is_empty() { "Default Inner Lower Limit" } else { innerlowerlimit },
        if outerupperlimit.is_empty() { "Default Outer Upper Limit" } else { outerupperlimit },
        if outerlowerlimit.is_empty() { "Default Outer Lower Limit" } else { outerlowerlimit },
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
    let mut name_input = String::new();
    print!("Enter a name for your Hexad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).expect("Failed to read Hexad name");
    let name = name_input.trim();

    let mut resources_input = String::new();
    print!("Enter the Hexad's resources: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut resources_input).expect("Failed to read resources");
    let resources = resources_input.trim();

    let mut values_input = String::new();
    print!("Enter the Hexad's values: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut values_input).expect("Failed to read values");
    let values = values_input.trim();

    let mut options_input = String::new();
    print!("Enter the Hexad's options: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut options_input).expect("Failed to read options");
    let options = options_input.trim();

    let mut criteria_input = String::new();
    print!("Enter the Hexad's criteria: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut criteria_input).expect("Failed to read criteria");
    let criteria = criteria_input.trim();

    let mut facts_input = String::new();
    print!("Enter the Hexad's facts: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut facts_input).expect("Failed to read facts");
    let facts = facts_input.trim();

    let mut priorities_input = String::new();
    print!("Enter the Hexad's priorities: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut priorities_input).expect("Failed to read priorities");
    let priorities = priorities_input.trim();

    let my_hexad = Hexad::new(
        if name.is_empty() { "Unnamed Hexad" } else { name },
        if resources.is_empty() { "Default Resources" } else { resources },
        if values.is_empty() { "Default Values" } else { values },
        if options.is_empty() { "Default Options" } else { options },
        if criteria.is_empty() { "Default Criteria" } else { criteria },
        if facts.is_empty() { "Default Facts" } else { facts },
        if priorities.is_empty() { "Default Priorities" } else { priorities },
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
    let mut name_input = String::new();
    print!("Enter a name for your Heptad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).expect("Failed to read Heptad name");
    let name = name_input.trim();

    let mut insight_input = String::new();
    print!("Enter the Heptad's insight: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut insight_input).expect("Failed to read insight");
    let insight = insight_input.trim();

    let mut research_input = String::new();
    print!("Enter the Heptad's research: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut research_input).expect("Failed to read research");
    let research = research_input.trim();

    let mut design_input = String::new();
    print!("Enter the Heptad's design: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut design_input).expect("Failed to read design");
    let design = design_input.trim();

    let mut synthesis_input = String::new();
    print!("Enter the Heptad's synthesis: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut synthesis_input).expect("Failed to read synthesis");
    let synthesis = synthesis_input.trim();

    let mut application_input = String::new();
    print!("Enter the Heptad's application: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut application_input).expect("Failed to read application");
    let application = application_input.trim();

    let mut delivery_input = String::new();
    print!("Enter the Heptad's delivery: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut delivery_input).expect("Failed to read delivery");
    let delivery = delivery_input.trim();

    let mut value_input = String::new();
    print!("Enter the Heptad's value: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut value_input).expect("Failed to read value");
    let value = value_input.trim();

    let my_heptad = Heptad::new(
        if name.is_empty() { "Unnamed Heptad" } else { name },
        if insight.is_empty() { "Default Insight" } else { insight },
        if research.is_empty() { "Default Research" } else { research },
        if design.is_empty() { "Default Design" } else { design },
        if synthesis.is_empty() { "Default Synthesis" } else { synthesis },
        if application.is_empty() { "Default Application" } else { application },
        if delivery.is_empty() { "Default Delivery" } else { delivery },
        if value.is_empty() { "Default Value" } else { value },
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
    let mut name_input = String::new();
    print!("Enter a name for your Octad: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_input).expect("Failed to read Octad name");
    let name = name_input.trim();

    let mut holon_input = String::new();
    print!("Enter the Octad's smallest significant holon: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut holon_input).expect("Failed to read smallest significant holon");
    let smallest_significant_holon = holon_input.trim();

    let mut functions_input = String::new();
    print!("Enter the Octad's critical functions: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut functions_input).expect("Failed to read critical functions");
    let critical_functions = functions_input.trim();

    let mut platform_input = String::new();
    print!("Enter the Octad's supportive platform: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut platform_input).expect("Failed to read supportive platform");
    let supportive_platform = platform_input.trim();

    let mut resourcing_input = String::new();
    print!("Enter the Octad's necessary resourcing: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut resourcing_input).expect("Failed to read necessary resourcing");
    let necessary_resourcing = resourcing_input.trim();

    let mut totality_input = String::new();
    print!("Enter the Octad's integrative totality: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut totality_input).expect("Failed to read integrative totality");
    let integrative_totality = totality_input.trim();

    let mut values_input = String::new();
    print!("Enter the Octad's inherent values: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut values_input).expect("Failed to read inherent values");
    let inherent_values = values_input.trim();

    let mut nature_input = String::new();
    print!("Enter the Octad's intrinsic nature: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nature_input).expect("Failed to read intrinsic nature");
    let intrinsic_nature = nature_input.trim();

    let mut modes_input = String::new();
    print!("Enter the Octad's organisational modes: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut modes_input).expect("Failed to read organisational modes");
    let organisational_modes = modes_input.trim();

    let my_octad = Octad::new(
        if name.is_empty() { "Unnamed Octad" } else { name },
        if smallest_significant_holon.is_empty() { "Default Smallest Significant Holon" } else { smallest_significant_holon },
        if critical_functions.is_empty() { "Default Critical Functions" } else { critical_functions },
        if supportive_platform.is_empty() { "Default Supportive Platform" } else { supportive_platform },
        if necessary_resourcing.is_empty() { "Default Necessary Resourcing" } else { necessary_resourcing },
        if integrative_totality.is_empty() { "Default Integrative Totality" } else { integrative_totality },
        if inherent_values.is_empty() { "Default Inherent Values" } else { inherent_values },
        if intrinsic_nature.is_empty() { "Default Intrinsic Nature" } else { intrinsic_nature },
        if organisational_modes.is_empty() { "Default Organisational Modes" } else { organisational_modes },
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
