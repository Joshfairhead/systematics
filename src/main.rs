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
            5 => {
                match Pentad::create_interactive() {
                    Ok(_pentad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating pentad: {}", e),
                }
            }
            6 => {
                match Hexad::create_interactive() {
                    Ok(_hexad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating hexad: {}", e),
                }
            }
            7 => {
                match Heptad::create_interactive() {
                    Ok(_heptad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating heptad: {}", e),
                }
            }
            8 => {
                match Octad::create_interactive() {
                    Ok(_octad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating octad: {}", e),
                }
            }
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
