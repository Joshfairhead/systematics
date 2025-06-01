# Basic Backend Model - Simple Link Properties

## Overview
This branch implements a **simple, straightforward approach** to modeling relationships between terms in systematic structures. It focuses on explicit, user-definable connections without complex abstractions.

## Design Principles

### 1. **Simplicity First**
- Minimal abstractions - just `Link` struct and `HasLinks` trait
- Easy to understand and modify
- No automatic relationship generation unless explicitly called

### 2. **Explicit Control**
- You define what links exist
- You can name the relationships (or leave them unnamed)
- You can add links one by one as you discover them

### 3. **Flexible Usage**
- Start with no links, add them as you learn about the system
- Some systems can have predefined "basic connections" (optional)
- Custom links can be added anytime

## Implementation

### Core Structures
```rust
#[derive(Debug, Clone)]
pub struct Link {
    pub from: String,
    pub to: String,
    pub label: Option<String>, // Optional name for the relationship
}

pub trait HasLinks {
    fn get_links(&self) -> &Vec<Link>;
    fn add_link(&mut self, link: Link);
    fn get_terms(&self) -> Vec<String>;
}
```

### Usage Patterns

**1. Simple unnamed connections:**
```rust
triad.add_link(Link::new("Active", "Passive"));
```

**2. Named relationships:**
```rust
triad.add_link(Link::with_label("Active", "Passive", "drives"));
```

**3. Optional predefined patterns:**
```rust
triad.add_basic_connections(); // Adds some known triadic relationships
```

## Example Output

```
Triad 'Test Triad' initially has 0 links
After adding basic connections, it has 3 links:
  Initiative --[drives]--> Response
  Response --[flows to]--> Balance
  Balance --[returns to]--> Initiative

After adding custom links, it has 5 links:
  Initiative --[drives]--> Response
  Response --[flows to]--> Balance
  Balance --[returns to]--> Initiative
  Balance --> Initiative
  Response --[supports]--> Balance
```

## Advantages of This Approach

1. **Easy to understand** - no complex relationship types or confidence levels
2. **Incremental discovery** - add links as you learn about them
3. **User control** - you decide what relationships exist
4. **Simple visualization** - direct mapping to graph nodes and edges
5. **Database ready** - maps directly to graph database edge tables

## Hard-Coded Link Properties

Each systematic structure can have:

- **Basic connection methods** (like `add_basic_connections()`) for well-known patterns
- **Custom link addition** for discovered or theoretical relationships
- **Named or unnamed links** depending on your knowledge level
- **Directional relationships** (from -> to structure)

## Next Steps

This approach provides a foundation for:
1. **Manual relationship mapping** - explicitly define what you know
2. **Progressive discovery** - add more links as understanding develops
3. **Graph database storage** - direct mapping to database edges
4. **Future ML integration** - suggested links can be reviewed and added manually

## Status

✅ **Working implementation** on Triad structure
🔄 **Ready for extension** to other systematic structures
📝 **Simple and documented** - easy to understand and modify
🎯 **Focused approach** - does exactly what's needed, nothing more 