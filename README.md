# System-master

A Rust-based application for creating, managing, and analyzing systematic structures. This project provides both CLI and web interfaces for working with various systematic frameworks from monad (1 term) to dodecad (12 terms).

## Overview

Systematics allows you to:
- Create user-defined instances of JGBs core ontological grammars.
- Define terms and relationships (connectives) between terms from semantic templates. 
- Store and manage multiple systematic structures.
- Visualize structures through a web interface.

## Current Features

### ✅ Implemented
- **Tetrad Implementation**: Fully featured with canonical connectives
  - Interactive CLI creation with robust error handling
  - Input validation (length, characters, required fields)
  - Default canonical connectives with modification options
  - Comprehensive test suite (16 tests covering validation, edge cases, error handling)
  - Loop-based input until valid data is provided

- **CLI Interface**: Main menu for selecting systematic structures
- **Yew Frontend**: Web interface with system overlay visualizations
- **Input Validation**: Robust handling of user input with clear error messages
- **Testing**: Comprehensive unit tests with good coverage

### 🔄 Partially Implemented
- **Other Systematic Structures**: Scaffolded but not fully implemented (monad through dodecad)
- **Frontend Components**: System overlays exist but not connected to backend logic
- **Relationship System**: Basic structure described in `BASIC_BACKEND_CONTEXT.md`

## Project Structure

```
systematics/
├── src/
│   ├── main.rs                 # CLI entry point
│   └── modules/
│       ├── tetrad.rs          # Complete tetrad implementation
│       ├── triad.rs           # Scaffolded
│       ├── monad.rs           # Scaffolded
│       └── ...                # Other systematic structures
├── rust_yew_frontend/         # Web interface
│   └── src/components/        # Yew components for visualization
├── BASIC_BACKEND_CONTEXT.md   # Backend design documentation
└── Cargo.toml
```

## Usage

### CLI Interface
```bash
cargo run
```

Select the number of terms in your system (1-12) and follow the interactive prompts.

### For Tetrad (4 terms):
- Enter instances for Ground, Ideal, Instrumental, and Directive
- Optionally modify default canonical connectives:
  - Ground↔Ideal: "Motivational imperative"
  - Ground↔Instrumental: "Technical power" 
  - Ground↔Directive: "Material Mastery"
  - Ideal↔Instrumental: "Effectual compatibility"
  - Ideal↔Directive: "Receptive regard"
  - Instrumental↔Directive: "Demonstrable activity"

### Testing
```bash
cargo test                    # Run all tests
cargo test tetrad            # Run tetrad-specific tests
```

## Roadmap

### 🎯 Immediate Next Steps
1. **Extend tetrad pattern** - Apply robust implementation to other systematic structures
2. **Remove dead code warnings** - Address unused methods
3. **Complete structure implementations** - Finish monad through dodecad

### 🔧 Backend Development
4. **Database integration** - Add persistence for created structures
5. **Link/relationship system** - Implement the model from `BASIC_BACKEND_CONTEXT.md`
6. **Graph database storage** - Store relationships between terms
7. **Import/Export functionality** - Save/load structures from files

### 🌐 Frontend Integration
8. **Connect CLI to web interface** - Bridge CLI and Yew frontend
9. **Interactive web creation** - Port CLI creation flow to browser
10. **Visualization improvements** - Better display of connectives and relationships
11. **Real-time updates** - Live editing of structures

### 🧪 Testing & Quality
12. **Integration tests** - Test database + structures together
13. **Property-based tests** - Add `proptest` for validation logic
14. **Frontend tests** - Add Yew component tests
15. **Performance testing** - Benchmark with large structures

### 🚀 Advanced Features
16. **Multi-structure analysis** - Compare and analyze multiple structures
17. **Relationship discovery** - ML suggestions for connections
18. **Export formats** - PDF, SVG, JSON export options
19. **Collaborative editing** - Multiple users working on same structure
20. **API development** - REST API for external integrations

## Contributing

### Development Setup
1. Install Rust: https://rustup.rs/
2. Clone the repository
3. Run `cargo test` to ensure everything works
4. For frontend development: `cd rust_yew_frontend && trunk serve`

### Code Quality
- All new features should include comprehensive tests
- Follow the established pattern from tetrad implementation
- Input validation should loop until valid input is provided
- Error messages should be user-friendly and actionable

### Testing Guidelines
- Unit tests for individual functions
- Integration tests for component interactions
- Property-based tests for validation logic
- Test both happy path and error conditions

## Architecture

### Current Design Principles
1. **Simplicity First** - Minimal abstractions, easy to understand
2. **Explicit Control** - Users define relationships explicitly  
3. **Robust Input Handling** - Loop until valid input, clear error messages
4. **Comprehensive Testing** - High test coverage with multiple test types
5. **Incremental Discovery** - Add relationships as understanding develops

### Systematic Structures Supported
- **Monad** (1 term): Single point systems
- **Dyad** (2 terms): Binary relationships  
- **Triad** (3 terms): Three-way dynamics
- **Tetrad** (4 terms): Four-way systematic relationships ✅ **Complete**
- **Pentad** (5 terms): Five-element systems
- **Hexad** (6 terms): Six-point frameworks
- **Heptad** (7 terms): Seven-element structures
- **Octad** (8 terms): Eight-way systems
- **Dodecad** (12 terms): Twelve-element comprehensive frameworks

## Status

🟢 **Active Development** - The tetrad implementation serves as a robust foundation for extending to all other systematic structures. The project demonstrates solid software engineering practices with comprehensive testing, error handling, and user experience design. 