# SysteMaster

**A Rust-based system for creating, managing, and exploring systematic structures using Bennett's ontological frameworks.**

## 🚀 Quick Start

### One-Command Startup
```bash
# From CLI directory - automatically starts API server
cd cli && cargo run
```

### Or from project root
```bash
./start.sh
```

The system automatically starts the API server if it's not running, then launches the interactive CLI interface.

## ✨ Features

### 🔬 **Systematic Structures**
Create and manage structures from Bennett's ontological frameworks:
- **Monad (1 term)**: Unity principles with custom attributes
- **Dyad (2 terms)**: Essence/Existence relationships  
- **Triad (3 terms)**: Will/Function/Being dynamics
- **Tetrad (4 terms)**: Ground/Ideal/Instrumental/Directive
- **Pentad (5 terms)**: Purpose/Higher Potential/Quintessence/Lower Potential/Source
- **Hexad (6 terms)**: Complete six-fold systematic structures
- **Heptad (7 terms)**: Insight through Value frameworks
- **Octad (8 terms)**: Organisational Modes and Smallest Significant Holon
- **Dodecad (12 terms)**: Autocracy through Wholeness (66 connectives)

### 🌐 **Auto-Start Architecture**
- **Zero Configuration**: No manual server setup required
- **Background API Server**: Starts automatically on port 3001
- **Database Ready**: SurrealDB with persistent storage
- **Cross-Platform**: Works on macOS, Linux, and Windows

### 📚 **Data Management**
- **Persistent Storage**: All structures saved automatically
- **Search & Discovery**: Find structures by name, terms, or relationships
- **Graph Relationships**: Explore connections between structures
- **JSON Export**: Export complete database with metadata

### 🎯 **Six Permutations**
Generate named permutation patterns for any three terms:
- **Expansion**: Outward growth patterns
- **Interaction**: Dynamic relationships  
- **Order**: Structural arrangements
- **Concentration**: Focused intensification
- **Identity**: Core recognition patterns
- **Freedom**: Liberation dynamics

## 🖥️ Interactive CLI Menu

```
🔬 SysteMaster
=============

📚 Database connected

Options:
1. Create structure    4. Permutations
2. View saved         5. Exit
3. Search

Choice (1-5): 
```

## 🏗️ Architecture

```
SysteMaster/
├── api/                    # REST API server with SurrealDB
├── cli/                    # Interactive CLI (auto-starts API)
├── frontend/               # Web interface (Yew/WASM)
├── library/                # Bennett's systematic knowledge
├── data/                   # Database storage
└── start.sh               # One-command system launcher
```

### Components
- **API Server**: Axum-based REST API with SurrealDB backend
- **CLI Client**: Interactive interface with auto-start capability
- **Database**: SurrealDB graph database with persistent storage
- **Frontend**: Yew-based web interface (in development)

## 🔧 Advanced Usage

### Direct Commands
```bash
# Create structure directly
cargo run create --terms 3

# Database operations
cargo run storage list              # List all structures
cargo run storage search "term"    # Search structures
cargo run storage stats            # Database statistics
cargo run storage export           # Export to JSON
```

### Environment Variables
```bash
SYSTEMATICS_API_URL=http://localhost:3001    # API server URL
PORT=3001                                    # Server port
```

## 📊 Current Status

### ✅ **Fully Operational**
- **API Server**: Running on port 3001 with SurrealDB
- **CLI Interface**: Auto-start functionality working
- **Database**: structures store and retrievable
- **All Structures**: Complete implementations (1-12 terms)
- **Data Persistence**: Structures saved automatically

### 🔄 **In Development**
- **Web Frontend**: Yew-based interface
- **GraphQL Integration**: Enhanced query capabilities
- **Advanced Search**: Semantic relationship queries

## 🤝 Contributing

This project implements Bennett's systematic philosophy in modern software architecture. Contributions welcome for:

- Enhanced search capabilities
- Frontend development
- Documentation improvements

## 📖 Background

Based on J.G. Bennett's systematic philosophy, this project makes ontological frameworks accessible through modern software tools. Each systematic structure follows Bennett's canonical terms and relationships, providing authentic implementations of his ontological grammars.

## 🎉 Recently Completed

### ✅ Architecture & Auto-Start (Latest)
- ✅ **One-Command Startup**: `cd cli && cargo run` starts entire system
- ✅ **API-Centric Architecture**: CLI uses API endpoints exclusively
- ✅ **Auto-Start Functionality**: Automatic API server startup
- ✅ **Code Quality**: Fixed deprecated features and compiler warnings
- ✅ **Port Standardization**: System runs on port 3001

### ✅ Core System Implementation
- ✅ **Library/Binary Split**: Clean architectural separation
- ✅ **SurrealDB Integration**: Graph database with persistence
- ✅ **REST API Server**: Comprehensive HTTP endpoints
- ✅ **Database Centralization**: Moved to `/data/systematics.db`
- ✅ **All Major Structures**: 9/12 systematic structures implemented
- ✅ **Six Permutations**: Named pattern generation system
- ✅ **Bennett Schema Integration**: Authentic canonical terms
- ✅ **JSON Export/Import**: Complete database export capability
- ✅ **Search & Discovery**: Full-text search across all fields
- ✅ **Relationship Analysis**: Graph-based connection exploration

### ✅ Developer Experience
- ✅ **Unified Error System**: Comprehensive `SystematicsError` handling
- ✅ **Clean Builder Pattern**: Consistent API across structures
- ✅ **Bidirectional Mapping**: Complete position ↔ term navigation
- ✅ **Comprehensive Testing**: 51+ tests passing
- ✅ **Interactive CLI**: User-friendly menu system
- ✅ **Zero Configuration**: No manual setup required


---

*Built with Rust • Powered by SurrealDB • Inspired by Bennett's Systematics*