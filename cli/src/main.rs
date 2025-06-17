use systematics_api::{SystematicsApi, SystematicStructure};
use std::io::{self, Write};
use clap::{Parser, Subcommand};
use tokio;

mod storage;
use storage::{StorageArgs, StorageCli};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "systematics")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new systematic structure
    Create {
        /// Number of terms (1-12)
        #[arg(short, long)]
        terms: Option<u32>,
    },
    /// Manage storage (database operations)
    Storage(StorageArgs),
    /// Create permutations
    Permutations,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(command) => {
            // Command-line argument mode
            match &command {
                Commands::Create { terms } => {
                    let api = SystematicsApi::new();
                    
                    let num_terms = match terms {
                        Some(n) => *n,
                        None => {
                            println!("How many terms in your system? (1, 2, 3, 4, 5, 6, 7, 8, 12)");
                            println!("Note: All systems now use the new modular API architecture");
                            let mut choice_input = String::new();
                            io::stdin().read_line(&mut choice_input).expect("Failed to read choice");
                            choice_input.trim().parse::<u32>().unwrap_or(0)
                        }
                    };
                    
                    // Initialize storage for command-line mode
                    let storage_cli = match StorageCli::new().await {
                        Ok(cli) => Some(cli),
                        Err(e) => {
                            eprintln!("⚠️  Warning: Could not initialize storage: {}", e);
                            eprintln!("   Structures will not be auto-saved.");
                            None
                        }
                    };
                    
                    handle_structure_creation(num_terms, &api, storage_cli.as_ref()).await?;
                }
                Commands::Storage(args) => {
                    let storage_cli = StorageCli::new().await?;
                    storage_cli.handle_command(args).await?;
                }
                Commands::Permutations => {
                    let api = SystematicsApi::new();
                    match create_permutations_interactive(&api) {
                        Ok(_) => {}, // Successfully created
                        Err(e) => eprintln!("Error creating permutations: {}", e),
                    }
                }
            }
        }
        None => {
            // Interactive menu mode (default when no arguments provided)
            run_interactive_menu().await?;
        }
    }
    
    Ok(())
}

async fn run_interactive_menu() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 SysteMaster");
    println!("=============");
    println!();
    
    // Create a single shared storage instance for the entire session
    let shared_storage = match StorageCli::new().await {
        Ok(storage) => {
            println!("📚 Database connected");
            Some(storage)
        }
        Err(e) => {
            println!("⚠️  Database unavailable: {}", e);
            println!("   Structures will not be saved between operations");
            None
        }
    };
    println!();
    
    loop {
        println!("Options:");
        println!("1. Create structure    4. Permutations");
        println!("2. View saved         5. Exit");
        println!("3. Search");
        println!();
        print!("Choice (1-5): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        
        match choice {
            "1" => {
                println!();
                println!("Terms: 1=Monad 2=Dyad 3=Triad 4=Tetrad 5=Pentad 6=Hexad 7=Heptad 8=Octad 12=Dodecad");
                print!("Number of terms: ");
                io::stdout().flush()?;
                
                let mut terms_input = String::new();
                io::stdin().read_line(&mut terms_input)?;
                let num_terms = terms_input.trim().parse::<u32>().unwrap_or(0);
                
                let api = SystematicsApi::new();
                if let Err(e) = handle_structure_creation(num_terms, &api, shared_storage.as_ref()).await {
                    eprintln!("❌ {}", e);
                }
                println!();
            }
            "2" => {
                println!();
                match &shared_storage {
                    Some(storage_cli) => {
                        let args = StorageArgs {
                            command: storage::StorageCommand::List,
                        };
                        if let Err(e) = storage_cli.handle_command(&args).await {
                            eprintln!("❌ {}", e);
                        }
                    }
                    None => {
                        eprintln!("❌ Database not available");
                    }
                }
                println!();
            }
            "3" => {
                println!();
                print!("Search term: ");
                io::stdout().flush()?;
                
                let mut search_input = String::new();
                io::stdin().read_line(&mut search_input)?;
                let search_term = search_input.trim().to_string();
                
                if !search_term.is_empty() {
                    match &shared_storage {
                        Some(storage_cli) => {
                            let args = StorageArgs {
                                command: storage::StorageCommand::Search { query: search_term },
                            };
                            if let Err(e) = storage_cli.handle_command(&args).await {
                                eprintln!("❌ {}", e);
                            }
                        }
                        None => {
                            eprintln!("❌ Database not available");
                        }
                    }
                } else {
                    println!("❌ Enter a search term");
                }
                println!();
            }
            "4" => {
                println!();
                let api = SystematicsApi::new();
                if let Err(e) = create_permutations_interactive(&api) {
                    eprintln!("❌ {}", e);
                }
                println!();
            }
            "5" | "q" | "quit" | "exit" => {
                println!("👋 Goodbye!");
                return Ok(());
            }
            "" => {
                // Just pressed enter, show menu again
                continue;
            }
            _ => {
                println!("❌ Enter 1-5");
                println!();
            }
        }
    }
}

async fn handle_structure_creation(num_terms: u32, api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    match num_terms {
        1 => {
            match create_monad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating monad: {}", e),
            }
        }
        2 => {
            match create_dyad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating dyad: {}", e),
            }
        }
        3 => {
            match create_triad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating triad: {}", e),
            }
        }
        4 => { 
            match create_tetrad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating tetrad: {}", e),
            }
        }
        5 => {
            match create_pentad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating pentad: {}", e),
            }
        }
        6 => {
            match create_hexad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating hexad: {}", e),
            }
        }
        7 => {
            match create_heptad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating heptad: {}", e),
            }
        }
        8 => {
            match create_octad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating octad: {}", e),
            }
        }
        12 => {
            match create_dodecad_interactive(&api, storage_cli).await {
                Ok(_) => {}, // Successfully created
                Err(e) => eprintln!("Error creating dodecad: {}", e),
            }
        }
        _ => {
            println!("❌ Invalid number of terms. Please choose 1, 2, 3, 4, 5, 6, 7, 8, or 12.");
        }
    }
    
    Ok(())
}

// Helper function for input with default values
fn get_optional_input(prompt: &str, default: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

// Helper function for yes/no input
fn get_yes_no_input(prompt: &str, default: bool) -> Result<bool, Box<dyn std::error::Error>> {
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim().to_lowercase();
        
        if trimmed.is_empty() {
            return Ok(default);
        }
        
        match trimmed.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("Please enter 'y' or 'n' (or press Enter for default)"),
        }
    }
}

fn get_forced_yes_no_input(prompt: &str) -> Result<bool, Box<dyn std::error::Error>> {
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim().to_lowercase();
        
        match trimmed.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}

// Helper function for collecting connectives using term names from schema
fn collect_connectives_with_schema(structure_name: &str, term_characters: &[&str], user_terms: &[String]) -> Result<Vec<((usize, usize), String)>, Box<dyn std::error::Error>> {
    let add_connectives = get_yes_no_input(&format!("Add connectives to {}? (y/n, default n): ", structure_name), false)?;
    
    if !add_connectives {
        return Ok(Vec::new());
    }
    
    let mut connectives = Vec::new();
    
    // For each unique pair of terms, ask for the bidirectional relationship
    for i in 0..term_characters.len() {
        for j in (i + 1)..term_characters.len() {
            print!("{} <--> {} relationship (Enter to skip): ", term_characters[i], term_characters[j]);
            std::io::stdout().flush()?;
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let trimmed = input.trim();
            
            if !trimmed.is_empty() {
                // Add both directions for bidirectional relationship
                connectives.push(((i, j), trimmed.to_string()));
                connectives.push(((j, i), trimmed.to_string()));
                println!("  Added: {} <--{}--> {}", user_terms[i], trimmed, user_terms[j]);
            }
        }
    }
    
    Ok(connectives)
}

// Interactive creation functions using the API
async fn create_monad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Monad ---");
    
    // Get term characters and term designation from the schema
    use systematics_library::MonadicSystem;
    use systematics_library::System;
    let system = MonadicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    let system_name = system.name();
    
    let name = get_optional_input(&format!("Enter name (Press enter for {}): ", system_name), system_name)?;
    let term = get_optional_input(&format!("Enter {} (Press enter to use {}): ", term_designation, term_characters[0]), term_characters[0])?;
    
    // Collect attributes
    let mut attributes = Vec::new();
    println!("Enter attributes (press Enter when done):");
    loop {
        print!("Attribute (or Enter to finish): ");
        std::io::stdout().flush()?;
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            break;
        }
        attributes.push(trimmed.to_string());
    }
    
    let mut builder = api.create_monad()
        .name(&name)
        .term(&term);
    
    for attr in attributes {
        builder = builder.attribute(attr);
    }
    
    let monad = builder.build()?;
    
    println!("\n✅ Created Monad:");
    monad.display();
    monad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&monad, &name, None).await?;
    }
    
    Ok(())
}

async fn create_dyad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Dyad ---");
    
    let name = get_optional_input("Enter name (Press enter for Dyad): ", "Dyad")?;
    
    // Get term characters and term designation from the schema
    use systematics_library::DyadicSystem;
    use systematics_library::System;
    let system = DyadicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    
    println!("Term {}: {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1]);
    let term1 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[0], term_designation.trim_end_matches('s'), term_characters[0]), term_characters[0])?;
    let term2 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[1], term_designation.trim_end_matches('s'), term_characters[1]), term_characters[1])?;
    
    let mut dyad = api.create_dyad()
        .name(&name)
        .terms(&term1, &term2)
        .build()?;
    
    // Collect connectives
    println!();
    let user_terms = vec![term1, term2];
    let connectives = collect_connectives_with_schema(&name, term_characters, &user_terms)?;
    
    // Add connectives to the structure
    for ((from, to), relationship) in connectives {
        dyad.set_connective(from, to, relationship);
    }
    
    println!("\n✅ Created Dyad:");
    dyad.display();
    dyad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&dyad, &name, None).await?;
    }
    
    Ok(())
}

async fn create_triad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Triad ---");
    
    let name = get_optional_input("Enter name (Press enter for Triad): ", "Triad")?;
    
    // Get term characters and term designation from the schema
    use systematics_library::TriadicSystem;
    use systematics_library::System;
    let system = TriadicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    
    println!("Term {}: {} / {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1], term_characters[2]);
    let term1 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[0], term_designation.trim_end_matches('s'), term_characters[0]), term_characters[0])?;
    let term2 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[1], term_designation.trim_end_matches('s'), term_characters[1]), term_characters[1])?;
    let term3 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[2], term_designation.trim_end_matches('s'), term_characters[2]), term_characters[2])?;
    
    let mut triad = api.create_triad()
        .name(&name)
        .terms(&term1, &term2, &term3)
        .build()?;
    
    // Collect connectives
    println!();
    let user_terms = vec![term1, term2, term3];
    let connectives = collect_connectives_with_schema(&name, term_characters, &user_terms)?;
    
    // Add connectives to the structure
    for ((from, to), relationship) in connectives {
        triad.set_connective(from, to, relationship);
    }
    
    println!("\n✅ Created Triad:");
    triad.display();
    triad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&triad, &name, None).await?;
    }
    
    Ok(())
}

async fn create_tetrad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Tetrad ---");
    
    // Get term characters and term designation from the schema
    use systematics_library::TetradicSystem;
    use systematics_library::System;
    let system = TetradicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    let system_name = system.name();
    
    let name = get_optional_input(&format!("Enter name (Press enter for {}): ", system_name), system_name)?;
    
    println!("Term {}: {} / {} / {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1], term_characters[2], term_characters[3]);
    
    let mut terms = Vec::new();
    for &term_character in term_characters.iter() {
        let term = get_optional_input(
            &format!("Enter {} {} (Press enter for '{}'): ", term_character, term_designation.trim_end_matches('s'), term_character),
            term_character
        )?;
        terms.push(term);
    }
    
    let mut tetrad = api.create_tetrad()
        .name(&name)
        .terms(&terms[0], &terms[1], &terms[2], &terms[3])
        .build()?;
    
    // Ask if user wants to add connectives
    println!();
    let add_connectives = get_forced_yes_no_input("Add connectives to Tetrad? (y/n): ")?;
    
    if add_connectives {
        // Show each canonical connective and ask if it should be replaced
        let canonical_connectives = system.connectives();
        println!("\nCanonical tetrad connectives (Enter to keep, or type replacement):");
        
        // Calculate column widths for alignment (same as display)
        let max_left_len = canonical_connectives.iter()
            .map(|c| terms[c.from_position].len())
            .max().unwrap_or(0);
        let max_rel_len = canonical_connectives.iter()
            .map(|c| c.relationship.len())
            .max().unwrap_or(0);
        let max_right_len = canonical_connectives.iter()
            .map(|c| terms[c.to_position].len())
            .max().unwrap_or(0);
        
        for connective in canonical_connectives {
            let from_term = &terms[connective.from_position];
            let to_term = &terms[connective.to_position];
            
            print!("{:^left_width$} <---[{:^rel_width$}]---> {:^right_width$}: ", 
                from_term, 
                connective.relationship, 
                to_term,
                left_width = max_left_len,
                rel_width = max_rel_len,
                right_width = max_right_len);
            std::io::stdout().flush()?;
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let replacement = input.trim();
            
            let final_relationship = if replacement.is_empty() {
                connective.relationship.clone()
            } else {
                replacement.to_string()
            };
            
            tetrad.set_connective(connective.from_position, connective.to_position, final_relationship.clone());
            // Also set reverse direction for bidirectional display
            tetrad.set_connective(connective.to_position, connective.from_position, final_relationship);
        }
    }
    
    println!("\n✅ Created Tetrad:");
    tetrad.display();
    tetrad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&tetrad, &name, None).await?;
    }
    
    Ok(())
}

async fn create_pentad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Pentad ---");
    
    // Get term characters and term designation from the schema
    use systematics_library::PentadicSystem;
    use systematics_library::System;
    let system = PentadicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    let system_name = system.name();
    let connectives_name = system.first_order_connectives_name();
    
    let name = get_optional_input(&format!("Enter name (Press enter for {}): ", system_name), system_name)?;
    
    println!("\nTerm {}: {} / {} / {} / {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1], term_characters[2], term_characters[3], term_characters[4]);
    
    let mut terms = Vec::new();
    for &term_character in term_characters.iter() {
        let term = get_optional_input(
            &format!("Enter {} {} (Press enter for '{}'): ", term_character, term_designation.trim_end_matches('s'), term_character),
            term_character
        )?;
        terms.push(term);
    }

    println!();
    let add_connectives = get_forced_yes_no_input(&format!("Add {} (y/n): ", connectives_name))?;
    
    let mut pentad = api.create_pentad()
        .name(&name)
        .terms(&terms[0], &terms[1], &terms[2], &terms[3], &terms[4])
        .build()?;
    
    if add_connectives {
        // Get canonical connectives from schema
        let canonical_connectives = system.connectives();
        
        println!("\nCanonical {} (Enter to keep, or type replacement):", connectives_name);
        
        // Calculate column widths for alignment (same as display)
        let max_left_len = canonical_connectives.iter()
            .map(|c| terms[c.from_position].len())
            .max().unwrap_or(0);
        let max_rel_len = canonical_connectives.iter()
            .map(|c| c.relationship.len())
            .max().unwrap_or(0);
        let max_right_len = canonical_connectives.iter()
            .map(|c| terms[c.to_position].len())
            .max().unwrap_or(0);
        
        for connective in canonical_connectives {
            let from_term = &terms[connective.from_position];
            let to_term = &terms[connective.to_position];
            
            print!("{:^left_width$} <---[{:^rel_width$}]---> {:^right_width$}: ", 
                from_term, 
                connective.relationship, 
                to_term,
                left_width = max_left_len,
                rel_width = max_rel_len,
                right_width = max_right_len);
            std::io::stdout().flush()?;
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let replacement = input.trim();
            
            let final_relationship = if replacement.is_empty() {
                connective.relationship.clone()
            } else {
                replacement.to_string()
            };
            
            pentad.set_connective(connective.from_position, connective.to_position, final_relationship);
        }
    }
    
    println!("\n✅ Created Pentad:");
    pentad.display();
    pentad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&pentad, &name, None).await?;
    }
    
    Ok(())
}

async fn create_hexad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Hexad ---");
    
    // Get term characters and term designation from the schema
    use systematics_library::HexadicSystem;
    use systematics_library::System;
    let system = HexadicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    let system_name = system.name();
    
    let name = get_optional_input(&format!("Enter name (Press enter for {}): ", system_name), system_name)?;
    
    println!("\nTerm {}: {} / {} / {} / {} / {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1], term_characters[2], term_characters[3], term_characters[4], term_characters[5]);
    
    let mut terms = Vec::new();
    for &term_character in term_characters.iter() {
        let term = get_optional_input(
            &format!("Enter {} {} (Press enter for '{}'): ", term_character, term_designation.trim_end_matches('s'), term_character),
            term_character
        )?;
        terms.push(term);
    }

    let hexad = api.create_hexad()
        .name(&name)
        .terms(&terms[0], &terms[1], &terms[2], &terms[3], &terms[4], &terms[5])
        .build()?;
    
    println!("\n✅ Created Hexad:");
    hexad.display();
    hexad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&hexad, &name, None).await?;
    }
    
    Ok(())
}

async fn create_heptad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Heptad ---");
    
    // Get term characters and term designation from the schema
    use systematics_library::HeptadicSystem;
    use systematics_library::System;
    let system = HeptadicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    let system_name = system.name();
    
    let name = get_optional_input(&format!("Enter name (Press enter for {}): ", system_name), system_name)?;
    
    println!("Term {}: {} / {} / {} / {} / {} / {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1], term_characters[2], term_characters[3], 
        term_characters[4], term_characters[5], term_characters[6]);
    
    let mut terms = Vec::new();
    for &term_character in term_characters.iter() {
        let term = get_optional_input(
            &format!("Enter {} {} (Press enter for '{}'): ", term_character, term_designation.trim_end_matches('s'), term_character),
            term_character
        )?;
        terms.push(term);
    }

    let heptad = api.create_heptad()
        .name(name.clone())
        .terms(terms[0].clone(), terms[1].clone(), terms[2].clone(), terms[3].clone(), terms[4].clone(), terms[5].clone(), terms[6].clone())
        .build()?;
    
    println!("\n✅ Created Heptad:");
    heptad.display();
    heptad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&heptad, &name, None).await?;
    }
    
    Ok(())
}

async fn create_octad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating an Octad ---");
    
    // Get term characters and term designation from the schema
    use systematics_library::OctadicSystem;
    use systematics_library::System;
    let system = OctadicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    let system_name = system.name();
    
    let name = get_optional_input(&format!("Enter name (Press enter for {}): ", system_name), system_name)?;
    
    println!("Term {}: {} / {} / {} / {} / {} / {} / {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1], term_characters[2], term_characters[3], 
        term_characters[4], term_characters[5], term_characters[6], term_characters[7]);
    
    let mut terms = Vec::new();
    for &term_character in term_characters.iter() {
        let term = get_optional_input(
            &format!("Enter {} {} (Press enter for '{}'): ", term_character, term_designation.trim_end_matches('s'), term_character),
            term_character
        )?;
        terms.push(term);
    }

    let octad = api.create_octad()
        .name(name.clone())
        .terms(terms[0].clone(), terms[1].clone(), terms[2].clone(), terms[3].clone(), terms[4].clone(), terms[5].clone(), terms[6].clone(), terms[7].clone())
        .build()?;
    
    println!("\n✅ Created Octad:");
    octad.display();
    octad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&octad, &name, None).await?;
    }
    
    Ok(())
}

async fn create_dodecad_interactive(api: &SystematicsApi, storage_cli: Option<&StorageCli>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Dodecad ---");
    
    // Get term characters and term designation from the schema
    use systematics_library::DodecadicSystem;
    use systematics_library::System;
    let system = DodecadicSystem;
    let term_characters = system.term_characters();
    let term_designation = system.term_designation();
    let system_name = system.name();
    
    let name = get_optional_input(&format!("Enter name (Press enter for {}): ", system_name), system_name)?;
    
    println!("Term {}: {} / {} / {} / {} / {} / {} / {} / {} / {} / {} / {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1], term_characters[2], term_characters[3], 
        term_characters[4], term_characters[5], term_characters[6], term_characters[7],
        term_characters[8], term_characters[9], term_characters[10], term_characters[11]);
    
    let mut terms = Vec::new();
    for &term_character in term_characters.iter() {
        let term = get_optional_input(
            &format!("Enter {} {} (Press enter for '{}'): ", term_character, term_designation.trim_end_matches('s'), term_character),
            term_character
        )?;
        terms.push(term);
    }

    // Convert Vec<String> to [String; 12] array
    let terms_array: [String; 12] = [
        terms[0].clone(), terms[1].clone(), terms[2].clone(), terms[3].clone(),
        terms[4].clone(), terms[5].clone(), terms[6].clone(), terms[7].clone(),
        terms[8].clone(), terms[9].clone(), terms[10].clone(), terms[11].clone()
    ];

    let dodecad = api.create_dodecad()
        .name(name.clone())
        .terms(terms_array)
        .build()?;
    
    println!("\n✅ Created Dodecad:");
    dodecad.display();
    dodecad.validate()?;
    
    if let Some(storage_cli) = storage_cli {
        let _ = storage_cli.auto_save_structure(&dodecad, &name, None).await?;
    }
    
    Ok(())
}

fn create_permutations_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating Six Permutations ---");
    
    let term1 = get_optional_input("Enter initiating term: ", "Term1")?;
    let term2 = get_optional_input("Enter colouring term: ", "Term2")?;
    let term3 = get_optional_input("Enter outcome term: ", "Term3")?;
    
    let permutations = api.permutations([term1.clone(), term2.clone(), term3.clone()]);
    
    println!("\n✅ Six Permutations for [{}, {}, {}]:", term1, term2, term3);
    
    for (i, perm) in permutations.permutations().iter().enumerate() {
        let ordered = perm.ordered_terms();
        println!("{}. {}: {} → {} → {}", 
            i + 1,
            perm.name,
            ordered[0],
            ordered[1], 
            ordered[2]
        );
    }
    
    Ok(())
}




