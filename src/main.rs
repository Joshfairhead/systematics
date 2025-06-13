mod modules;
mod schemas; 

use crate::modules::{MonadicStructure, DyadicStructure, TriadicStructure, TetradicStructure, PentadicStructure, HexadicStructure, HeptadicStructure, OctadicStructure, DodecadicStructure};
use crate::modules::permutations;
use std::io; // Import for input/output

fn main() {
    println!("How many terms in your system? (1, 2, 3, 4, 5, 6, 7, 8, 12, or P for permutations)");
    println!("Note: All systems now use the new schema-based implementation");
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
    
    // Handle numeric choices
    match choice.parse::<u32>() {
        Ok(num_terms) => match num_terms {
            1 => {
                match MonadicStructure::create_interactive() {
                    Ok(_monad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating monad: {}", e),
                }
            }
            2 => {
                match DyadicStructure::create_interactive() {
                    Ok(_dyad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating dyad: {}", e),
                }
            }
            3 => {
                match TriadicStructure::create_interactive() {
                    Ok(_triad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating triad: {}", e),
                }
            }
            4 => { 
                match TetradicStructure::create_interactive() {
                    Ok(_tetrad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating tetrad: {}", e),
                }
            }
            5 => {
                match PentadicStructure::create_interactive() {
                    Ok(_pentad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating pentad: {}", e),
                }
            }
            6 => {
                match HexadicStructure::create_interactive() {
                    Ok(_hexad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating hexad: {}", e),
                }
            }
            7 => {
                match HeptadicStructure::create_interactive() {
                    Ok(_heptad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating heptad: {}", e),
                }
            }
            8 => {
                match OctadicStructure::create_interactive() {
                    Ok(_octad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating octad: {}", e),
                }
            }
            12 => {
                match DodecadicStructure::create_interactive() {
                    Ok(_dodecad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating dodecad: {}", e),
                }
            }
            _ => {
                println!("Invalid number of terms. Please choose 1, 2, 3, 4, 5, 6, 7, 8, or 12.");
            }
        },
        Err(_) => {
            println!("Invalid input. Please enter a number (1-12) or 'P' for permutations.");
        }
    }
}


