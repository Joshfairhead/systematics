use systematics_api::{SystematicsApi, SystematicStructure};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Monad Port from CLI ===\n");
    
    let api = SystematicsApi::new();
    
    // Test 1: Basic Monad Creation (like CLI)
    println!("Test 1: Basic Monad Creation");
    let monad1 = api.create_monad()
        .name("Test Monad")
        .term("Unity")
        .build()?;
    
    println!("✓ Created monad: {}", monad1.name());
    println!("✓ Term: {}", monad1.term());
    println!("✓ ID: {}", monad1.id());
    println!("✓ System: {}", monad1.system().name());
    
    // Test 2: Attributes System (CLI feature)
    println!("\nTest 2: Attributes System");
    let monad2 = api.create_monad()
        .name("Attributed Monad")
        .term("Absolute")
        .attribute("infinite")
        .attribute("eternal")
        .attribute("unchanging")
        .build()?;
    
    println!("✓ Monad with {} attributes", monad2.attributes().len());
    for (i, attr) in monad2.attributes().iter().enumerate() {
        println!("  {}. {}", i + 1, attr);
    }
    
    // Test 3: Attribute Management (CLI methods)
    println!("\nTest 3: Attribute Management");
    let mut monad3 = api.create_monad()
        .name("Dynamic Monad")
        .term("Source")
        .build()?;
    
    monad3.add_attribute("boundless".to_string());
    monad3.add_attribute("perfect".to_string());
    println!("✓ Added attributes dynamically: {} total", monad3.attributes().len());
    
    assert!(monad3.has_attribute("boundless"));
    println!("✓ Found attribute 'boundless'");
    
    monad3.remove_attribute("boundless");
    assert!(!monad3.has_attribute("boundless"));
    println!("✓ Removed attribute 'boundless'");
    
    // Test 4: Validation (CLI validation rules)
    println!("\nTest 4: Validation Rules");
    
    // Test empty term validation
    let invalid_result = api.create_monad()
        .name("Invalid Monad")
        .term("")
        .build();
    assert!(invalid_result.is_err());
    println!("✓ Empty term validation works");
    
    // Test empty name validation
    let invalid_result2 = api.create_monad()
        .name("")
        .term("Unity")
        .build();
    assert!(invalid_result2.is_err());
    println!("✓ Empty name validation works");
    
    // Test long term validation (>100 chars)
    let long_term = "a".repeat(101);
    let invalid_result3 = api.create_monad()
        .name("Test")
        .term(&long_term)
        .build();
    assert!(invalid_result3.is_err());
    println!("✓ Long term validation works");
    
    // Test invalid characters
    let invalid_result4 = api.create_monad()
        .name("Test")  
        .term("Unity@#$%^&*")
        .build();
    assert!(invalid_result4.is_err());
    println!("✓ Invalid character validation works");
    
    // Test 5: Schema Integration (CLI schema system)
    println!("\nTest 5: Schema Integration");
    let monad4 = api.create_monad()
        .name("Schema Test")
        .term("Unity")
        .build()?;
    
    let term_characters = monad4.term_characters();
    println!("✓ Term characters: {:?}", term_characters);
    assert_eq!(term_characters, vec!["Unity"]);
    
    let instances = monad4.user_terms();
    println!("✓ Current instances: {:?}", instances);
    assert_eq!(instances, vec!["Unity"]);
    
    // Test 6: Display Method (CLI display functionality)
    println!("\nTest 6: Display Method");
    let display_monad = api.create_monad()
        .name("Display Test")
        .term("Absolute Unity")
        .attribute("infinite")
        .attribute("eternal")
        .build()?;
    
    display_monad.display();
    
    // Test 7: SystematicStructure Trait (API integration)
    println!("\nTest 7: SystematicStructure Trait");
    println!("✓ Term count: {}", display_monad.user_terms().len());
    println!("✓ Terms: {:?}", display_monad.user_terms());
    assert_eq!(display_monad.user_terms().len(), 1);
    
    match display_monad.validate() {
        Ok(_) => println!("✓ Structure validation passed"),
        Err(e) => {
            println!("✗ Validation failed: {}", e);
            return Err(e.into());
        }
    }
    
    // Test 8: Default Values (CLI behavior)
    println!("\nTest 8: Default Values");
    let default_monad = api.create_monad()
        .term("Unity")
        .build()?;  // No name provided
    
    println!("✓ Default name: {}", default_monad.name());
    assert_eq!(default_monad.name(), "Unnamed Monad");
    
    // Test 9: Bulk Attribute Creation
    println!("\nTest 9: Bulk Attribute Creation");
    let bulk_monad = api.create_monad()
        .name("Bulk Test")
        .term("Unity")
        .attributes(vec!["infinite", "eternal", "unchanging", "perfect"])
        .build()?;
    
    println!("✓ Bulk attributes: {} total", bulk_monad.attributes().len());
    assert_eq!(bulk_monad.attributes().len(), 4);
    
    // Test 10: Error Handling Consistency
    println!("\nTest 10: Error Handling");
    match api.create_monad().term("").build() {
        Err(e) => println!("✓ Consistent error: {}", e),
        Ok(_) => {
            println!("✗ Should have failed!");
            return Err("Validation should have failed".into());
        }
    }
    
    println!("\n=== All Monad Port Tests Passed! ===");
    println!("✅ Basic creation works");
    println!("✅ Attributes system fully functional");
    println!("✅ Validation rules ported correctly");
    println!("✅ Schema integration working");
    println!("✅ Display method available");
    println!("✅ API traits implemented");
    println!("✅ Error handling consistent");
    println!("✅ CLI functionality preserved");
    
    println!("\n🎯 Monad is ready for production use!");
    println!("📋 Ready to replicate this pattern for other structures");
    
    Ok(())
} 