mod modules; 

use crate::modules::monad::Monad;
use crate::modules::dyad::Dyad;
use crate::modules::triad::Triad;
use crate::modules::tetrad::Tetrad;
use std::io::{self, Write}; // Import for input/output

fn main() {
    println!("Create a new entity: Monad (m), Dyad (d), Triad (t), or Tetrad (e)?");
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
