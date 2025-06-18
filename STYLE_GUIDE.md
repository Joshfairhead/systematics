# SysteMaster Style Guide

## Code Standards & Corrections Needed

### API Structure Files Organization
- **Issue**: API structure files could use reorganization for better maintainability
- **Priority**: Medium
- **Location**: `api/src/structures/`

### Method Naming & Documentation
- **Issue**: Content access method names are misleading
- **Example**: `fn first_term()` gets user's first term but naming suggests it returns canonical first term
- **Current**: `/// Get the first user instance (maps to "Will")`
- **Should be**: `/// Get user instance at position 0 (maps to canonical "Will")`
- **Pattern**: All `first_term()`, `second_term()`, etc. methods should clearly indicate they return user instances, not canonical terms
- **Affected files**: All structure files (triad.rs, tetrad.rs, etc.)
- **Priority**: High (affects API clarity)

### Consistent Terminology
- **User instances**: Terms provided by users (stored in database)
- **Canonical terms**: Reference terms from trusted library (e.g., "Will", "Being", "Function")
- **Term characters**: Library method name for canonical terms
- **Positional mapping**: Clear indication of which user instance maps to which canonical term
- Tetrad indexing is ad hoc

### Documentation Standards
- Method comments should clearly distinguish between:
  - User data access (what user provided)
  - Canonical term mapping (what it represents in Bennett's system)
  - Positional coordinates (array indices)

### Frontend Architecture
- **Issue**: SystemOverlay uses hardcoded canonical terms
- **Current**: Hardcoded `"Will"`, `"Being"`, `"Function"` in render methods
- **Should be**: Dynamic fetch from API `/schema/{structure_type}` endpoint
- **Priority**: High (violates separation of concerns)

## Implementation Priorities
1. **High**: Fix SystemOverlay to use API calls for canonical terms
2. **High**: Update method documentation to clarify user vs canonical terms
3. **Medium**: Reorganize API structure files
4. **Low**: Standardize terminology across codebase

## Code Review Checklist
- [ ] Method names clearly indicate user vs canonical data
- [ ] Comments distinguish between user instances and canonical terms
- [ ] No hardcoded canonical terms in frontend
- [ ] API calls used for all canonical term references
- [ ] Consistent terminology throughout 