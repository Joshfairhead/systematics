#!/bin/bash

# Comprehensive Expression Terminology Cleanup Script
# Converts deprecated "instance" terminology to "expression" throughout SysteMaster

echo "🚀 Starting Expression Terminology Cleanup..."

# Define directories to process
DIRS=(
    "api/src"
    "cli/src"
    "frontend/src"
    "library/src"
)

# Define file patterns
PATTERNS="*.rs"

echo "📁 Processing directories: ${DIRS[*]}"

# Phase 1: Method names and function signatures
echo "🔧 Phase 1: Method names and function signatures..."
for dir in "${DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo "  Processing $dir..."
        find "$dir" -name "$PATTERNS" -type f -exec sed -i '' \
            -e 's/user_instance_index(/user_expressions(/g' \
            -e 's/fn user_instance_index/fn user_expressions/g' \
            -e 's/first_user_instance(/first_user_expression(/g' \
            -e 's/fn first_user_instance/fn first_user_expression/g' \
            -e 's/second_user_instance(/second_user_expression(/g' \
            -e 's/fn second_user_instance/fn second_user_expression/g' \
            -e 's/third_user_instance(/third_user_expression(/g' \
            -e 's/fn third_user_instance/fn third_user_expression/g' \
            -e 's/fourth_user_instance(/fourth_user_expression(/g' \
            -e 's/fn fourth_user_instance/fn fourth_user_expression/g' \
            -e 's/fifth_user_instance(/fifth_user_expression(/g' \
            -e 's/fn fifth_user_instance/fn fifth_user_expression/g' \
            {} +
    fi
done

# Phase 2: Field names and struct members
echo "🔧 Phase 2: Field names and struct members..."
for dir in "${DIRS[@]}"; do
    if [ -d "$dir" ]; then
        find "$dir" -name "$PATTERNS" -type f -exec sed -i '' \
            -e 's/user_instance_index:/user_expressions:/g' \
            -e 's/instances:/user_expressions:/g' \
            -e 's/\.instances/.user_expressions/g' \
            {} +
    fi
done

# Phase 3: Variable names and local bindings
echo "🔧 Phase 3: Variable names and local bindings..."
for dir in "${DIRS[@]}"; do
    if [ -d "$dir" ]; then
        find "$dir" -name "$PATTERNS" -type f -exec sed -i '' \
            -e 's/let instances =/let user_expressions =/g' \
            -e 's/let user_instances =/let user_expressions =/g' \
            -e 's/user_instances\./user_expressions\./g' \
            {} +
    fi
done

# Phase 4: Comments and documentation
echo "🔧 Phase 4: Comments and documentation..."
for dir in "${DIRS[@]}"; do
    if [ -d "$dir" ]; then
        find "$dir" -name "$PATTERNS" -type f -exec sed -i '' \
            -e 's/user instance/user expression/g' \
            -e 's/User instance/User expression/g' \
            -e 's/USER INSTANCE/USER EXPRESSION/g' \
            -e 's/user instances/user expressions/g' \
            -e 's/User instances/User expressions/g' \
            {} +
    fi
done

# Phase 5: API and database field names (be more careful here)
echo "🔧 Phase 5: API and database fields..."
for dir in "${DIRS[@]}"; do
    if [ -d "$dir" ]; then
        find "$dir" -name "$PATTERNS" -type f -exec sed -i '' \
            -e 's/"instances"/"user_expressions"/g' \
            -e 's/instances = /user_expressions = /g' \
            {} +
    fi
done

echo "✅ Expression terminology cleanup complete!"
echo "🔍 Run 'cargo check' to identify any remaining issues." 