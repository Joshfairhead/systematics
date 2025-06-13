use systematics_api::{SystematicsApi, SystematicStructure};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the API
    let api = SystematicsApi::new();
    
    // Create a monad using the builder pattern
    let monad = api.builder()
        .monad()
        .name("My First Monad")
        .term("Unity")
        .attribute("infinite")
        .attribute("eternal")
        .attribute("unchanging")
        .build()?;
    
    // Display the monad
    println!("Created monad: {}", monad.name());
    println!("  ID: {}", monad.id());
    println!("  Term: {}", monad.terms()[0]);
    println!("  Attributes: {:?}", monad.attributes());
    println!("  Schema: {}", monad.schema().name());
    
    // Validate the structure
    match monad.validate() {
        Ok(_) => println!("✓ Monad is valid"),
        Err(e) => println!("✗ Monad validation failed: {}", e),
    }
    
    // Create permutations
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