# Systematics

A Rust-based application for creating, managing, and analyzing systematic structures using Bennett's ontological frameworks. This project provides a comprehensive CLI interface for working with systematic structures from monad (1 term) to dodecad (12 terms), plus a standalone six permutations generator.

## Overview

Systematics allows you to:
- Create user-defined instances from Bennett's core ontological grammars
- Define terms and relationships using authentic schema-based templates
- Generate six permutations with named patterns (Expansion, Interaction, Order, Concentration, Identity, Freedom)
- Work with comprehensive systematic frameworks using Bennett's canonical terms and relationships
- Manage unlimited custom attributes for monadic structures

## Current Features

### ✅ **Schema-Based Implementation**
All systematic structures now use a unified schema-based architecture:
- **Canonical Terms**: Authentic Bennett terminology for each structure type
- **Schema-Aware Connectives**: Meaningful relationship labels (e.g., "Creativity <> Polarity")
- **Interactive Creation**: Streamlined user experience with schema guidance
- **Consistent Architecture**: Unified patterns across all structure types

### ✅ **Fully Implemented Structures**
- **Monad (1 term)**: Enhanced with unlimited custom attributes and streamlined input flow
- **Dyad (2 terms)**: Essence/Existence with schema-based connectives
- **Triad (3 terms)**: Will/Function/Being with Active/Passive/Reconciling roles
- **Tetrad (4 terms)**: Complete implementation with Bennett's canonical terms
- **Pentad (5 terms)**: Full systematic structure with schema integration
- **Hexad (6 terms)**: Complete implementation with comprehensive functionality
- **Heptad (7 terms)**: Insight through Value with schema-aware connectives
- **Octad (8 terms)**: Smallest Significant Holon through Organisational Modes
- **Dodecad (12 terms)**: Autocracy through Wholeness with 66 schema-based connectives
- **Permutations**: Six named permutation patterns for any three terms

### ✅ **Enhanced User Experience**
- **CLI Interface**: Intuitive main menu for selecting systematic structures (1-12 terms) or permutations
- **Streamlined Input**: Press Enter to use canonical terms, or provide custom instances
- **Attribute Management**: Add unlimited descriptive attributes to monadic structures
- **Schema Selection**: Automatic application of appropriate Bennett schemas
- **Comprehensive Testing**: 44 tests passing with excellent coverage across all modules

## Project Structure

```
systematics/
├── src/
│   ├── main.rs                           # CLI entry point
│   ├── modules/
│   │   ├── structure1_monad.rs          # Monad with attributes system ✅
│   │   ├── structure2_dyad.rs           # Dyadic structures ✅
│   │   ├── structure3_triad.rs          # Triadic structures ✅
│   │   ├── structure4_tetrad.rs         # Tetradic structures ✅
│   │   ├── structure5_pentad.rs         # Pentadic structures ✅
│   │   ├── structure6_hexad.rs          # Hexadic structures ✅
│   │   ├── structure7_heptad.rs         # Heptadic structures ✅
│   │   ├── structure8_octad.rs          # Octadic structures ✅
│   │   ├── structure12_dodecad.rs       # Dodecadic structures ✅
│   │   ├── permutations.rs              # Six permutations generator ✅
│   │   └── mod.rs                       # Module declarations
│   └── schemas/
│       ├── schema1_monad/               # Monad schemas ✅
│       ├── schema2_dyad/                # Dyad schemas ✅
│       ├── schema3_triad/               # Triad schemas ✅
│       ├── schema4_tetrad/              # Tetrad schemas ✅
│       ├── schema5_pentad/              # Pentad schemas ✅
│       ├── schema6_hexad/               # Hexad schemas ✅
│       ├── schema7_heptad/              # Heptad schemas ✅
│       ├── schema8_octad/               # Octad schemas ✅
│       ├── schema12_dodecad/            # Dodecad schemas ✅
│       └── mod.rs                       # Schema trait and exports
├── rust_yew_frontend/                   # Web interface (future development)
├── Cargo.toml                          # Project configuration
└── README.md                           # This file
```

## Usage

### CLI Interface
```bash
cargo run
```

Select the number of terms in your system (1, 2, 3, 4, 5, 6, 7, 8, 12) or enter 'P' for the permutations generator.

### Example: Creating a Monad
```
1. Select "1" for Monad
2. Enter monad name (or press Enter for default)
3. Press Enter to add attributes
4. Enter attributes one by one:
   - "infinite" [Enter]
   - "eternal" [Enter]
   - "unitive" [Enter]
   - [Empty line to finish]
5. View your completed monad with attributes
```

### Example: Creating a Dodecad
```
1. Select "12" for Dodecad
2. Bennett's Dodecad Schema automatically applied
3. Enter custom instances for each canonical term:
   - Autocracy: [your instance]
   - Domination: [your instance]
   - ... (12 terms total)
4. Choose whether to modify default connectives
5. View structure with 66 schema-based connectives
```

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
cargo test                    # Run all tests (44 tests)
cargo test structure1_monad   # Run monad-specific tests
cargo test structure2_dyad    # Run dyad-specific tests
cargo test permutations       # Run permutations-specific tests
# ... similarly for other modules
```

## Development Status

### 🎯 **Recent Major Achievements**
- ✅ **Complete Schema-Based Refactor**: All structures now use unified schema architecture
- ✅ **Enhanced Monad System**: Unlimited custom attributes with streamlined input flow
- ✅ **Reorganized Codebase**: Numerical organization with clear structure naming
- ✅ **Fixed All Tests**: 44/44 tests passing with comprehensive coverage
- ✅ **Schema-Aware Connectives**: Meaningful relationship labels throughout

### 🔧 **Current Capabilities**
1. **Complete CLI Implementation**: All systematic structures (1-12 terms) fully functional
2. **Authentic Bennett Schemas**: Proper canonical terminology and relationships
3. **Robust Architecture**: Unified schema-based design patterns
4. **Comprehensive Testing**: Full test coverage with integration tests
5. **User-Friendly Interface**: Intuitive prompts with canonical defaults

### 🌐 **Future Development Roadmap**

#### **Phase 1: API Restructuring**
1. **Library/Binary Split**: Separate core library from CLI application
2. **Unified Structure API**: Generic `SystematicStructure<N>` with type aliases
3. **Clean Builder Pattern**: Consistent API across all structure types
4. **Error Handling**: Proper error types with `thiserror` integration
5. **Feature Flags**: Optional CLI, serde support, etc.
6. **API Documentation**: Comprehensive rustdoc with examples
7. **Position Descriptions**: Add position descriptions and rename function to indicate if the positions are terms, sources, influences etc.
8. **Semantic Alignment**: Align semantic positionality between numbers and terms. Once aligned remove number from CLI output

#### **Phase 2: Core Enhancements**
7. **Missing Structures**: Implement Enneagram (9), Decad (10), Hendecad (11)
8. **Additional Schemas**: Multiple schema options per structure type
9. **Export/Import**: Save and load structures to/from files (JSON/YAML)
10. **Enhanced Validation**: More sophisticated input validation and error handling

#### **Phase 3: Advanced Features**
11. **Database Integration**: Persistent storage for created structures
12. **Relationship Mapping**: Advanced connective relationship analysis
13. **Structure Comparison**: Compare different instances of same structure type
14. **Batch Operations**: Create multiple structures from templates

#### **Phase 4: Web Interface**
15. **Frontend Revival**: Modern web interface using the existing Yew foundation
16. **Interactive Creation**: Browser-based structure creation and editing
17. **Visualization**: Graphical representation of structures and relationships
18. **Collaborative Features**: Multi-user editing and sharing capabilities

## Systematic Structures

### Implementation Status
- **Monad** (1 term): **Complete** ✅ - Enhanced with attributes system
- **Dyad** (2 terms): **Complete** ✅ - Essence/Existence schema
- **Triad** (3 terms): **Complete** ✅ - Will/Function/Being schema
- **Tetrad** (4 terms): **Complete** ✅ - Full schema integration
- **Pentad** (5 terms): **Complete** ✅ - Bennett's canonical terms
- **Hexad** (6 terms): **Complete** ✅ - Comprehensive functionality
- **Heptad** (7 terms): **Complete** ✅ - Insight through Value
- **Octad** (8 terms): **Complete** ✅ - Holon through Modes
- **Enneagram** (9 terms): **Not Implemented** ⏳
- **Decad** (10 terms): **Not Implemented** ⏳
- **Hendecad** (11 terms): **Not Implemented** ⏳
- **Dodecad** (12 terms): **Complete** ✅ - Autocracy through Wholeness
- **Six Permutations**: **Complete** ✅ - Named pattern generator

## Contributing

### Development Setup
1. Install Rust: https://rustup.rs/
2. Clone the repository
3. Run `cargo test` to ensure everything works (should see 44 tests pass)
4. Run `cargo run` to test the CLI interface

### Code Quality Standards
- **Schema-Based Design**: All new structures must use the unified schema architecture
- **Bennett's Terminology**: Use authentic canonical terms and relationships
- **Comprehensive Testing**: All new features must include thorough test coverage
- **Input Validation**: Robust validation with clear, actionable error messages
- **Documentation**: Clear comments and documentation for all public APIs

### Testing Guidelines
- Unit tests for individual functions and methods
- Integration tests for schema and structure interactions
- Test both successful operations and error conditions
- Maintain current test coverage standards (44+ tests)
- Test schema-aware connective generation

## Architecture

### Design Principles
1. **Schema-Based Architecture**: Unified design using StructureSchema trait
2. **Authentic Implementation**: Bennett's canonical terms and relationships
3. **Robust Validation**: Comprehensive input validation with clear feedback
4. **Modular Design**: Clear separation between structures and schemas
5. **User Experience**: Intuitive CLI with helpful prompts and canonical defaults
6. **Comprehensive Testing**: High test coverage ensuring reliability

### Technical Highlights
- **Unified Schema System**: StructureSchema trait provides consistent interface
- **Schema-Aware Connectives**: Meaningful relationship labels (e.g., "Autocracy <> Domination")
- **Enhanced Monad System**: Unlimited custom attributes with streamlined input
- **Numerical Organization**: Clear structure naming (structure1_monad, structure2_dyad, etc.)
- **Clean Rust Architecture**: Leverages Rust's type system for safety and performance
- **Interactive CLI**: User-friendly command-line interface with validation loops
- **Named Permutations**: Semantic labeling of permutation patterns
- **Canonical Defaults**: Press Enter to use authentic Bennett terminology
- **Test-Driven Development**: Comprehensive test suite ensures reliability

### Schema Architecture
Each systematic structure is supported by:
- **Schema Definition**: Canonical terms and relationships
- **Interactive Selection**: Automatic schema application
- **Connective Generation**: Schema-aware relationship labels
- **Position Descriptions**: Detailed explanations for each term position
- **Attribute Descriptions**: Core characteristics of each structure type