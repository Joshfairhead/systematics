# Systematics API

A clean, type-safe Rust library for creating and managing systematic structures based on Bennett's ontological frameworks.

## Overview

The Systematics API provides a modern, builder-pattern-based interface for working with systematic structures from monad (1 term) to dodecad (12 terms), plus permutation generation. This library separates the core logic from user interfaces, making it suitable for use in CLI applications, web interfaces, and other contexts.

## Features

- **🏗️ Builder Pattern**: Fluent, type-safe construction of systematic structures
- **🎯 Schema-Based**: Authentic Bennett terminology and relationships
- **⚡ Type Safety**: Compile-time validation and comprehensive error handling
- **🔧 Modular**: Clean separation between core library and applications
- **📊 Permutations**: Six-fold permutation patterns for any three terms
- **🧪 Well Tested**: Comprehensive test coverage
- **📦 Feature Flags**: Optional serialization, CLI, and web support

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
systematics-api = { path = "../api" }

# Optional features
systematics-api = { path = "../api", features = ["serde_support"] }
```

### Basic Usage

```rust
use systematics_api::{SystematicsApi, SystematicStructure};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the API
    let api = SystematicsApi::new();
    
    // Create a monad using the builder pattern
    let monad = api.builder()
        .monad()
        .name("My Unity")
        .term("Absolute")
        .attribute("infinite")
        .attribute("eternal")
        .build()?;
    
    // Work with the structure
    println!("Created: {}", monad.name());
    println!("Terms: {:?}", monad.terms());
    println!("Schema: {}", monad.schema().name());
    
    // Validate
    monad.validate()?;
    
    // Generate permutations
    let terms = ["Will", "Function", "Being"];
    let perms = api.permutations(terms);
    
    for perm in perms.permutations() {
        let ordered = perm.ordered_terms();
        println!("{}: {} → {} → {}", 
            perm.name, ordered[0], ordered[1], ordered[2]);
    }
    
    Ok(())
}
```

## Architecture

### Core Traits

- **`SystematicStructure`**: Unified interface for all structures
- **`Schema`**: Defines canonical terms and relationships  
- **`StructureBuilder`**: Generic builder pattern interface

### Error Handling

Comprehensive error types with `thiserror`:

```rust
use systematics_api::{Result, SystematicsError};

match structure.validate() {
    Ok(_) => println!("✓ Valid structure"),
    Err(SystematicsError::StructureValidation { reason }) => {
        eprintln!("Invalid: {}", reason);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Schema System

Each structure type has a corresponding schema with Bennett's canonical terms:

```rust
let schema = monad.schema();
println!("Schema: {}", schema.name());
println!("Terms: {:?}", schema.canonical_terms());
println!("Connectives: {:?}", schema.connectives());
```

## Structure Types

### Currently Implemented

- **Monad (1 term)**: ✅ Complete with attributes
- **Permutations**: ✅ Six-fold patterns

### In Development

- **Dyad (2 terms)**: Essence/Existence
- **Triad (3 terms)**: Will/Function/Being
- **Tetrad through Dodecad**: Higher-order structures

## Advanced Features

### Custom Attributes (Monad)

```rust
let monad = api.builder()
    .monad()
    .term("Unity")
    .attribute("boundless")
    .attributes(vec!["eternal", "perfect"])
    .build()?;
```

### Serialization (Feature Flag)

```rust
// Requires "serde_support" feature
let json = monad.to_json()?;
let restored = Monad::from_json(&json)?;
```

### Validation

```rust
// Automatic validation in builders
let result = api.builder().monad().term("").build();
assert!(result.is_err());

// Manual validation
structure.validate()?;
```

## Development Status

### ✅ Completed
- Core library architecture
- Error handling system
- Schema framework
- Monad implementation
- Permutation system
- Builder patterns
- Basic example

### 🚧 In Progress
- Complete implementations for Dyad through Dodecad
- Enhanced schema relationships
- Additional examples

### 📋 Planned
- Database persistence
- Multiple schema variants
- Web assembly support
- Advanced validation rules

## Contributing

The API is designed for easy extension:

1. **New Structures**: Implement `SystematicStructure` trait
2. **New Schemas**: Implement `Schema` trait  
3. **New Builders**: Implement `StructureBuilder<T>` trait

## Examples

See `examples/` directory for usage patterns:

- `basic_usage.rs`: Core functionality demonstration
- More examples coming soon...

## Integration

### CLI Integration

```rust
// In your CLI application
use systematics_api::SystematicsApi;

let api = SystematicsApi::new();
let structure = create_structure_from_user_input(&api)?;
```

### Web Integration

```rust
// With wasm-bindgen
#[wasm_bindgen]
pub fn create_monad(name: &str, term: &str) -> Result<JsValue, JsValue> {
    let api = SystematicsApi::new();
    let monad = api.builder().monad().name(name).term(term).build()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(serde_wasm_bindgen::to_value(&monad)?)
}
```

## License

Same as parent project. 