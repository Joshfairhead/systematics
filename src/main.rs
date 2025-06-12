mod modules;
mod schemas; 

use crate::modules::systematic_structures::{Monad, TetradicStructure};
use crate::modules::dyad::Dyad;
use crate::modules::triad::Triad;
use crate::modules::tetrad::Tetrad;
use crate::modules::pentad::Pentad;
use crate::modules::hexad::Hexad;
use crate::modules::heptad::Heptad;
use crate::modules::octad::Octad;
use crate::modules::dodecad::Dodecad;
use crate::modules::permutations;
use crate::schemas::BennettTetradSchema;
use std::io; // Import for input/output

fn demo_schema_system() {
    println!("\n=== Schema System Demo ===");
    
    // Demo Bennett's Tetrad Schema
    println!("\nBennett's Tetrad Schema:");
    let mut bennett_tetrad = TetradicStructure::new_with_positions(
        "Bennett Example",
        "Earth",
        "Heaven", 
        "Tools",
        "Purpose"
    );
    bennett_tetrad.apply_schema(Box::new(BennettTetradSchema));
    bennett_tetrad.display();
    bennett_tetrad.display_connectives();
    
    println!("\n=== Demo Complete ===");
    println!("This demonstrates the new architecture with:");
    println!("- Positional structures (TetradicStructure)");
    println!("- Dynamic schema overlays (Bennett's Tetrad)");
    println!("- Schema-aware connective labeling");
    println!("- Separation of structure from semantics");
}

fn main() {
    println!("How many terms in your system? (1, 2, 3, 4, 4s, 4d, 5, 6, 7, 8, 12, or P for permutations)");
    println!("Note: Use '4s' for the new schema-based Tetrad system");
    println!("Note: Use '4d' for a demo of the schema system");
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read choice");

    let choice = choice_input.trim();
    
    // Handle permutations option
    if choice.to_lowercase() == "p" || choice.to_lowercase() == "permutations" {
        match permutations::create_interactive() {
            Ok(_) => {}, // Successfully created
            Err(e) => eprintln!("Error creating permutations: {}", e),
        }
        return;
    }
    
    // Handle schema demo option
    if choice.to_lowercase() == "4d" {
        demo_schema_system();
        return;
    }
    
    // Handle schema-based tetrad option
    if choice.to_lowercase() == "4s" {
        match TetradicStructure::create_interactive() {
            Ok(_tetrad) => {}, // Successfully created
            Err(e) => eprintln!("Error creating schema-based tetrad: {}", e),
        }
        return;
    }
    
    // Handle numeric choices
    match choice.parse::<u32>() {
        Ok(num_terms) => match num_terms {
            1 => {
                match Monad::create_interactive() {
                    Ok(_monad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating monad: {}", e),
                }
            }
            2 => {
                match Dyad::create_interactive() {
                    Ok(_dyad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating dyad: {}", e),
                }
            }
            3 => {
                match Triad::create_interactive() {
                    Ok(_triad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating triad: {}", e),
                }
            }
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
            12 => {
                match Dodecad::create_interactive() {
                    Ok(_dodecad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating dodecad: {}", e),
                }
            }
            _ => println!("Invalid number of terms. Please enter 1, 2, 3, 4, 5, 6, 7, 8, 12, or P for permutations."),
        },
        Err(_) => {
            println!("Invalid input. Please enter a number (1, 2, 3, 4, 5, 6, 7, 8, 12) or P for permutations.");
        }
    }
}


