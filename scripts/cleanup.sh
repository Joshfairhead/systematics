#!/bin/bash

# SysteMaster Cleanup Script
# Run this periodically to keep the project light

echo "🧹 Cleaning up SysteMaster..."

# Clean Rust build artifacts
echo "  Cleaning Rust build artifacts..."
cargo clean

# Clean git repository
echo "  Optimizing git repository..."
git gc --quiet

# Remove any accidental database files
echo "  Removing database files..."
find data/ -name "*.db" -delete 2>/dev/null || true
find data/ -name "*.sqlite*" -delete 2>/dev/null || true

# Clean frontend build artifacts
echo "  Cleaning frontend artifacts..."
rm -rf frontend/pkg/ frontend/dist/ 2>/dev/null || true

# Show final size
echo "  Final project size:"
du -h -d 1 . | grep -E "^\s*[0-9].*\.$" | head -1

echo "✅ Cleanup complete!" 