#!/bin/bash

# Setup script for pre-commit hooks
# Run this script to configure Git hooks for the msk_musik project

set -e

echo "🔧 Setting up Git pre-commit hooks for msk_musik project..."

# Check if we're in a Git repository
if [ ! -d ".git" ]; then
    echo "❌ Error: This script must be run from the root of the Git repository"
    exit 1
fi

# Create the pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash

# Pre-commit hook for msk_musik project
# This script runs the same checks as the CI workflow before allowing commits

set -e  # Exit on any error

echo "🔍 Running pre-commit checks..."

# 1. Check formatting
echo "📝 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting check failed!"
    echo "💡 Run 'cargo fmt --all' to fix formatting issues."
    exit 1
fi
echo "✅ Code formatting check passed"

# 2. Run clippy lints
echo "🔍 Running clippy lints..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy lints failed!"
    echo "💡 Fix the clippy warnings above before committing."
    exit 1
fi
echo "✅ Clippy lints passed"

# 3. Build workspace
echo "🔨 Building workspace..."
if ! cargo build --all-targets; then
    echo "❌ Build failed!"
    echo "💡 Fix the build errors above before committing."
    exit 1
fi
echo "✅ Build passed"

# 4. Run tests
echo "🧪 Running tests..."
if ! cargo test --all; then
    echo "❌ Tests failed!"
    echo "💡 Fix the failing tests above before committing."
    exit 1
fi
echo "✅ Tests passed"

# 5. Run doc tests
echo "📚 Running doc tests..."
if ! cargo test --doc; then
    echo "❌ Doc tests failed!"
    echo "💡 Fix the failing doc tests above before committing."
    exit 1
fi
echo "✅ Doc tests passed"

echo "🎉 All pre-commit checks passed! Proceeding with commit..."
EOF

# Make the hook executable
chmod +x .git/hooks/pre-commit

echo "✅ Pre-commit hook installed successfully!"
echo ""
echo "📋 What this hook does:"
echo "  • Checks code formatting with 'cargo fmt'"
echo "  • Runs clippy lints with warnings as errors"
echo "  • Builds the entire workspace"
echo "  • Runs all unit tests and doc tests"
echo ""
echo "🚫 Commits will be blocked if any check fails"
echo "💡 Use 'git commit --no-verify' to bypass hooks (not recommended)"
echo ""
echo "🎉 You're all set! Your commits will now be validated locally."