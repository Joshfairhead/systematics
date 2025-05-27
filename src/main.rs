mod monad;
use crate::monad::Monad;
use std::io::{self, Write}; // Import for input/output

fn main() {
    println!("Let's create a Monad!");

    // Get Monad name from user
    let mut monad_name_input = String::new();
    print!("Enter a name for your Monad: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input
    io::stdin().read_line(&mut monad_name_input).expect("Failed to read name");
    let monad_name = monad_name_input.trim();

    let mut my_monad = Monad::new(if monad_name.is_empty() { "Unnamed Monad" } else { monad_name });

    println!("\nEnter terms for \"{}\". Press Enter on an empty line when done.", my_monad.name);
    loop {
        let mut term_input = String::new();
        print!("Term: ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed
        io::stdin().read_line(&mut term_input).expect("Failed to read term");

        let trimmed_term = term_input.trim();

        if trimmed_term.is_empty() {
            break; // Exit loop if user enters an empty line
        }
        my_monad.add_term(trimmed_term);
    }

    println!("\n--- Monad Details ---");
    println!("Monad Name: {}", my_monad.name);
    println!("Term Attribute: {}", Monad::TERM_ATTRIBUTE_DESCRIPTION);

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

    // If you want to see the raw Vec<String> for debugging:
    // println!("Raw terms vector: {:?}", my_monad.terms);

}
