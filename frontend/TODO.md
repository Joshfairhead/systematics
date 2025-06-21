# Frontend Refactoring TODO

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





