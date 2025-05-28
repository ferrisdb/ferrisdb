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

gh api \
  --method PUT \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  /repos/ferrisdb/ferrisdb/branches/main/protection \
  -f "required_status_checks[strict]=true" \
  -f "required_status_checks[contexts][]=Quick Checks" \
  -f "required_status_checks[contexts][]=Markdown Lint" \
  -f "required_status_checks[contexts][]=Spell Check" \
  -f "required_status_checks[contexts][]=Test Suite (ubuntu-latest, stable)" \
  -f "required_status_checks[contexts][]=Test Suite (windows-latest, stable)" \
  -f "required_status_checks[contexts][]=Test Suite (macOS-latest, stable)" \
  -f "required_status_checks[contexts][]=Test Suite (ubuntu-latest, beta)" \
  -f "required_status_checks[contexts][]=Documentation" \
  -f "required_status_checks[contexts][]=MSRV (1.81.0)" \
  -f "required_status_checks[contexts][]=Jekyll Site Build" \
  -f "required_status_checks[contexts][]=Security Audit" \
  -f "required_status_checks[contexts][]=Required Checks" \
  -f "required_pull_request_reviews[dismiss_stale_reviews]=true" \
  -f "required_pull_request_reviews[require_code_owner_reviews]=false" \
  -f "required_pull_request_reviews[required_approving_review_count]=0" \
  -f "required_pull_request_reviews[require_last_push_approval]=false" \
  -f "required_pull_request_reviews[bypass_pull_request_allowances][teams][]=maintainers" \
  -f "enforce_admins=false" \
  -f "restrictions=null" \
  -f "allow_force_pushes=false" \
  -f "allow_deletions=false" \
  -f "required_conversation_resolution=true" \
  -f "lock_branch=false" \
  -f "allow_fork_syncing=true" \
  -f "required_linear_history=true"

echo "✅ Branch protection rules applied successfully!"
echo ""
echo "Additional manual steps:"
echo "1. Go to Settings → General → Pull Requests"
echo "2. Enable 'Allow squash merging' only"
echo "3. Enable 'Automatically delete head branches'"
echo "4. Configure squash merge defaults"