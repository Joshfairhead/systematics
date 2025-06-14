use systematics_api::{SystematicsApi, SystematicStructure};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the API
    let api = SystematicsApi::new();
    
    // =========================================================================
    // Create a monad
    // =========================================================================
    let monad = api.create_monad()
        .name("My First Monad")
        .term("Unity")
        .attribute("infinite")
        .attribute("eternal")
        .attribute("unchanging")
        .build()?;
    
    // Display the monad in a clean format
    println!("✓ Created: {}", monad.name());
    println!("  Term: {}", monad.term());
    
    if !monad.attributes().is_empty() {
        println!("  Attributes: {}", monad.attributes().join(", "));
    }
    
    println!("  Schema: {}", monad.schema().name());
    println!("  ID: {}", &monad.id()[..8]); // Show only first 8 chars of UUID
    
    // Validate the structure
    match monad.validate() {
        Ok(_) => println!("✓ Structure is valid"),
        Err(e) => println!("✗ Validation failed: {}", e),
    }
    
    // =========================================================================
    // Create a dyad
    // =========================================================================
    println!("\n{}", "=".repeat(50));
    
    let dyad = api.create_dyad()
        .name("My First Dyad")
        .terms("Spirit", "Matter")
        .attribute("fundamental")
        .attribute("complementary")
        .build()?;
    
    // Display the dyad
    println!("✓ Created: {}", dyad.name());
    println!("  Terms: {} ↔ {}", dyad.first_term(), dyad.second_term());
    
    if let Some(connective) = dyad.connective() {
        println!("  Relationship: {} {} {}", dyad.first_term(), connective, dyad.second_term());
    }
    
    if !dyad.attributes().is_empty() {
        println!("  Attributes: {}", dyad.attributes().join(", "));
    }
    
    println!("  Schema: {}", dyad.schema().name());
    println!("  ID: {}", &dyad.id()[..8]);
    
    match dyad.validate() {
        Ok(_) => println!("✓ Structure is valid"),
        Err(e) => println!("✗ Validation failed: {}", e),
    }
    
    // =========================================================================
    // Create a triad
    // =========================================================================
    println!("\n{}", "=".repeat(50));
    
    let triad = api.create_triad()
        .name("My First Triad".to_string())
        .terms("Will".to_string(), "Function".to_string(), "Being".to_string())
        .attributes(vec!["dynamic".to_string(), "triadic".to_string()])
        .build()?;
    
    // Display the triad
    println!("✓ Created: {}", triad.name());
    println!("  Terms: {} → {} → {}", triad.first_term(), triad.second_term(), triad.third_term());
    
    // Show relationships
    if let Some(rel1) = triad.get_connective(0, 1) {
        println!("  Relationship: {} {} {}", triad.first_term(), rel1, triad.second_term());
    }
    if let Some(rel2) = triad.get_connective(1, 2) {
        println!("               {} {} {}", triad.second_term(), rel2, triad.third_term());
    }
    if let Some(rel3) = triad.get_connective(2, 0) {
        println!("               {} {} {}", triad.third_term(), rel3, triad.first_term());
    }
    
    if !triad.attributes().is_empty() {
        println!("  Attributes: {}", triad.attributes().join(", "));
    }
    
    println!("  Schema: {}", triad.schema().name());
    println!("  ID: {}", &triad.id()[..8]);
    
    match triad.validate() {
        Ok(_) => println!("✓ Structure is valid"),
        Err(e) => println!("✗ Validation failed: {}", e),
    }
    
    // =========================================================================
    // Create a tetrad
    // =========================================================================
    println!("\n{}", "=".repeat(50));
    
    let tetrad = api.create_tetrad()
        .name("My First Tetrad".to_string())
        .terms("Foundation".to_string(), "Vision".to_string(), "Method".to_string(), "Guidance".to_string())
        .attributes(vec!["foundational".to_string(), "systematic".to_string()])
        .build()?;
    
    // Display the tetrad
    println!("✓ Created: {}", tetrad.name());
    println!("  Terms: {} → {} → {} → {}", 
             tetrad.first_term(), tetrad.second_term(), tetrad.third_term(), tetrad.fourth_term());
    
    if !tetrad.attributes().is_empty() {
        println!("  Attributes: {}", tetrad.attributes().join(", "));
    }
    
    println!("  Schema: {}", tetrad.schema().name());
    println!("  ID: {}", &tetrad.id()[..8]);
    
    match tetrad.validate() {
        Ok(_) => println!("✓ Structure is valid"),
        Err(e) => println!("✗ Validation failed: {}", e),
    }
    
    // =========================================================================
    // Show full display methods
    // =========================================================================
    println!("\nFull display methods:");
    monad.display();
    dyad.display();
    triad.display();
    tetrad.display();
    
    // =========================================================================
    // Create permutations
    // =========================================================================
    let terms = ["Will", "Function", "Being"];
    let permutation_set = api.permutations(terms);
    
    println!("\nPermutations of [{}, {}, {}]:", terms[0], terms[1], terms[2]);
    for perm in permutation_set.permutations() {
        let ordered = perm.ordered_terms();
        println!("  {}: {} → {} → {}", 
            perm.name, ordered[0], ordered[1], ordered[2]);
    }
    
    Ok(())
} 