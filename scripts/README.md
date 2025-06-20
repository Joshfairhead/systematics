# SysteMaster Maintenance Scripts

## cleanup.sh

Keeps the project light by cleaning up build artifacts and optimizing the git repository.

**Usage:**
```bash
./scripts/cleanup.sh
```

**What it does:**
- Cleans Rust build artifacts (`cargo clean`)
- Optimizes git repository (`git gc`)
- Removes database files from data/
- Cleans frontend build artifacts
- Shows final project size

**When to run:**
- After major development sessions
- Before committing large changes
- When project size exceeds 1GB
- Weekly maintenance

**Expected results:**
- Project size should be ~300-400MB after cleanup
- Git repository should be well-optimized
- No stale build artifacts or database files 