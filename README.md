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
- **Comprehensive Testing**: 51 tests passing with excellent coverage across all modules

### ✅ **Six-Component Schema Architecture**
Each systematic structure now implements a comprehensive six-component schema:
1. **System Name**: Canonical name (Monad, Dyad, Triad, etc.)
2. **Coherence Attribute**: Internal consistency principle (e.g., Universality, Complimentarity, Dynamism)
3. **Term Designation**: What individual elements are called (e.g., Totality, Poles, Impulses)
4. **Term Characteristics**: Canonical terms from Bennett's frameworks
5. **First-Order Connectives Names**: Categorical names for relationships (e.g., Connectionless unity, Force, Acts)
6. **Actual Connectives**: Specific relationship mappings between terms

## Project Structure

```
SysteMaster/
├── api/                                 # 🏗️ Core library with type-safe API ✅ COMPLETE
│   ├── src/
│   │   ├── lib.rs                       # Main API entry point and documentation
│   │   ├── structures/
│   │   │   ├── monad.rs                 # Monad implementation ✅
│   │   │   ├── dyad.rs                  # Dyad implementation ✅
│   │   │   ├── triad.rs                 # Triad implementation ✅
│   │   │   ├── tetrad.rs                # Tetrad implementation ✅
│   │   │   ├── pentad.rs                # Pentad implementation ✅
│   │   │   ├── hexad.rs                 # Hexad implementation ✅
│   │   │   ├── heptad.rs                # Heptad implementation ✅
│   │   │   ├── octad.rs                 # Octad implementation ✅
│   │   │   ├── dodecad.rs               # Dodecad implementation ✅
│   │   │   └── mod.rs                   # Structure exports
│   │   ├── schemas/                     # 🔄 Modular schema architecture ✅
│   │   │   ├── mod.rs                   # Core schema traits and re-exports
│   │   │   ├── canonical/               # Bennett's canonical schemas
│   │   │   │   ├── mod.rs               # Schema module exports
│   │   │   │   ├── monad.rs             # MonadSchema implementation
│   │   │   │   ├── dyad.rs              # DyadSchema implementation
│   │   │   │   ├── triad.rs             # TriadSchema implementation
│   │   │   │   ├── tetrad.rs            # TetradSchema implementation
│   │   │   │   ├── pentad.rs            # PentadSchema implementation
│   │   │   │   ├── hexad.rs             # HexadSchema implementation
│   │   │   │   ├── heptad.rs            # HeptadSchema implementation
│   │   │   │   ├── octad.rs             # OctadSchema implementation
│   │   │   │   └── dodecad.rs           # DodecadSchema implementation
│   │   │   └── providers/               # Schema provider implementations
│   │   │       └── mod.rs               # BennettSchemas provider
│   │   ├── error.rs                     # Comprehensive error handling ✅
│   │   └── permutations.rs              # Six-fold permutation patterns ✅
│   ├── Cargo.toml                       # API package configuration
│   └── README.md                        # API documentation
├── cli/                                 # Command-line interface application
│   ├── src/
│   │   ├── main.rs                      # CLI entry point
│   │   ├── modules/
│   │   │   ├── structure1_monad.rs     # Monad with attributes system ✅
│   │   │   ├── structure2_dyad.rs      # Dyadic structures ✅
│   │   │   ├── structure3_triad.rs     # Triadic structures ✅
│   │   │   ├── structure4_tetrad.rs    # Tetradic structures ✅
│   │   │   ├── structure5_pentad.rs    # Pentadic structures ✅
│   │   │   ├── structure6_hexad.rs     # Hexadic structures ✅
│   │   │   ├── structure7_heptad.rs    # Heptadic structures ✅
│   │   │   ├── structure8_octad.rs     # Octadic structures ✅
│   │   │   ├── structure12_dodecad.rs  # Dodecadic structures ✅
│   │   │   ├── permutations.rs         # Six permutations generator ✅
│   │   │   └── mod.rs                  # Module declarations
│   │   └── schemas/
│   │       ├── schema1_monad/          # Monad schemas ✅
│   │       ├── schema2_dyad/           # Dyad schemas ✅
│   │       ├── schema3_triad/          # Triad schemas ✅
│   │       ├── schema4_tetrad/         # Tetrad schemas ✅
│   │       ├── schema5_pentad/         # Pentad schemas ✅
│   │       ├── schema6_hexad/          # Hexad schemas ✅
│   │       ├── schema7_heptad/         # Heptad schemas ✅
│   │       ├── schema8_octad/          # Octad schemas ✅
│   │       ├── schema12_dodecad/       # Dodecad schemas ✅
│   │       └── mod.rs                  # Schema trait and exports
│   ├── Cargo.toml                      # CLI package configuration
│   └── README.md                       # CLI documentation
├── frontend/                           # Web interface application
│   ├── Cargo.toml                      # Frontend package configuration
│   └── README.md                       # Frontend documentation
├── Cargo.toml                          # Workspace configuration
└── README.md                           # Project overview
```

## Usage

### CLI Interface
```bash
cargo run --bin systematics
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
cargo test --package systematics-api      # Run all API tests (51 tests)
cargo test --package systematics-cli     # Run all CLI tests  
cargo test structure1_monad              # Run monad-specific tests
cargo test structure2_dyad               # Run dyad-specific tests
cargo test permutations                  # Run permutations-specific tests
cargo test schemas                       # Run schema refactoring tests
# ... similarly for other modules
```

## Development Status

### 🎯 **Recent Major Achievements**
- ✅ **Terminology Refactor**: Renamed `canonical_terms` → `term_characters` throughout codebase for semantic clarity and consistency
- ✅ **Schema Refactoring & Enhancement**: Added coherence attributes, term designations, and first-order connectives names to all schemas and structures
- ✅ **Modular Schema Architecture**: Refactored monolithic `schemas.rs` (332 lines) into clean modular structure
- ✅ **Complete Bidirectional Mapping API**: 8-method bidirectional API across all structures (Tetrad-Dodecad)
- ✅ **Enhanced Monad System**: Unlimited custom attributes with streamlined input flow
- ✅ **Reorganized Codebase**: Numerical organization with clear structure naming
- ✅ **All Tests Passing**: 51/51 tests passing with comprehensive coverage
- ✅ **Schema-Aware Connectives**: Meaningful relationship labels throughout

### 🔧 **Current Capabilities**
1. **Complete CLI Implementation**: All systematic structures (1-12 terms) fully functional
2. **✅ Complete API Library**: Type-safe Rust library with builder pattern and comprehensive error handling
3. **🧩 Enhanced Schema System**: Six-component schema architecture with coherence attributes, term designations, and connectives naming
4. **🔄 Bidirectional Mapping API**: Position ↔ Term navigation across all structures (49 methods total)
5. **Authentic Bennett Schemas**: Proper canonical terminology and relationships with 100+ connectives
6. **Robust Architecture**: Unified schema-based design patterns across CLI and API
7. **Comprehensive Testing**: 51 tests passing across all API structures and schemas
8. **User-Friendly Interface**: Intuitive prompts with canonical defaults

### 📝 **TODO Items**
- [ ] **Graceful Error Handling**: Implement graceful error handling in CLI - when validation errors occur, allow user to correct the error rather than quitting the application
- [ ] **Term Characters → Term Designation Refactor**: Change "term characters" to the relevant "term designation" for each system across the entire codebase so that each system's term characters are accurately referenced by their term designation (e.g., "totalities" for Monad, "poles" for Dyad, "impulses" for Triad, etc.)
- [ ] **Monad Terminology Refactoring**: Change attributes terminology to match term designation - factor out generic "attributes" terminology and replace with "totalities" to align with Monad's specific term designation
- [ ] **Structure Term Designation Consistency**: Structure files use term designations in the display methods println, but reference user_instances in the code. We may need to change the references again, ideally the semantics would be on point in the code "canonical_sources", "user_sources", "user_impulses" etc. but would add a comment mentioning that this is the term designation or add a prefix etc.
- [ ] **CLI Display Format Enhancement**: Improve tetrad and pentad connectives display format. Instead of the current arrow format (`Purpose <--Output--> Higher Potential`), consider displaying canonical connectives as keys with user-defined instances as values:
  ```
  Interplays/Mutualities:
    - Output: Purpose → Higher Potential: Research
    - Range of potential: Higher Potential → Lower Potential: Governance
    - Aspiration: Quintessence → Higher Potential: Strategy
  ```
  This would provide clearer semantic separation between the canonical relationship names (which are the meaningful systematic concepts) and the user's specific instances. The canonical connective names carry the ontological significance, while the user instances are the concrete manifestations. This format would make the systematic structure more readable and emphasize the conceptual framework underlying the user's specific application.
- [ ] **CLI API Integration**: Fix remaining hardcoded canonical terms in CLI functions (tetrad, pentad, hexad, heptad, octad, dodecad) to use API schema access like triad now does
- [ ] **Architectural Discussion - Unbiased API & Schema Modularity**: Design discussion around making the API truly unbiased and composable:
  - Move Bennett's canonical schemas from core API to a provider system
  - Enable loading alternative schema perspectives (Buddhist, other schools of thought)
  - Consider microservices architecture for maximum modularity
  - Design schema provider interface for pluggable systematic frameworks
  - Evaluate separation of concerns between core API and canonical implementations

### 🌐 **Future Development Roadmap**

#### **✅ Phase 1: API Restructuring (COMPLETED)**
1. ✅ **Library/Binary Split**: Core API library separated from CLI application
2. ✅ **Unified Structure API**: Complete `SystematicStructure` trait implementation
3. ✅ **Clean Builder Pattern**: Consistent API across all structure types (monad to dodecad)
4. ✅ **Error Handling**: Proper error types with `thiserror` integration
5. ✅ **Feature Flags**: Modular design ready for optional features
6. ✅ **API Documentation**: Comprehensive rustdoc with examples in lib.rs
7. ✅ **Modular Schema Architecture**: Broke monolithic `schemas.rs` into clean directory structure
8. ✅ **Complete Bidirectional Mapping**: 8-method API for position ↔ term navigation across all structures
9. ✅ **Six-Component Schema Enhancement**: Added coherence attributes, term designations, and first-order connectives names to all schemas and structures
10. 🚧 **Position Descriptions**: Add position descriptions and rename function to indicate if the positions are terms, sources, influences etc.
11. 🚧 **Semantic Alignment**: Align semantic positionality between numbers and terms. Once aligned remove number from CLI output

#### **Phase 2: CLI Modernization**

#### **Phase 2: Core Enhancements**
11. **Missing Structures**: Implement Enneagram (9), Decad (10), Hendecad (11)
12. **Additional Schemas**: Multiple schema options per structure type
13. **Alternative Schemas**: Create alternative schemas beyond Bennett's canonical ones (now easy with modular structure)
14. **Export/Import**: Save and load structures to/from files (JSON/YAML)
15. **Enhanced Validation**: More sophisticated input validation and error handling

#### **Phase 3: Advanced Features**
16. **Database Integration**: Persistent storage for created structures
17. **Relationship Mapping**: Advanced connective relationship analysis
18. **Structure Comparison**: Compare different instances of same structure type
19. **Batch Operations**: Create multiple structures from templates

#### **Phase 4: Web Interface**
20. **Frontend Revival**: Modern web interface using the existing Yew foundation
21. **Interactive Creation**: Browser-based structure creation and editing
22. **Visualization**: Graphical representation of structures and relationships
23. **Collaborative Features**: Multi-user editing and sharing capabilities

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
3. Run `cargo test --package systematics-api` to ensure API works (should see 49 tests pass)
4. Run `cargo test --package systematics-cli` to ensure CLI works
5. Run `cargo run --bin systematics` to test the CLI interface

### Code Quality Standards
- **Modular Schema Design**: All schemas organized in `api/src/schemas/canonical/` directory structure
- **Bennett's Terminology**: Use authentic canonical terms and relationships
- **Bidirectional API Compliance**: New structures must implement full 8-method bidirectional mapping API
- **Comprehensive Testing**: All new features must include thorough test coverage
- **Input Validation**: Robust validation with clear, actionable error messages
- **Documentation**: Clear comments and documentation for all public APIs

### Testing Guidelines
- Unit tests for individual functions and methods
- Integration tests for schema and structure interactions
- Test both successful operations and error conditions
- Maintain current test coverage standards (49+ API tests)
- Test bidirectional mapping methods and aliases
- Test schema-aware connective generation

## Architecture

### Design Principles
1. **Modular Schema Architecture**: Clean separation of canonical schemas, providers, and core traits
2. **Bidirectional Navigation**: Complete position ↔ term mapping across all systematic structures
3. **Authentic Implementation**: Bennett's canonical terms and relationships
4. **Robust Validation**: Comprehensive input validation with clear feedback
5. **Extensible Design**: Easy addition of alternative schemas and providers
6. **User Experience**: Intuitive CLI with helpful prompts and canonical defaults
7. **Comprehensive Testing**: High test coverage ensuring reliability

### Technical Highlights
- **🧩 Enhanced Schema System**: Six-component architecture with coherence attributes, term designations, and connectives naming
- **🔄 Complete Bidirectional Mapping**: 8-method API for position ↔ term navigation (49 total methods)
- **Schema-Aware Connectives**: Meaningful relationship labels (e.g., "Autocracy <> Domination")
- **Enhanced Monad System**: Unlimited custom attributes with streamlined input
- **Extensible Architecture**: Easy addition of alternative schemas and custom providers
- **Numerical Organization**: Clear structure naming (structure1_monad, structure2_dyad, etc.)
- **Clean Rust Architecture**: Leverages Rust's type system for safety and performance
- **Interactive CLI**: User-friendly command-line interface with validation loops
- **Named Permutations**: Semantic labeling of permutation patterns
- **Canonical Defaults**: Press Enter to use authentic Bennett terminology
- **Test-Driven Development**: Comprehensive test suite ensures reliability (51 API tests)

### Schema Architecture
The modular schema system provides:
- **🏗️ Canonical Schema Directory**: Individual files for each systematic structure (`monad.rs` through `dodecad.rs`)
- **🔄 Bidirectional Mapping**: Complete position ↔ term navigation with aliases
- **🏢 Schema Providers**: Extensible provider system for different schema sets
- **⚡ Interactive Selection**: Automatic schema application
- **🔗 Connective Generation**: Schema-aware relationship labels
- **📍 Position Descriptions**: Detailed explanations for each term position
- **🎯 Attribute Descriptions**: Core characteristics of each structure type

**Modular Benefits:**
- **Maintainability**: Each schema in separate file for easy editing
- **Extensibility**: Future schema variants can be added as new directories
- **Clarity**: Individual schema files focus on single concern
- **Collaboration**: Multiple developers can work on different schemas simultaneously
- **Versioning**: Individual schema changes tracked separately