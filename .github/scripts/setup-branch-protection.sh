#!/bin/bash
# Script to apply branch protection rules using GitHub CLI
# Requires: gh CLI tool and appropriate permissions

set -e

echo "Setting up branch protection for ferrisdb/ferrisdb main branch..."

# Check if gh is installed
if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed."
    echo "Install it from: https://cli.github.com/"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo "Error: Not authenticated with GitHub CLI."
    echo "Run: gh auth login"
    exit 1
fi

# Apply branch protection rules
echo "Applying branch protection rules..."

# Create the JSON payload
PROTECTION_RULES=$(cat <<'EOF'
{
  "required_status_checks": {
    "strict": true,
    "contexts": [
      "Quick Checks",
      "Markdown Lint", 
      "Spell Check",
      "Test Suite (ubuntu-latest, stable)",
      "Test Suite (windows-latest, stable)",
      "Test Suite (macOS-latest, stable)",
      "Test Suite (ubuntu-latest, beta)",
      "Documentation",
      "MSRV (1.81.0)",
      "Jekyll Site Build",
      "Security Audit",
      "Required Checks"
    ]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": false,
    "required_approving_review_count": 0,
    "require_last_push_approval": false,
    "bypass_pull_request_allowances": {
      "teams": ["ferrisdb/maintainers"]
    }
  },
  "restrictions": null,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "block_creations": false,
  "required_conversation_resolution": true,
  "lock_branch": false,
  "allow_fork_syncing": true,
  "required_linear_history": true
}
EOF
)

echo "$PROTECTION_RULES" | gh api \
  --method PUT \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  /repos/ferrisdb/ferrisdb/branches/main/protection \
  --input -

echo "✅ Branch protection rules applied successfully!"
echo ""
echo "Additional manual steps:"
echo "1. Go to Settings → General → Pull Requests"
echo "2. Enable 'Allow squash merging' only"
echo "3. Enable 'Automatically delete head branches'"
echo "4. Configure squash merge defaults"