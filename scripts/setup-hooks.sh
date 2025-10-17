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

# Use the Makefile's ci-check target to ensure consistency with CI
if ! make ci-check; then
    echo "❌ Pre-commit checks failed!"
    echo "💡 Fix the issues above before committing."
    exit 1
fi

echo "🎉 All pre-commit checks passed! Proceeding with commit..."
EOF

# Make the hook executable
chmod +x .git/hooks/pre-commit

echo "✅ Pre-commit hook installed successfully!"
echo ""
echo "📋 What this hook does:"
echo "  • Runs 'make ci-check' to execute the full CI workflow"
echo "  • Ensures consistency between local pre-commit and CI checks"
echo "  • Includes formatting, linting, building, and testing"
echo ""
echo "🚫 Commits will be blocked if any check fails"
echo "💡 Use 'git commit --no-verify' to bypass hooks (not recommended)"
echo "💡 Run 'make ci-check' manually to test before committing"
echo ""
echo "🎉 You're all set! Your commits will now be validated locally."