# SystematicStructure API Migration Guide

**Version**: 2025.01 (Commit: `ca743d8`)  
**Type**: Breaking Changes - Method Renames & Layout Standardization

## 🚨 Breaking Changes Overview

This migration guide covers the comprehensive refactor of the `SystematicStructure` trait that introduces breaking changes to method names and API responses.

## 📋 Method Renames

### **Core Trait Methods**

| **Old Method** | **New Method** | **Return Type** | **Description** |
|----------------|----------------|-----------------|------------------|
| `first_order_connectives_name()` | `first_order_connectives_type()` | `&str` | Returns the type/category name for connectives |
| `connectives()` | `connectives_traits()` | `&HashMap<(usize, usize), String>` | Returns actual connective relationships |

### **Layout Reorganization**
- `user_instance_index()` moved to **Content Access** section (after `term_characters()`)
- All 12 structure files now follow **monad.rs** as the reference layout

## 🔧 Code Migration

### **Rust Code Updates**

#### **Before (Old API)**
```rust
// Old method calls
let connective_type = structure.first_order_connectives_name();
let relationships = structure.connectives();

// Old trait usage
impl SystematicStructure for MyStructure {
    fn first_order_connectives_name(&self) -> &str {
        "MyConnectives"
    }
    
    fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.relationships
    }
}
```

#### **After (New API)**
```rust
// New method calls
let connective_type = structure.first_order_connectives_type();
let relationships = structure.connectives_traits();

// New trait usage
impl SystematicStructure for MyStructure {
    fn first_order_connectives_type(&self) -> &str {
        "MyConnectives"
    }
    
    fn connectives_traits(&self) -> &HashMap<(usize, usize), String> {
        &self.relationships
    }
}
```

### **JSON API Response Changes**

#### **Before (Old Schema)**
```json
{
  "id": "structure-123",
  "name": "Example Structure",
  "first_order_connectives_name": "Forces",
  "connectives": [
    {"from": 0, "to": 1, "relationship": "influences"}
  ]
}
```

#### **After (New Schema)**
```json
{
  "id": "structure-123", 
  "name": "Example Structure",
  "first_order_connectives_type": "Forces",
  "connectives": [
    {"from": 0, "to": 1, "relationship": "influences"}
  ]
}
```

## 🌐 Frontend/Client Updates

### **JavaScript/TypeScript**
```typescript
// Before
interface StructureResponse {
  id: string;
  name: string;
  first_order_connectives_name: string;
  connectives: ConnectiveInfo[];
}

// After  
interface StructureResponse {
  id: string;
  name: string;
  first_order_connectives_type: string;  // ← Field name changed
  connectives: ConnectiveInfo[];
}

// Update API calls
const response = await fetch('/api/structures/123');
const structure = await response.json();
console.log(structure.first_order_connectives_type); // ← Use new field name
```

### **Python/HTTP Clients**
```python
# Before
response = requests.get(f"{api_url}/structures/{structure_id}")
data = response.json()
connective_name = data["first_order_connectives_name"]

# After
response = requests.get(f"{api_url}/structures/{structure_id}")  
data = response.json()
connective_type = data["first_order_connectives_type"]  # ← Field name changed
```

## 🗂️ File Structure Changes

### **Reference Implementation**
**monad.rs** now serves as the definitive layout reference for all structure implementations:

```rust
impl SystematicStructure for Monad {
    // -------------------------------------------------------------------------
    // Core Identity & Structure  
    // -------------------------------------------------------------------------
    fn id(&self) -> &str { ... }
    fn name(&self) -> &str { ... }
    fn structure_type(&self) -> &str { ... }
    fn coherence_attribute(&self) -> &str { ... }
    fn term_designation(&self) -> &str { ... }
    fn source(&self) -> &str { ... }
    
    // -------------------------------------------------------------------------
    // Content Access
    // -------------------------------------------------------------------------
    fn term_characters(&self) -> Vec<String> { ... }
    fn user_instance_index(&self) -> &[String] { ... }  // ← Moved here
    fn first_order_connectives_type(&self) -> &str { ... }  // ← Renamed
    fn connectives_traits(&self) -> &HashMap<(usize, usize), String> { ... }  // ← Renamed
}
```

## ⚠️ Important Notes

### **What Didn't Change**
- **Library System Methods**: `system.first_order_connectives_name()` remains unchanged
- **Functionality**: No behavioral changes, only naming improvements
- **Data Structure**: Underlying data structures remain identical
- **API Endpoints**: URLs and HTTP methods unchanged

### **Mixed Usage Pattern**
```rust
// This is correct - different contexts use different methods
let system_method = structure.system.first_order_connectives_name();  // Library method (unchanged)
let trait_method = structure.first_order_connectives_type();          // Trait method (renamed)
```

## 🔍 Testing Your Migration

### **Compilation Test**
```bash
# Should compile without errors after migration
cargo check --features server
```

### **Basic Functionality Test** 
```bash
# Core functionality should work unchanged
cargo test structures::monad::tests --lib
```

### **API Response Test**
```bash
# Check JSON response has new field names
curl http://localhost:3001/structures | jq '.[] | .first_order_connectives_type'
```

## 📞 Support

### **Common Issues**
1. **CLI Compilation Error**: "method `terms` not found"
   - **Solution**: Update CLI to use `user_instance_index()` - Fixed in commit `8b86a1b`

2. **Compilation Error**: "method `first_order_connectives_name` not found"
   - **Solution**: Update to `first_order_connectives_type()`

3. **JSON Parse Error**: "field `first_order_connectives_name` missing" 
   - **Solution**: Update JSON parsing to use `first_order_connectives_type`

4. **Test Failures**: Method not found in test assertions
   - **Solution**: Update test method calls to new names

### **Known Remaining Inconsistencies**
These are documented for future cleanup but don't affect functionality:

1. **Field Name Inconsistency**: 
   - Monad uses `user_instance_index` field (consistent with trait method)
   - Other structures use `user_term_index` field (inconsistent naming)
   - **Impact**: Confusing for developers but functionally correct

2. **Comment/Implementation Mismatch**:
   - Comments say "Get first user instance" 
   - But access `user_term_index[0]` instead of `user_instance_index[0]`
   - **Impact**: Misleading documentation but works correctly

3. **Method Naming Patterns**:
   - Mix of `get_term()` and direct field access patterns
   - **Impact**: Inconsistent API surface but functional

### **Migration Checklist**
- [ ] Updated all `first_order_connectives_name()` calls to `first_order_connectives_type()`
- [ ] Updated all `connectives()` calls to `connectives_traits()`  
- [ ] Updated JSON parsing to use `first_order_connectives_type` field
- [ ] Updated frontend/client code to use new field names
- [ ] Tested compilation with `cargo check`
- [ ] Verified API responses use new field names
- [ ] Updated any custom tests or integrations

## 🎯 Benefits of Migration

1. **Clarity**: Method names now clearly indicate their purpose
2. **Consistency**: Uniform layout across all 12 structure implementations
3. **Maintainability**: Single reference implementation (monad.rs) for all structures
4. **Disambiguation**: Clear separation between trait methods and library system methods

---

*Migration Guide - January 2025 - SystematicStructure Trait Refactor* 





## 📋 Future Cleanup Tasks

The following inconsistencies remain for future standardization:

### **Field Naming Standardization**
```rust
// Current inconsistency - should be unified
struct Monad {
    user_instance_index: [String; 1],  // ✅ Consistent with trait method
}

struct Heptad {
    user_term_index: [String; 7],      // ❌ Inconsistent naming
}

// Proposed fix: Rename all to user_instance_index
```

### **Comment/Code Alignment**
```rust
// Current mismatch
/// Get the first user instance (maps to "Insight") 
pub fn first_user_instance(&self) -> &str {
    &self.user_term_index[0]  // ❌ Should be user_instance_index[0]
}
```

### **Method Pattern Unification**
```rust
// Mixed patterns exist:
pub fn get_term(&self, index: usize) -> Option<&str> {
    self.user_term_index.get(index).map(|s| s.as_str())
}
// vs direct field access in other methods
```

### **Server Schema Mapping** 
The server correctly maps library methods to API fields:
```rust
// server.rs - This is correct ✅
first_order_connectives_type: system.first_order_connectives_name().to_string(),
```

### **Recommended Approach**
1. **Phase 1** (Completed): Trait method renames and API schema updates
2. **Phase 2** (Future): Field name standardization (`user_term_index` → `user_instance_index`)
3. **Phase 3** (Future): Comment/documentation alignment
4. **Phase 4** (Future): Method pattern unification