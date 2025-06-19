# SysteMaster Frontend

A web-based interface for creating and visualizing systematic structures based on J.G. Bennett's work.

## Overview

The SysteMaster Frontend provides a modern web interface for working with systematic structures. Built with Rust and Yew, it offers an intuitive browser-based experience for creating, editing, and visualizing systematic relationships.

## Current Status

🚧 **Under Development** - The frontend is currently in early development phase.

The CLI application is fully functional and serves as the reference implementation for all systematic structure functionality. The frontend will provide the same capabilities through a modern web interface.

## Planned Features

### Interactive Structure Creation
- Visual structure builders for all systematic types (Monad through Dodecad)
- Drag-and-drop interface for positioning and connecting elements
- Real-time validation and schema application
- Guided workflows for beginners

### Visualization Engine
- Dynamic graphical representation of systematic structures
- Interactive relationship mapping and exploration
- Multiple visualization modes (network, hierarchical, circular)
- Export capabilities (SVG, PNG, PDF)

### Enhanced User Experience
- Responsive design for desktop and mobile
- Dark/light theme support
- Keyboard shortcuts and accessibility features
- Collaborative editing capabilities

### Advanced Functionality
- Structure comparison and analysis tools
- Template library with pre-built examples
- Import/export functionality (JSON, YAML)
- Integration with external data sources

## Technology Stack

- **Framework**: [Yew](https://yew.rs/) - Modern Rust framework for web applications
- **Language**: Rust - Memory-safe systems programming
- **Styling**: CSS3 with modern layout techniques
- **Build**: Trunk for WASM compilation and bundling

## Development Setup

```bash
# Install trunk (Yew build tool)
cargo install trunk

# Install wasm target
rustup target add wasm32-unknown-unknown

# Serve development version
trunk serve

# Build for production
trunk build --release
```

## Architecture

### Component Structure
```
frontend/
├── src/
│   ├── main.rs              # Application entry point
│   ├── components/          # Reusable UI components
│   │   ├── structure_builder.rs
│   │   ├── visualization.rs
│   │   └── schema_selector.rs
│   ├── pages/               # Application pages
│   │   ├── home.rs
│   │   ├── create.rs
│   │   └── explore.rs
│   └── services/            # Business logic and API calls
│       ├── structure_service.rs
│       └── schema_service.rs
├── static/                  # Static assets
└── index.html              # HTML template
```

### Shared Logic
The frontend will share core systematic structure logic with the CLI through:
- Common schema definitions
- Shared validation rules
- Unified data models
- Consistent API patterns

## Roadmap

### Phase 1: Foundation (Current)
- [ ] Basic Yew application setup
- [ ] Component architecture design
- [ ] Shared logic extraction from CLI
- [ ] Basic structure creation interface

### Phase 2: Core Features
- [ ] Interactive structure builders
- [ ] Schema integration and validation
- [ ] Basic visualization engine
- [ ] Responsive design implementation

### Phase 3: Advanced Features
- [ ] Collaborative editing
- [ ] Advanced visualizations
- [ ] Template library
- [ ] Export/import functionality

### Phase 4: Polish & Performance
- [ ] Performance optimization
- [ ] Accessibility improvements
- [ ] Mobile experience refinement
- [ ] Advanced user preferences

## Contributing

The frontend development follows the same principles as the CLI:
- Schema-based architecture
- Authentic Bennett terminology
- Comprehensive testing
- Clean, maintainable code

See the main project README for general contribution guidelines.

## Future Integration

The frontend will eventually provide:
- All CLI functionality through a web interface
- Enhanced visualization capabilities not possible in CLI
- Collaborative features for team-based structure development
- Integration with external tools and data sources

For immediate systematic structure work, use the CLI application while the frontend is under development. 



## NOTES
- System overlay is using bennetts terms rather than remaining agnostic and importing them... this needs fixing