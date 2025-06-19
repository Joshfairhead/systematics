# SysteMaster - Development Context

*For strategic roadmap see [ROADMAP.md](ROADMAP.md) | For development tasks see [TODO.md](TODO.md)*

## 🎯 Current System Status

### **Working Components**

The system has a functional architecture with core components operational and a frontend prototype in development.

#### **Development Environment**
- ✅ **Integrated Dev Runner**: `cargo dev` starts coordinated API and frontend servers
- ✅ **API Server**: Port 3001 with REST endpoints for all structures
- ✅ **Frontend Prototype**: Yew-based web interface with geometric rendering
- ✅ **Database**: SurrealDB with RocksDB persistence at `/api/data/systematics.db`

#### **API-Centric Architecture**
- ✅ **REST API Server**: Axum-based HTTP server with structure endpoints
- ✅ **CLI Client**: Command-line interface using API endpoints
- ✅ **Frontend Integration**: Web interface connected to API
- ✅ **CORS Support**: Ready for browser-based clients

#### **Code Quality**
- ✅ **Modern Dependencies**: Updated SurrealDB with RocksDB backend
- ✅ **Error Handling**: Comprehensive `SystematicsError` system
- ✅ **JSON Serialization**: Serde support with proper feature flags
- ✅ **Compiler Warnings**: Clean builds with resolved deprecations

## 🏗️ Architecture Overview

### **Four-Component Architecture**
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Client    │───▶│                 │───▶│   SurrealDB     │
│                 │    │   API Server    │    │                 │
│ • Interactive   │    │                 │    │ • Graph storage │
│ • Menu system   │    │ • REST API      │    │ • RocksDB       │
│ • Auto-start    │    │ • Port 3001     │    │ • Persistence   │
└─────────────────┘    │ • CORS enabled  │    └─────────────────┘
                       │                 │
┌─────────────────┐    │                 │
│ Web Frontend    │───▶│                 │
│                 │    └─────────────────┘
│ • Yew framework │
│ • Port 8081     │
│ • Geometric UI  │
└─────────────────┘
```

### **Component Responsibilities**

#### **CLI (`/cli`)**
- **Purpose**: Command-line user interface and system management
- **Architecture**: HTTP client consuming API endpoints
- **Features**: Auto-start, interactive menu, structure creation
- **Database Access**: None (uses API exclusively)

#### **Frontend (`/frontend`)**
- **Purpose**: Web-based visual interface for systematic structures, eventually system management.
- **Architecture**: Yew WebAssembly application consuming API endpoints
- **Features**: Geometric rendering, interactive overlays, structure visualization
- **Database Access**: None (uses API exclusively)

#### **API (`/api`)**
- **Purpose**: Central business logic and data management
- **Architecture**: Axum HTTP server with SurrealDB integration
- **Features**: REST endpoints, validation, graph storage, CORS support
- **Database**: Direct SurrealDB connection and management

#### **Database (`/api/data`)**
- **Purpose**: Persistent data storage
- **Technology**: SurrealDB with RocksDB backend
- **Location**: API-managed for centralized control
- **Features**: Graph storage, relationships, full-text search

### **Development Startup Flow**
1. **User runs**: `cargo dev`
2. **Dev runner starts**: API server on port 3001
3. **Dev runner starts**: Frontend server on port 8081
4. **Both services**: Run coordinated in background
5. **User accesses**: Frontend at http://localhost:8081 or CLI separately

## 📊 Implementation Status

### ✅ **Completed Components**

#### **Systematic Structures (12/12 Complete)**
- ✅ **Monad (1)**: Unity with custom attributes
- ✅ **Dyad (2)**: Essence/Existence relationships
- ✅ **Triad (3)**: Will/Function/Being dynamics
- ✅ **Tetrad (4)**: Ground/Ideal/Instrumental/Directive
- ✅ **Pentad (5)**: Purpose/Higher Potential/Quintessence/Lower Potential/Source
- ✅ **Hexad (6)**: Complete six-fold structures
- ✅ **Heptad (7)**: Insight through Value frameworks
- ✅ **Octad (8)**: Organisational Modes
- ✅ **Ennead (9)**: Complete nine-fold structures
- ✅ **Decad (10)**: Complete ten-fold structures
- ✅ **Undecad (11)**: Complete eleven-fold structures
- ✅ **Dodecad (12)**: Autocracy through Wholeness (66 connectives)

#### **Core Functionality**
- ✅ **Structure Creation**: All 12 implemented structures
- ✅ **Data Persistence**: Automatic saving to graph database
- ✅ **Search & Discovery**: Full-text search across all fields
- ✅ **Relationship Analysis**: Graph-based connections
- ✅ **JSON Export**: Complete database export capability
- ✅ **Six Permutations**: Named pattern generation
- ✅ **Bennett Schema Integration**: Authentic term characters

#### **Frontend Prototype**
- ✅ **Yew Framework**: WebAssembly-based web interface
- ✅ **Geometric Rendering**: SVG-based structure visualization
- ✅ **API Integration**: Connected to REST endpoints
- ✅ **Interactive Overlays**: Node and edge rendering with labels
- ✅ **Structure Support**: Triad, Tetrad, and Pentad implemented
- ❌ **Remaining**: Geometric rendering for 9 additional structures

#### **Developer Experience**
- ✅ **Zero Configuration**: No manual setup required
- ✅ **Error Handling**: Comprehensive error messages
- ✅ **Testing**: 51+ tests passing
- ✅ **Documentation**: Up-to-date README and context
- ✅ **Cross-Platform**: Tested on macOS, Linux expected

## 🔧 Technical Implementation Details

### **Auto-Start Implementation**
Located in `cli/src/storage.rs`:
```rust
impl StorageCli {
    pub async fn new() -> Result<Self, SystematicsError> {
        // Check if API server is running
        if !api_client.health_check().await.unwrap_or(false) {
            // Start API server in background
            Self::start_api_server().await?;
            // Wait for readiness with timeout
            // Connect and proceed
        }
    }
}
```

### **API Endpoints**
```
GET    /health                    # Health check
GET    /structures                # List all structures
GET    /structures/search?q=term  # Search structures
GET    /structures/:id            # Get specific structure
POST   /structures                # Create new structure
DELETE /structures/:id            # Delete structure
GET    /structures/:id/related    # Get related structures
```

### **Database Schema**
```sql
-- Structures table (nodes)
structures: {
    id: String,
    name: String,
    structure_type: String,
    terms: Array<String>,
    connectives: Object,
    description: Option<String>,
    created_at: DateTime,
    updated_at: DateTime
}

-- Relationships (edges) - automatically generated
term_relations: FROM structures TO structures
```

## 🚀 Running the System

### **Production Usage**
```bash
# Option 1: From CLI directory
cd cli && cargo run

# Option 2: From project root
./start.sh

# Option 3: With custom environment
SYSTEMATICS_API_URL=http://localhost:3001 cd cli && cargo run
```

### **Development Usage**
```bash
# Start API server manually (for development)
cd api && cargo run --bin server --features server

# Run CLI separately
cd cli && cargo run

# Run tests
cargo test --package systematics-api
cargo test --package systematics-cli
```

### **Database Management**
```bash
# Database operations through CLI
cargo run storage list        # List all structures
cargo run storage stats       # Database statistics
cargo run storage export      # Export to JSON
cargo run storage search      # Search structures
```


---

*Development Context - Last Updated: After auto-start implementation and code quality improvements* 