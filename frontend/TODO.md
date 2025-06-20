# Frontend Refactoring TODO

## ✅ COMPLETED - Terminology Consistency

### ✅ **StoredStructure Field Rename**
- ✅ Renamed `StoredStructure.terms` → `StoredStructure.user_instance_index` 
- ✅ Updated all references in frontend code
- ✅ Updated API integration to match backend field names
- ✅ Ensures clear separation: `user_instance_index` = user data, `term_characters` = schema data

### ✅ **SystemOverlay Parameter Clarity**
- ✅ Renamed `term_index` → `position_index` in method signatures
- ✅ This parameter represents positional mapping, not term-specific data
- ✅ Updated all method signatures and usage in `system_overlay.rs`

### ✅ **Method Parameter Consistency**
- ✅ Reviewed all method parameters for terminology clarity
- ✅ Ensured consistent use of:
  - `user_instances` for user-provided data
  - `term_characters` for schema/Bennett definitions  
  - `display_values` for what's currently shown
  - `position_index` for array/layout positions

### ✅ **API Integration Consistency**
- ✅ Updated `CreateStructureRequest.terms` → `CreateStructureRequest.user_instance_index`
- ✅ Updated all placeholder structure creation methods
- ✅ Updated search functionality to use `user_instance_index`
- ✅ Updated display logic to use correct field names

### ✅ **Frontend Cursor Rules**
- ✅ Created `.cursor/rules/frontend-terminology.mdc` with comprehensive guidelines
- ✅ Established enforcement patterns for terminology consistency
- ✅ Documented correct vs incorrect patterns with examples

## Medium Priority - Architecture Hardening

### 4. **Frontend Reference Hardening**
- [ ] Review all API field references for consistency with backend
- [ ] Add type safety for structure field access
- [ ] Consider creating frontend-specific types that map cleanly to API types
- [ ] Add validation for structure data integrity

### 5. **SystemOverlay Refactoring**
- [ ] Extract position calculation logic into separate utility
- [ ] Simplify the rendering pipeline for better maintainability  
- [ ] Add proper error handling for malformed structure data
- [ ] Consider breaking large methods into smaller, focused functions

### 6. **State Management Improvements**
- [ ] Review prop drilling in SystemOverlay component
- [ ] Consider using context or state management for complex data flow
- [ ] Ensure consistent state updates across all components

## Low Priority - Code Quality

### 7. **Type Safety Enhancements**
- [ ] Add stronger typing for structure positions and indices
- [ ] Create enums for system types instead of string matching
- [ ] Add compile-time checks for structure field access

### 8. **Documentation**
- [ ] Document the distinction between instances vs term_characters
- [ ] Add inline documentation for complex rendering logic
- [ ] Create architecture decision records for terminology choices

## 🎉 Achievement Summary

**TERMINOLOGY CONSISTENCY ACHIEVED!** 

The frontend now uses identical terminology to the backend:
- ✅ `user_instance_index` field matches backend exactly
- ✅ `position_index` parameters are clear about their purpose
- ✅ All API integration uses consistent field names
- ✅ Clear separation between user data and schema data
- ✅ Comprehensive Cursor rules established for future development

**Technical Debt Eliminated:**
- No more confusion between `terms`, `user_terms`, and `user_instance_index`
- Clear distinction between user instances (data) and term characters (schema)
- Frontend-backend field name alignment prevents integration bugs
- Developer experience significantly improved with clear naming

**Compilation Status:** ✅ All frontend code compiles successfully with only expected warnings.

## Notes

- The backend already uses consistent terminology with `user_instance_index()`
- Frontend now aligns with backend patterns for maintainability
- These changes eliminate confusion between user data and schema data
- Priority was on terminology consistency first, then architectural improvements
- Cursor rules provide ongoing enforcement for future development 