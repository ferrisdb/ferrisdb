#!/bin/bash
# Tutorial Code Synchronization Verification Script
# Run this before committing changes to ensure MDX tutorial content matches implementation

set -e

TUTORIAL_DIR="$(pwd)"
MDX_FILE="../../ferrisdb-docs/src/content/docs/tutorials/01-key-value-store.mdx"

echo "🔍 Verifying Tutorial 01 code synchronization..."
echo "Tutorial dir: $TUTORIAL_DIR"
echo "MDX file: $MDX_FILE"

# Check if files exist
if [[ ! -f "src/lib.rs" ]]; then
    echo "❌ Error: src/lib.rs not found. Run from tutorial-01-kv-store directory."
    exit 1
fi

if [[ ! -f "$MDX_FILE" ]]; then
    echo "❌ Error: MDX file not found at $MDX_FILE"
    exit 1
fi

echo ""
echo "📋 Checking struct definitions..."

# Extract struct definition from implementation
impl_struct=$(grep -A 5 "pub struct KeyValueStore" src/lib.rs | head -6)
echo "Implementation struct:"
echo "$impl_struct"

# Extract struct definition from MDX (check multiple possible locations)
mdx_struct=$(grep -A 5 "pub struct KeyValueStore" "$MDX_FILE" | head -6)
echo ""
echo "Tutorial struct (first occurrence):"
echo "$mdx_struct"

# Compare struct definitions
if [[ "$impl_struct" == "$mdx_struct" ]]; then
    echo "✅ Struct definitions match"
else
    echo "❌ Struct definitions differ!"
    echo "Run 'diff <(echo \"$impl_struct\") <(echo \"$mdx_struct\")' for details"
fi

echo ""
echo "📋 Checking method signatures..."

# Extract method signatures from implementation
impl_methods=$(grep "pub fn" src/lib.rs | sed 's/^[[:space:]]*//')
echo "Implementation methods:"
echo "$impl_methods"

# Check if all implementation methods appear in MDX
echo ""
echo "Checking if all methods appear in tutorial..."
missing_methods=()
while IFS= read -r method; do
    if ! grep -q "$method" "$MDX_FILE"; then
        missing_methods+=("$method")
    fi
done <<< "$impl_methods"

if [[ ${#missing_methods[@]} -eq 0 ]]; then
    echo "✅ All methods appear in tutorial"
else
    echo "❌ Missing methods in tutorial:"
    printf '%s\n' "${missing_methods[@]}"
fi

echo ""
echo "📋 Checking test function names..."

# Extract test function names from implementation
impl_tests=$(grep "#\[test\]" -A 1 src/lib.rs | grep "fn " | sed 's/.*fn \([^(]*\).*/\1/')
echo "Implementation tests:"
echo "$impl_tests"

# Check if test names appear in MDX
echo ""
echo "Checking if test names appear in tutorial..."
missing_tests=()
while IFS= read -r test; do
    if [[ -n "$test" ]] && ! grep -q "$test" "$MDX_FILE"; then
        missing_tests+=("$test")
    fi
done <<< "$impl_tests"

if [[ ${#missing_tests[@]} -eq 0 ]]; then
    echo "✅ All test names appear in tutorial"
else
    echo "❌ Missing test names in tutorial:"
    printf '%s\n' "${missing_tests[@]}"
fi

echo ""
echo "📋 Checking imports..."

# Extract imports from implementation
impl_imports=$(grep "^use " src/lib.rs)
echo "Implementation imports:"
echo "$impl_imports"

# Check if imports appear in MDX
echo ""
echo "Checking if imports appear in tutorial..."
missing_imports=()
while IFS= read -r import; do
    if [[ -n "$import" ]] && ! grep -q "$import" "$MDX_FILE"; then
        missing_imports+=("$import")
    fi
done <<< "$impl_imports"

if [[ ${#missing_imports[@]} -eq 0 ]]; then
    echo "✅ All imports appear in tutorial"
else
    echo "❌ Missing imports in tutorial:"
    printf '%s\n' "${missing_imports[@]}"
fi

echo ""
echo "📋 Running compilation check..."

# Verify implementation compiles
if cargo check --quiet; then
    echo "✅ Implementation compiles"
else
    echo "❌ Implementation doesn't compile"
    exit 1
fi

# Verify main implementation tests pass
if cargo test --lib --quiet; then
    echo "✅ Main implementation tests pass"
else
    echo "❌ Main implementation tests failing"
    exit 1
fi

# Verify solutions tests pass  
if cargo test --test solutions --quiet; then
    echo "✅ Solution tests pass"
else
    echo "❌ Solution tests failing"
    exit 1
fi

# Check exercise templates compile (failures expected)
echo "📋 Checking exercise templates compile..."
if cargo check --test exercises --quiet 2>/dev/null; then
    echo "✅ Exercise templates compile (warnings expected)"
else
    echo "❌ Exercise templates don't compile"
    exit 1
fi

echo ""
echo "🎉 Synchronization verification complete!"
echo ""
echo "📝 Manual checks still needed:"
echo "   - Verify progressive code examples build correctly"
echo "   - Check that final complete code block matches src/lib.rs exactly"
echo "   - Ensure all derives (#[derive(Default)]) are explained when introduced"
echo "   - Confirm method body implementations match between tutorial and code"