# SysteMaster CLI

A command-line interface for creating and working with systematic structures based on J.G. Bennett's work.

## Overview

The SysteMaster CLI provides an interactive interface for creating systematic structures from Monad (1 term) through Dodecad (12 terms), plus a six permutations generator. Each structure uses authentic Bennett schemas with term characterinology and meaningful relationship labels.

## Quick Start

```bash
# Run the CLI
cargo run --bin systematics

# Run tests
cargo test --package systematics-cli
```

## Features

### Systematic Structures
- **Monad** (1 term): Unity with custom attributes
- **Dyad** (2 terms): Being/Will relationships  
- **Triad** (3 terms): Function/Being/Will dynamics
- **Tetrad** (4 terms): Ground/Ideal/Instrumental/Directive framework
- **Pentad** through **Octad**: Extended systematic relationships
- **Dodecad** (12 terms): Complete systematic totality

### Six Permutations Generator
Generate all six named permutation patterns from three input terms:
1. **Expansion** (123): Natural progression
2. **Interaction** (132): Dynamic exchange  
3. **Concentration** (213): Focused development
4. **Identity** (231): Essential recognition
5. **Order** (312): Structured arrangement
6. **Freedom** (321): Liberation pattern

### Schema-Based Architecture
- Authentic Bennett terminology and relationships
- Schema-aware connective generation
- Position descriptions (research placeholders for authentic sources)
- Consistent API across all structure types

## Usage Examples

### Creating a Monad
```
1. Select "1" for Monad
2. Enter name or press Enter for default
3. Add custom attributes:
   - "infinite"
   - "eternal" 
   - "unitive"
4. View completed monad with attributes
```

### Creating a Tetrad
```
1. Select "4" for Tetrad
2. Bennett's schema automatically applied
3. Enter instances for each position:
   - Ground: [your instance]
   - Ideal: [your instance]
   - Instrumental: [your instance]
   - Directive: [your instance]
4. Optionally modify connectives
5. View structure with 6 meaningful relationships
```

## Architecture

### Three-Layer Semantic System
1. **Base Layer**: 1-based numeric positions (1, 2, 3...)
2. **Schema Layer**: Bennett's term characters mapped to positions
3. **Content Layer**: User's specific instances filling the semantic positions

### Builder Pattern API
```rust
// Example usage (for future library development)
let monad = MonadicStructure::new("My Monad")
    .with_schema(Box::new(BennettMonadSchema))
    .with_content("Unity", "Divine Source")
    .with_attribute("infinite");
```

## Testing

The CLI includes comprehensive test coverage:
- 48 total tests across all modules
- Unit tests for individual structures
- Integration tests for schema interactions
- Permutation pattern validation
- Builder pattern functionality

```bash
# Run all tests
cargo test --package systematics-cli

# Run specific module tests
cargo test structure1_monad
cargo test permutations
```

## Development

### Code Organization
- `src/main.rs`: CLI entry point and user interface
- `src/modules/`: Structure implementations (structure1_monad.rs, etc.)
- `src/schemas/`: Bennett schema definitions organized by structure type

### Key Principles
- Schema-based architecture for consistency
- Authentic Bennett terminology where available
- Research placeholders for missing authentic descriptions
- Comprehensive test coverage
- Clean separation between structures and schemas

## Future Development

See the main project README for the complete roadmap, including:
- API restructuring for library use
- Additional authentic Bennett descriptions
- Alternative schema implementations
- Enhanced validation and error handling 