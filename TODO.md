# SysteMaster TODO List

## High Priority

### Architectural Refactor (Current)
- [ ] **API-Centric Architecture**: Refactor CLI to use API as central hub instead of direct database access
  - [ ] Move database from `cli/systematics.db` to `data/systematics.db` (project root)
  - [ ] Create HTTP/GraphQL server in API crate
  - [ ] Refactor CLI to call API endpoints instead of direct SurrealStorage
  - [ ] Add GraphQL endpoint for client applications (graph traversals, relationships)
  - [ ] Keep SQL for complex analytics and admin operations
  - [ ] Environment-based database path configuration
- [ ] **Database Access Pattern**: All clients (CLI, frontend, etc.) should go through API layer
- [ ] **Dual Query Support**: GraphQL for graph data, SQL for analytics

### Terminology Refactor
- [ ] Complete LibraryProvider → Source refactor (paused pending architectural clarification)
- [ ] Implement semantic hashing for qualitative positions? (RND)
- [ ] Prototype multidimensional knowledge graph architecture (RND)

### Core System Enhancements
- [ ] Missing Structures: Implement Enneagram (9), Decad (10), Hendecad (11)
- [ ] Position Descriptions: Add position descriptions and rename function to indicate if positions are terms, sources, influences etc.
- [ ] Semantic Alignment: Align semantic positionality between numbers and terms, then remove number from CLI output

## Medium Priority

### Advanced Features
- [ ] **Graph Database Integration**: Implement graph database for systematic structures and relationships
- [ ] **Holochain Integration**: Decentralized storage and sharing of systematic knowledge
- [ ] Export/Import: Save and load structures to/from files (JSON/YAML)
- [ ] Enhanced Validation: More sophisticated input validation and error handling

### Knowledge Graph Features
- [ ] Cross-tradition translation system
- [ ] Automatic semantic pattern discovery
- [ ] Mutual information flow between vertical content and horizontal meaning (related terms in position 1 etc.)
- [ ] Alternative Systems: Create alternative systems beyond Bennett's canonical ones
    - [ ] Landry perspective implementation
    - [ ] Gurdjieff perspective implementation

## Low Priority

### Infrastructure
- [ ] Database Integration: Persistent storage for created structures
- [ ] Relationship Mapping: Advanced connective relationship analysis
- [ ] Structure Comparison: Compare different instances of same structure type
- [ ] Batch Operations: Create multiple structures from templates

### Web Interface
- [ ] Frontend Revival: Modern web interface using existing Yew foundation
- [ ] Interactive Creation: Browser-based structure creation and editing
- [ ] Visualization: Graphical representation of structures and relationships
- [ ] Collaborative Features: Multi-user editing and sharing capabilities

## Interesting Use Cases

### Expert Interview System
- [ ] **Systematic Interview Framework**: Use stored schemas to design structured interview prompts
  - [ ] PhD Project Formulation: Guide doctoral students through comprehensive domain exploration
  - [ ] Requirements Engineering: Systematic elicitation of design/engineering requirements
  - [ ] Domain Knowledge Extraction: Interview experts using canonical terms as scaffolding
  - [ ] Comprehensive Coverage: Ensure all aspects of a domain are explored systematically
  - [ ] Dynamic Prompting: Generate context-aware follow-up questions based on responses
  - [ ] Multi-Level Analysis: Apply different systematic structures (triads, tetrads, etc.) to same domain
- [ ] **Interview Templates**: Pre-built interview frameworks for common domains
- [ ] **Response Analysis**: Pattern recognition in expert responses across systematic positions
- [ ] **Knowledge Synthesis**: Combine multiple expert interviews into comprehensive domain models

## Research & Exploration

### AD4M / REA Integration
- [ ] Engage with AD4M / REA community for collaborative exploration and integration
- [ ] Apply permutations analysis to explore transformation patterns

### Semantic Research
- [ ] Holonic vector space implementation
- [ ] Semantic equivalence patterns across different system levels
- [ ] Cross-framework communication protocols
- [ ] REA (Resource-Event-Agent) accounting integration

## Completed ✅
- ✅ Library/Binary Split: Core API library separated from CLI application
- ✅ Unified Structure API: Complete SystematicStructure trait implementation
- ✅ Clean Builder Pattern: Consistent API across all structure types
- ✅ Schema → System Refactor: Better semantic clarity with systematic naming
- ✅ Modular System Architecture: Clean separation of systematic knowledge
- ✅ Complete Bidirectional Mapping: 8-method API for position ↔ term navigation
- ✅ Six-Component System Enhancement: Coherence attributes, term designations, connectives
- ✅ API Library Separation: Pure mathematical API with pluggable knowledge library

---

*Last updated: After README/TODO cleanup and Schema→System terminology refactor* 