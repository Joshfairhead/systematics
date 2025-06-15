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
    
    // Get canonical terms and term designation from the schema
    use systematics_api::schemas::MonadSchema;
    use systematics_api::schemas::Schema;
    let schema = MonadSchema;
    let term_characters = schema.term_characters();
    let term_designation = schema.term_designation();
    let schema_name = schema.name();
    
    let name = get_optional_input(&format!("Enter name (Press enter for {}): ", schema_name), schema_name)?;
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
    
    Ok(())
}

fn create_dyad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Dyad ---");
    
    let name = get_optional_input("Enter name (Press enter for Dyad): ", "Dyad")?;
    
    // Get canonical terms and term designation from the schema
    use systematics_api::schemas::DyadSchema;
    use systematics_api::schemas::Schema;
    let schema = DyadSchema;
    let term_characters = schema.term_characters();
    let term_designation = schema.term_designation();
    
    println!("Term {}: {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1]);
    let term1 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[0], term_designation.trim_end_matches('s'), term_characters[0]), term_characters[0])?;
    let term2 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[1], term_designation.trim_end_matches('s'), term_characters[1]), term_characters[1])?;
    
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
    
    let name = get_optional_input("Enter name (Press enter for Triad): ", "Triad")?;
    
    // Get canonical terms and term designation from the schema
    use systematics_api::schemas::TriadSchema;
    use systematics_api::schemas::Schema;
    let schema = TriadSchema;
    let term_characters = schema.term_characters();
    let term_designation = schema.term_designation();
    
    println!("Term {}: {} / {} / {}", 
        term_designation.to_lowercase(), 
        term_characters[0], term_characters[1], term_characters[2]);
    let term1 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[0], term_designation.trim_end_matches('s'), term_characters[0]), term_characters[0])?;
    let term2 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[1], term_designation.trim_end_matches('s'), term_characters[1]), term_characters[1])?;
    let term3 = get_optional_input(&format!("Enter {} {} (Press enter for '{}'): ", term_characters[2], term_designation.trim_end_matches('s'), term_characters[2]), term_characters[2])?;
    
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
    let characters = ["Ground", "Ideal", "Instrumental", "Directive"];
    
    println!("Term characters: {} / {} / {} / {}", characters[0], characters[1], characters[2], characters[3]);
    let mut terms = Vec::new();
    
    for (i, &term_character) in characters.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, term_character),
            term_character
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
    let characters = ["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"];
    
    println!("Term characters: {} / {} / {} / {} / {}", 
        characters[0], characters[1], characters[2], characters[3], characters[4]);
    let mut terms = Vec::new();
    
        for (i, &term_character) in characters.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, term_character),
            term_character
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
    let characters = ["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"];
    
    println!("Term characters: {} / {} / {} / {} / {} / {}", 
        characters[0], characters[1], characters[2], characters[3], characters[4], characters[5]);
    let mut terms = Vec::new();
    
        for (i, &term_character) in characters.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, term_character),
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
    
    Ok(())
}

fn create_heptad_interactive(api: &SystematicsApi) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating a Heptad ---");
    
    let name = get_optional_input("Enter a name for your heptad: ", "Heptad Structure")?;
    let characters = ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"];
    
    println!("Term characters: {} / {} / {} / {} / {} / {} / {}", 
        characters[0], characters[1], characters[2], characters[3], characters[4], characters[5], characters[6]);
    let mut terms = Vec::new();
    
        for (i, &term_character) in characters.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, term_character),
            term_character
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
    let characters = ["Smallest Significant Holon", "Critical Functions", "Supportive Platform", "Necessary Resourcing", 
                    "Integrative Totality", "Inherent Values", "Intrinsic Nature", "Organisational Modes"];
    
    println!("Term characters: {} / {} / {} / {} / {} / {} / {} / {}", 
        characters[0], characters[1], characters[2], characters[3], characters[4], characters[5], characters[6], characters[7]);
    let mut terms = Vec::new();
    
        for (i, &term_character) in characters.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, term_character),
            term_character
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
    let characters = ["Autocracy", "Domination", "Creativity", "Pattern", "Individuality", "Structure", 
                    "Repetition", "Potentiality", "Subsistence", "Relatedness", "Polarity", "Wholeness"];
    
    println!("Term characters (12): {} / {} / {} / {} / {} / {} / {} / {} / {} / {} / {} / {}", 
        characters[0], characters[1], characters[2], characters[3], characters[4], characters[5], 
        characters[6], characters[7], characters[8], characters[9], characters[10], characters[11]);
    let mut terms = Vec::new();
    
    for (i, &term_character) in characters.iter().enumerate() {
        let term = get_optional_input(
            &format!("Enter term {} (or press Enter for '{}'): ", i + 1, term_character),
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


