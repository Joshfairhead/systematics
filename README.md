<!-- Testing local Git integration -->

# Systematics

A Rust-based application for creating, managing, and analyzing systematic structures. This project provides both CLI and web interfaces for working with various systematic frameworks from monad (1 term) to dodecad (12 terms), plus a standalone six permutations generator.

## Overview

Systematics allows you to:
- Create user-defined instances from JGB's core ontological grammars
- Define terms and relationships from semantic templates
- Generate six permutations with named patterns (Expansion, Interaction, Order, Concentration, Identity, Freedom)
- Visualize structures through a web interface
- Work with comprehensive systematic frameworks using Bennett's authentic terms and relationships

## Current Features
- **CLI Interface**: Main menu for selecting systematic structures (1-12 terms) or permutations generator
- **Six Permutations Generator**: Standalone tool for generating named permutation patterns from three terms
- **Comprehensive Testing**: 111 tests passing with excellent coverage across all modules
- **Input Validation**: Robust handling of user input with clear error messages
- **Yew Frontend**: Web interface with system overlay visualizations

### ✅ Fully Implemented
- **Monad (1 term)**: Complete with dynamic term collection and Bennett's "Unity in diversity and diversity in unity"
- **Dyad (2 terms)**: Complete with Essence/Existence terms and complementarity concepts
- **Triad (3 terms)**: Complete with Active/Passive/Reconciling terms and canonical defaults
- **Tetrad (4 terms)**: Complete implementation with canonical terms and creation patterns
- **Pentad (5 terms)**: Complete implementation with Bennett's authentic terms
- **Hexad (6 terms)**: Complete implementation with full systematic structure
- **Heptad (7 terms)**: Complete implementation with comprehensive functionality
- **Octad (8 terms)**: Complete implementation with all systematic relationships
- **Permutations**: Six named permutation patterns for any three terms

### 🔄 Partially Implemented
- **Dodecad**: Basic structure, needs completion
- **Frontend Integration**: System overlays exist but not fully connected to backend logic

## Project Structure

```
systematics/
├── src/
│   ├── main.rs                 # CLI entry point with permutations option
│   └── modules/
│       ├── monad.rs           # Complete implementation ✅
│       ├── dyad.rs            # Complete implementation ✅
│       ├── triad.rs           # Complete implementation ✅
│       ├── tetrad.rs          # Complete implementation ✅
│       ├── pentad.rs          # Complete implementation ✅
│       ├── hexad.rs           # Complete implementation ✅
│       ├── heptad.rs          # Complete implementation ✅
│       ├── octad.rs           # Complete implementation ✅
│       ├── dodecad.rs         # Needs completion 🔄
│       ├── permutations.rs    # Complete implementation ✅
│       └── mod.rs             # Module declarations
├── rust_yew_frontend/         # Web interface
│   └── src/
│       ├── components/        # Yew components for visualization
│       └── lib.rs            # Frontend entry point
├── Cargo.toml                # Project configuration
└── Cargo.lock               # Dependency lock file
```

## Usage

### CLI Interface
```bash
cargo run
```

Select the number of terms in your system (1, 2, 3, 4, 5, 6, 7, 8, 12) or enter 'P' for the permutations generator.

### Six Permutations Generator
Choose 'P' from the main menu to access the permutations generator:
- Enter three terms (initiating, colouring, outcome)
- View all six named permutation patterns:
  1. **Expansion** (123): term_1 → term_2 → term_3
  2. **Interaction** (132): term_1 → term_3 → term_2
  3. **Concentration** (213): term_2 → term_1 → term_3
  4. **Identity** (231): term_2 → term_3 → term_1
  5. **Order** (312): term_3 → term_1 → term_2
  6. **Freedom** (321): term_3 → term_2 → term_1

### Testing
```bash
cargo test                    # Run all tests (111 tests)
cargo test monad             # Run monad-specific tests
cargo test dyad              # Run dyad-specific tests
cargo test triad             # Run triad-specific tests
cargo test permutations      # Run permutations-specific tests
# ... similarly for other modules
```

## Development Status

### 🎯 Recent Achievements
- ✅ **Major Implementation Progress**: Monad through Octad are now fully implemented
- ✅ **Six Permutations Generator**: Complete standalone feature with named patterns
- ✅ **Comprehensive Testing**: 111 tests passing across all modules
- ✅ **Bennett's Authentic Terms**: Proper canonical terminology and relationships
- ✅ **Robust Input Validation**: Consistent across all implemented structures
- ✅ **Clean Architecture**: Modular design with clear separation of concerns
- ✅ **Canonical Defaults**: Default to canonical terms when user presses Enter

### 🔧 Immediate Next Steps
1. **Complete Dodecad**: Add `create_interactive()` method to match other modules
2. **Frontend Integration**: Connect CLI functionality to Yew web interface
3. **Enhanced Documentation**: Add detailed documentation for Bennett's canonical terms

### 🌐 Backend Development Roadmap
4. **Database integration**: Add persistence for created structures
5. **Relationship system**: Implement advanced linking between terms
6. **Graph database storage**: Store complex relationships and hierarchies
7. **Import/Export functionality**: Save/load structures from files
8. **API development**: REST API for external integrations

### 🎨 Frontend Development Roadmap
9. **Interactive web creation**: Port CLI creation flow to browser
10. **Real-time visualization**: Live editing and preview of structures
11. **Enhanced UI/UX**: Modern interface with intuitive controls
12. **Collaborative features**: Multi-user editing capabilities

### 🧪 Quality & Performance
13. **Integration tests**: Test database + structures together
14. **Property-based tests**: Add `proptest` for validation logic
15. **Performance optimization**: Benchmark with large structures
16. **Documentation**: Comprehensive API and usage documentation

## Systematic Structures

### Implementation Status
- **Monad** (1 term): **Complete** ✅
- **Dyad** (2 terms): **Complete** ✅
- **Triad** (3 terms): **Complete** ✅
- **Tetrad** (4 terms): **Complete** ✅
- **Pentad** (5 terms): **Complete** ✅
- **Hexad** (6 terms): **Complete** ✅
- **Heptad** (7 terms): **Complete** ✅
- **Octad** (8 terms): **Complete** ✅
- **Enneagram** (9 terms): Not yet implemented
- **Decad** (10 terms): Not yet implemented
- **Hendecad** (11 terms): Not yet implemented
- **Dodecad** (12 terms): Needs completion 🔄
- **Six Permutations**: **Complete** ✅

## Contributing

### Development Setup
1. Install Rust: https://rustup.rs/
2. Clone the repository
3. Run `cargo test` to ensure everything works (should see 111 tests pass)
4. For frontend development: `cd rust_yew_frontend && trunk serve`

### Code Quality Standards
- **Comprehensive Testing**: All new features must include thorough test coverage
- **Bennett's Terminology**: Use authentic canonical terms and relationships
- **Input Validation**: Loop-based validation until valid input is provided
- **Error Handling**: User-friendly, actionable error messages
- **Documentation**: Clear comments and documentation for all public APIs

### Testing Guidelines
- Unit tests for individual functions and methods
- Integration tests for component interactions
- Property-based tests for validation logic
- Test both successful operations and error conditions
- Maintain current test coverage standards (111+ tests)

## Architecture

### Design Principles
1. **Authentic Implementation**: Use Bennett's canonical terms and relationships
2. **Robust Validation**: Comprehensive input validation with clear feedback
3. **Modular Design**: Clear separation between different systematic structures
4. **User Experience**: Intuitive CLI with helpful prompts and canonical defaults
5. **Comprehensive Testing**: High test coverage with multiple test types
6. **Incremental Development**: Build complexity progressively from simpler structures

### Technical Highlights
- **Clean Rust Architecture**: Leverages Rust's type system for safety
- **Interactive CLI**: User-friendly command-line interface with validation loops
- **Named Permutations**: Semantic labeling of permutation patterns (Expansion, Interaction, etc.)
- **Canonical Defaults**: Press Enter to use authentic Bennett terminology
- **Yew Frontend**: Modern web interface using Rust throughout the stack
- **Test-Driven Development**: Comprehensive test suite ensures reliability
- **Canonical Accuracy**: Faithful implementation of Bennett's systematic frameworks