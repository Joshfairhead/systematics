# Systematics

A Rust-based application for creating, managing, and analyzing systematic structures. This project provides both CLI and web interfaces for working with various systematic frameworks from monad (1 term) to dodecad (12 terms).

## Overview

Systematics allows you to:
- Create user-defined instances from JGB's core ontological grammars
- Define terms and relationships from semantic templates
- Visualize structures through a web interface
- Work with comprehensive systematic frameworks using Bennett's authentic terms and relationships

## Current Features
- **CLI Interface**: Main menu for selecting systematic structures (1-12 terms)
- **Comprehensive Testing**: 50 tests passing with excellent coverage across all modules
- **Input Validation**: Robust handling of user input with clear error messages
- **Yew Frontend**: Web interface with system overlay visualizations

### 🔄 Partially Implemented
- **Triad & Dyad**: Basic implementation with some functionality
- **Monad**: Minimal scaffolding
- **Dodecad**: Basic structure, needs completion
- **Frontend Integration**: System overlays exist but not fully connected to backend logic

## Project Structure

```
systematics/
├── src/
│   ├── main.rs                 # CLI entry point
│   └── modules/
│       ├── tetrad.rs          # Complete implementation ✅
│       ├── pentad.rs          # Complete implementation ✅
│       ├── hexad.rs           # Complete implementation ✅
│       ├── heptad.rs          # Complete implementation ✅
│       ├── octad.rs           # Complete implementation ✅
│       ├── triad.rs           # Partial implementation 🔄
│       ├── dyad.rs            # Partial implementation 🔄
│       ├── monad.rs           # Basic scaffolding 🔄
│       ├── dodecad.rs         # Basic scaffolding 🔄
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

Select the number of terms in your system (1-12) and follow the interactive prompts.

### Testing
```bash
cargo test                    # Run all tests (50 tests)
cargo test tetrad            # Run tetrad-specific tests
cargo test pentad            # Run pentad-specific tests
# ... similarly for other modules
```

## Development Status

### 🎯 Recent Achievements
- ✅ **Major Implementation Progress**: Tetrad through Octad are now fully implemented
- ✅ **Comprehensive Testing**: 50 tests passing across all modules
- ✅ **Bennett's Authentic Terms**: Proper canonical terminology and relationships
- ✅ **Robust Input Validation**: Consistent across all implemented structures
- ✅ **Clean Architecture**: Modular design with clear separation of concerns

### 🔧 Immediate Next Steps
1. **Complete remaining structures**: Finish Monad, Dyad, Triad, and Dodecad implementations
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
- **Monad** (1 term): Basic scaffolding 🔄
- **Dyad** (2 terms): Partial implementation 🔄
- **Triad** (3 terms): Partial implementation 🔄
- **Tetrad** (4 terms): **Complete** ✅
- **Pentad** (5 terms): **Complete** ✅
- **Hexad** (6 terms): **Complete** ✅
- **Heptad** (7 terms): **Complete** ✅
- **Octad** (8 terms): **Complete** ✅
- **Enneagram** (9 terms): Not yet implemented
- **Decad** (10 terms): Not yet implemented
- **Hendecad** (11 terms): Not yet implemented
- **Dodecad** (12 terms): Basic scaffolding 🔄

## Contributing

### Development Setup
1. Install Rust: https://rustup.rs/
2. Clone the repository
3. Run `cargo test` to ensure everything works (should see 50 tests pass)
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
- Maintain current test coverage standards (50+ tests)

## Architecture

### Design Principles
1. **Authentic Implementation**: Use Bennett's canonical terms and relationships
2. **Robust Validation**: Comprehensive input validation with clear feedback
3. **Modular Design**: Clear separation between different systematic structures
4. **User Experience**: Intuitive CLI with helpful prompts and error messages
5. **Comprehensive Testing**: High test coverage with multiple test types
6. **Incremental Development**: Build complexity progressively from simpler structures

### Technical Highlights
- **Clean Rust Architecture**: Leverages Rust's type system for safety
- **Interactive CLI**: User-friendly command-line interface with validation loops
- **Yew Frontend**: Modern web interface using Rust throughout the stack
- **Test-Driven Development**: Comprehensive test suite ensures reliability
- **Canonical Accuracy**: Faithful implementation of Bennett's systematic frameworks