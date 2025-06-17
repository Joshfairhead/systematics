# SysteMaster - Development Context

## 🎯 Current System Status

### ✅ **Fully Operational Architecture**

The system has reached a mature, production-ready state with all major components working seamlessly together.

#### **Auto-Start System (Latest Achievement)**
- ✅ **One-Command Startup**: `cd cli && cargo run` starts entire system
- ✅ **Automatic API Server**: Starts API server in background if not running
- ✅ **Port 3001**: Standardized on port 3001 to avoid Obsidian conflicts
- ✅ **Cross-Platform**: Works on macOS, Linux, and Windows
- ✅ **30-Second Timeout**: Robust error handling for server startup
- ✅ **Background Process**: API server runs silently in background

#### **API-Centric Architecture**
- ✅ **REST API Server**: Axum-based HTTP server with comprehensive endpoints
- ✅ **CLI as Client**: CLI now properly uses API endpoints (not direct DB access)
- ✅ **SurrealDB Backend**: Graph database with RocksDB persistence
- ✅ **Database Location**: Centralized at `/data/systematics.db`
- ✅ **CORS Support**: Ready for frontend integration

#### **Code Quality Improvements**
- ✅ **Deprecated Features Fixed**: Updated SurrealDB `File` → `RocksDb`
- ✅ **Feature Warnings Resolved**: Fixed `serde_support` → `serde`
- ✅ **Unused Variables**: Cleaned up all compiler warnings
- ✅ **Error Handling**: Comprehensive `SystematicsError` system
- ✅ **JSON Serialization**: Optional serde support with default implementations

## 🏗️ Architecture Overview

### **Three-Tier Architecture**
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Client    │───▶│   API Server    │───▶│   SurrealDB     │
│                 │    │                 │    │                 │
│ • Auto-start    │    │ • REST API      │    │ • Graph storage │
│ • Interactive   │    │ • Port 3001     │    │ • RocksDB       │
│ • Menu system   │    │ • Background    │    │ • Persistence   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### **Component Responsibilities**

#### **CLI (`/cli`)**
- **Purpose**: User interface and interaction
- **Architecture**: HTTP client consuming API endpoints
- **Features**: Auto-start, interactive menu, structure creation
- **Database Access**: None (uses API exclusively)

#### **API (`/api`)**
- **Purpose**: Central business logic and data management
- **Architecture**: Axum HTTP server with SurrealDB integration
- **Features**: REST endpoints, validation, graph storage
- **Database**: Direct SurrealDB connection and management

#### **Database (`/data`)**
- **Purpose**: Persistent data storage
- **Technology**: SurrealDB with RocksDB backend
- **Location**: Project root for centralized management
- **Features**: Graph storage, relationships, full-text search

### **Startup Flow**
1. **User runs**: `cd cli && cargo run`
2. **CLI checks**: API server health at `http://localhost:3001`
3. **If not running**: CLI starts API server in background
4. **CLI waits**: Up to 30 seconds for server to be ready
5. **Connection established**: CLI connects and shows menu
6. **API server runs**: Continues in background for subsequent CLI runs

## 📊 Implementation Status

### ✅ **Completed Components**

#### **Systematic Structures (9/12 Complete)**
- ✅ **Monad (1)**: Unity with custom attributes
- ✅ **Dyad (2)**: Essence/Existence relationships
- ✅ **Triad (3)**: Will/Function/Being dynamics
- ✅ **Tetrad (4)**: Ground/Ideal/Instrumental/Directive
- ✅ **Pentad (5)**: Purpose/Higher Potential/Quintessence/Lower Potential/Source
- ✅ **Hexad (6)**: Complete six-fold structures
- ✅ **Heptad (7)**: Insight through Value frameworks
- ✅ **Octad (8)**: Organisational Modes
- ✅ **Dodecad (12)**: Autocracy through Wholeness (66 connectives)
- ❌ **Missing**: Enneagram (9), Decad (10), Hendecad (11)

#### **Core Functionality**
- ✅ **Structure Creation**: All 9 implemented structures
- ✅ **Data Persistence**: Automatic saving to graph database
- ✅ **Search & Discovery**: Full-text search across all fields
- ✅ **Relationship Analysis**: Graph-based connections
- ✅ **JSON Export**: Complete database export capability
- ✅ **Six Permutations**: Named pattern generation
- ✅ **Bennett Schema Integration**: Authentic canonical terms

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

## 🎯 Architecture Principles

### **Design Decisions**
1. **API-First**: All data access goes through REST API
2. **Zero Configuration**: System starts without manual setup
3. **Background Processing**: API server runs silently
4. **Graph Database**: SurrealDB for relationships and search
5. **Type Safety**: Rust's type system for reliability
6. **Bennett Authenticity**: Canonical terms and relationships

### **Separation of Concerns**
- **CLI**: User interaction and experience
- **API**: Business logic and data validation
- **Database**: Persistent storage and relationships
- **Library**: Bennett's systematic knowledge

## 🔄 Recent Achievements

### **Auto-Start System (Latest)**
- **Problem**: Users had to manually start API server
- **Solution**: CLI automatically starts API server when needed
- **Impact**: Single command startup, improved user experience

### **Code Quality Improvements**
- **Problem**: Deprecated SurrealDB features, compiler warnings
- **Solution**: Updated to current APIs, fixed all warnings
- **Impact**: Clean compilation, future-ready codebase

### **Architecture Maturity**
- **Problem**: CLI was accessing database directly
- **Solution**: Proper API-centric architecture
- **Impact**: Scalable, maintainable system ready for multiple clients

---

*Development Context - Last Updated: After auto-start implementation and code quality improvements* 