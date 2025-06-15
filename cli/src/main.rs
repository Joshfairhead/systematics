use systematics_api::{SystematicsApi, SystematicStructure};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = SystematicsApi::new();
    
    println!("How many terms in your system? (1, 2, 3, 4, 5, 6, 7, 8, 12, or P for permutations)");
    println!("Note: All systems now use the new modular API architecture");
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read choice");

    let choice = choice_input.trim();
    
    // Handle permutations option
    if choice.to_lowercase() == "p" || choice.to_lowercase() == "permutations" {
        match create_permutations_interactive(&api) {
            Ok(_) => {}, // Successfully created
            Err(e) => eprintln!("Error creating permutations: {}", e),
        }
        return Ok(());
    }
    
    // Handle numeric choices
    match choice.parse::<u32>() {
        Ok(num_terms) => match num_terms {
            1 => {
                match create_monad_interactive(&api) {
                    Ok(_monad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating monad: {}", e),
                }
            }
            2 => {
                match create_dyad_interactive(&api) {
                    Ok(_dyad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating dyad: {}", e),
                }
            }
            3 => {
                match create_triad_interactive(&api) {
                    Ok(_triad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating triad: {}", e),
                }
            }
            4 => { 
                match create_tetrad_interactive(&api) {
                    Ok(_tetrad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating tetrad: {}", e),
                }
            }
            5 => {
                match create_pentad_interactive(&api) {
                    Ok(_pentad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating pentad: {}", e),
                }
            }
            6 => {
                match create_hexad_interactive(&api) {
                    Ok(_hexad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating hexad: {}", e),
                }
            }
            7 => {
                match create_heptad_interactive(&api) {
                    Ok(_heptad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating heptad: {}", e),
                }
            }
            8 => {
                match create_octad_interactive(&api) {
                    Ok(_octad) => {}, // Successfully created
                    Err(e) => eprintln!("Error creating octad: {}", e),
                }
            }
            12 => {
                match create_dodecad_interactive(&api) {
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

// Interactive creation functions using the API
fn create_monad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Monad ---");
    
    let name = get_optional_input("Enter a name for your monad: ", "Monad Structure")?;
    let term = get_optional_input("Enter term (or press Enter for 'Unity'): ", "Unity")?;
    
    let monad = api.create_monad()
        .name(&name)
        .term(&term)
        .build()?;
    
    println!("\n✅ Created Monad:");
    monad.display();
    monad.validate()?;
    
    Ok(())
}

fn create_dyad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Dyad ---");
    
    let name = get_optional_input("Enter a name for your dyad: ", "Dyad Structure")?;
    let canonical = ["Essence", "Existence"];
    
    println!("Canonical terms: {} / {}", canonical[0], canonical[1]);
    let term1 = get_optional_input(&format!("Enter term 1 (or press Enter for '{}'): ", canonical[0]), canonical[0])?;
    let term2 = get_optional_input(&format!("Enter term 2 (or press Enter for '{}'): ", canonical[1]), canonical[1])?;
    
    let dyad = api.create_dyad()
        .name(&name)
        .terms(&term1, &term2)
        .build()?;
    
    println!("\n✅ Created Dyad:");
    dyad.display();
    dyad.validate()?;
    
    Ok(())
}

fn create_triad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Triad ---");
    
    let name = get_optional_input("Enter a name for your triad: ", "Triad Structure")?;
    let canonical = ["Will", "Function", "Being"];
    
    println!("Canonical terms: {} / {} / {}", canonical[0], canonical[1], canonical[2]);
    let term1 = get_optional_input(&format!("Enter term 1 (or press Enter for '{}'): ", canonical[0]), canonical[0])?;
    let term2 = get_optional_input(&format!("Enter term 2 (or press Enter for '{}'): ", canonical[1]), canonical[1])?;
    let term3 = get_optional_input(&format!("Enter term 3 (or press Enter for '{}'): ", canonical[2]), canonical[2])?;
    
    let triad = api.create_triad()
        .name(&name)
        .terms(&term1, &term2, &term3)
        .build()?;
    
    println!("\n✅ Created Triad:");
    triad.display();
    triad.validate()?;
    
    Ok(())
}

fn create_tetrad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Tetrad ---");
    
    let name = get_optional_input("Enter a name for your tetrad: ", "Tetrad Structure")?;
    let canonical = ["Ground", "Ideal", "Instrumental", "Directive"];
    
    println!("Canonical terms: {} / {} / {} / {}", canonical[0], canonical[1], canonical[2], canonical[3]);
    let mut terms = Vec::new();
    
    for (i, &canonical_term) in canonical.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, canonical_term),
            canonical_term
        )?;
        terms.push(term);
    }
    
    let tetrad = api.create_tetrad()
        .name(&name)
        .terms(&terms[0], &terms[1], &terms[2], &terms[3])
        .build()?;
    
    println!("\n✅ Created Tetrad:");
    tetrad.display();
    tetrad.validate()?;
    
    Ok(())
}

fn create_pentad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Pentad ---");
    
    let name = get_optional_input("Enter a name for your pentad: ", "Pentad Structure")?;
    let canonical = ["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"];
    
    println!("Canonical terms: {} / {} / {} / {} / {}", 
        canonical[0], canonical[1], canonical[2], canonical[3], canonical[4]);
    let mut terms = Vec::new();
    
    for (i, &canonical_term) in canonical.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, canonical_term),
            canonical_term
        )?;
        terms.push(term);
    }
    
    let pentad = api.create_pentad()
        .name(&name)
        .terms(&terms[0], &terms[1], &terms[2], &terms[3], &terms[4])
        .build()?;
    
    println!("\n✅ Created Pentad:");
    pentad.display();
    pentad.validate()?;
    
    Ok(())
}

fn create_hexad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Hexad ---");
    
    let name = get_optional_input("Enter a name for your hexad: ", "Hexad Structure")?;
    let canonical = ["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"];
    
    println!("Canonical terms: {} / {} / {} / {} / {} / {}", 
        canonical[0], canonical[1], canonical[2], canonical[3], canonical[4], canonical[5]);
    let mut terms = Vec::new();
    
    for (i, &canonical_term) in canonical.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, canonical_term),
            canonical_term
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
    
    Ok(())
}

fn create_heptad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Heptad ---");
    
    let name = get_optional_input("Enter a name for your heptad: ", "Heptad Structure")?;
    let canonical = ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"];
    
    println!("Canonical terms: {} / {} / {} / {} / {} / {} / {}", 
        canonical[0], canonical[1], canonical[2], canonical[3], canonical[4], canonical[5], canonical[6]);
    let mut terms = Vec::new();
    
    for (i, &canonical_term) in canonical.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, canonical_term),
            canonical_term
        )?;
        terms.push(term);
    }
    
    let heptad = api.create_heptad()
        .name(name)
        .terms(terms[0].clone(), terms[1].clone(), terms[2].clone(), terms[3].clone(), terms[4].clone(), terms[5].clone(), terms[6].clone())
        .build()?;
    
    println!("\n✅ Created Heptad:");
    heptad.display();
    heptad.validate()?;
    
    Ok(())
}

fn create_octad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating an Octad ---");
    
    let name = get_optional_input("Enter a name for your octad: ", "Octad Structure")?;
    let canonical = ["Smallest Significant Holon", "Critical Functions", "Supportive Platform", "Necessary Resourcing", 
                    "Integrative Totality", "Inherent Values", "Intrinsic Nature", "Organisational Modes"];
    
    println!("Canonical terms: {} / {} / {} / {} / {} / {} / {} / {}", 
        canonical[0], canonical[1], canonical[2], canonical[3], canonical[4], canonical[5], canonical[6], canonical[7]);
    let mut terms = Vec::new();
    
    for (i, &canonical_term) in canonical.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, canonical_term),
            canonical_term
        )?;
        terms.push(term);
    }
    
    let octad = api.create_octad()
        .name(name)
        .terms(terms[0].clone(), terms[1].clone(), terms[2].clone(), terms[3].clone(), terms[4].clone(), terms[5].clone(), terms[6].clone(), terms[7].clone())
        .build()?;
    
    println!("\n✅ Created Octad:");
    octad.display();
    octad.validate()?;
    
    Ok(())
}

fn create_dodecad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Dodecad ---");
    
    let name = get_optional_input("Enter a name for your dodecad: ", "Dodecad Structure")?;
    let canonical = ["Autocracy", "Domination", "Creativity", "Pattern", "Individuality", "Structure", 
                    "Repetition", "Potentiality", "Subsistence", "Relatedness", "Polarity", "Wholeness"];
    
    println!("Canonical terms (12): {} / {} / {} / {} / {} / {} / {} / {} / {} / {} / {} / {}", 
        canonical[0], canonical[1], canonical[2], canonical[3], canonical[4], canonical[5], 
        canonical[6], canonical[7], canonical[8], canonical[9], canonical[10], canonical[11]);
    let mut terms = Vec::new();
    
    for (i, &canonical_term) in canonical.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, canonical_term),
            canonical_term
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
        .name(name) 
        .terms(terms_array)
        .build()?;
    
    println!("\n✅ Created Dodecad:");
    dodecad.display();
    dodecad.validate()?;
    
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


