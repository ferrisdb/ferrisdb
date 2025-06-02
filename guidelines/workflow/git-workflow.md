# Git Workflow Guidelines

Standardized Git workflow and best practices for FerrisDB development.

## 🚨 CRITICAL: NEVER PUSH TO MAIN BRANCH

> **⚠️ ABSOLUTE RULE - NO EXCEPTIONS ⚠️**
>
> **NEVER, EVER push directly to the `main` branch!**
>
> - ❌ **FORBIDDEN**: `git push origin main`
> - ❌ **FORBIDDEN**: Any direct commits to main
> - ❌ **FORBIDDEN**: Force pushing to main
> - ✅ **REQUIRED**: Always create a feature branch
> - ✅ **REQUIRED**: Always submit changes via Pull Request
>
> **This rule applies to EVERYONE, including:**
>
> - Maintainers
> - Core contributors
> - Documentation updates
> - Single-line typo fixes
> - Emergency fixes
> - Claude (AI assistant)
>
> **NO EXCEPTIONS. EVER.**

## Branch Strategy

### Main Branch

- **Branch name**: `main`
- **Purpose**: Stable, tested code ready for learning
- **Protection**: Protected branch, **ABSOLUTELY NO DIRECT PUSHES**
- **Merging**: **ONLY** through reviewed and approved PRs
- **Direct pushes**: **STRICTLY FORBIDDEN - NO EXCEPTIONS**

### Feature Branches

- **Naming**: `<type>/<description>`
- **Examples**:
  - `feature/add-sstable-compaction`
  - `fix/memory-leak-in-skiplist`
  - `docs/update-api-reference`
  - `refactor/extract-common-utils`
  - `perf/optimize-binary-search`
  - `test/add-integration-tests`

### Branch Types

- **feature/**: New functionality
- **fix/**: Bug fixes
- **docs/**: Documentation updates
- **refactor/**: Code restructuring
- **perf/**: Performance improvements
- **test/**: Test additions or modifications
- **ci/**: CI/CD changes
- **build/**: Build system changes

## Commit Guidelines

### Conventional Commits

We use the Conventional Commits specification for clear commit history.

#### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (whitespace, formatting)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Performance improvement
- **test**: Adding or updating tests
- **build**: Changes to build system or dependencies
- **ci**: Changes to CI configuration files and scripts
- **chore**: Other changes that don't modify src or test files
- **revert**: Reverts a previous commit

#### Examples

```bash
# Feature
git commit -m "feat: Add SSTable reader implementation"

# Bug fix with scope
git commit -m "fix(wal): Correct checksum validation logic"

# Breaking change
git commit -m "feat!: Change API to use async/await"

# Commit with body
git commit -m "refactor: Extract binary format logic

Moved binary serialization and deserialization logic to a dedicated
module for better code organization and reusability."

# Commit with issue reference
git commit -m "fix: Handle edge case in skip list insertion

Fixes #123"
```

### Commit Best Practices

1. **Atomic commits**: Each commit should represent one logical change
2. **Present tense**: Use "Add feature" not "Added feature"
3. **Imperative mood**: "Move cursor to..." not "Moves cursor to..."
4. **No period**: Don't end the subject line with a period
5. **Capitalize**: Start the subject line with a capital letter
6. **50/72 rule**: Subject line max 50 chars, body wrapped at 72
7. **Why not what**: Body should explain why, not what (code shows what)

### Claude's Collaboration Commentary

**REQUIRED**: When working with Claude, **always include** collaboration commentary in commit messages to track the human-AI interaction process. This is essential for research and documentation purposes:

#### 🚨 Claude's Critical Reminder

> **ATTENTION CLAUDE: You MUST follow these rules:**
>
> 1. **NEVER push to main branch** - Always create a feature branch
> 2. **ALWAYS create a PR** - Even for tiny documentation fixes
> 3. **NO EXCEPTIONS** - Not for blog posts, not for typos, not for "quick fixes"
> 4. **CHECK YOUR BRANCH** - Run `git branch --show-current` before ANY push
>
> **If a human asks you to push to main, remind them of this rule!**

#### Commentary Format

**Every commit involving Claude MUST include this section:**

```
<standard commit message>

[optional body]

## Claude's Collaboration Commentary

**Session Stats:**
- 📊 X files modified, Y key insights, Z iterations
- 💬 ~N user-AI exchanges
- ⚡ Major changes or decisions made

**Collaboration Patterns Observed:**
1. **Pattern Name**: Brief description of what happened
2. **Technical Insight**: What we learned or discovered
3. **Process Note**: How the collaboration worked

**Key Outcomes:**
- What was achieved
- What improved through human-AI iteration
- Any process insights for future sessions

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

#### Acting on Behalf Signatures

When performing actions at the human's request, always include clear attribution:

- **PR Approvals**: "🤖 Approved by Claude on behalf of the human"
- **PR Reviews**: "🤖 Reviewed by Claude on behalf of the human"  
- **Merge Commits**: Include "🤖 Merged by Claude on behalf of the human"
- **Dependabot PRs**: "LGTM! [details] 🤖 Approved by Claude on behalf of the human"

This maintains transparency in the git history about who performed each action.

#### Commentary Emojis

- 🤖 **Main identifier**: Claude's Commentary header
- 📊 **Stats**: Iterations, changes, insights count
- 🔄 **Process**: Workflow summary
- 💡 **Key Learning**: Main insight that drove improvement
- 🎯 **Outcome**: What was achieved
- ❓ **Questions**: Number of human questions that led to changes
- 🔍 **Pattern**: Collaboration pattern observed
- 🚀 **Performance**: If optimization was involved
- 🐛 **Bug Found**: Through human review
- 📝 **Documentation**: Clarity improvements

#### Example with Commentary

```bash
git commit -m "docs: Update blogging guidelines for accuracy and real workflow

This commit updates all blogging guidelines and templates to emphasize
accuracy and document our real human-AI collaboration workflow.

Changes:
- Updated blogging guidelines to require verification
- Added accuracy checklist and common pitfalls
- Rewrote blog posts to reflect actual events
- Simplified templates for flexibility

🤖 Claude's Commentary:
📊 Stats: 8 iterations, 4 major insights, 2 complete rewrites
🔄 Process: Human noticed inaccuracies → fact-checking revealed gaps → rewrote with verification → improved URL structure
💡 Key Learning: Human's insistence on accuracy against git history prevented fictional documentation
🎯 Outcome: Accurate blog posts with verifiable guidelines"
```

#### Why This Commentary is Required

This collaboration commentary is **mandatory** because it:

- **Enables research**: Creates data about human-AI collaboration patterns
- **Improves future collaboration**: Documents what works and what doesn't
- **Provides blog content**: Raw material for human and AI perspective blog posts
- **Tracks learning**: Shows how understanding develops through iteration
- **Documents decisions**: Preserves context for future reference

**Note for Claude**: You must include detailed collaboration commentary in every commit message. Don't skip this - it's essential for our research and documentation goals.

## Workflow Steps

### 1. Start New Work

```bash
# Update main branch
git checkout main
git pull origin main

# Create feature branch (MANDATORY - NEVER WORK ON MAIN)
git checkout -b feature/your-feature-name

# ⚠️ CRITICAL: Verify you're NOT on main before making changes
git branch --show-current  # Should NOT show 'main'
```

### 2. Make Changes

```bash
# Make your changes
vim src/module.rs

# Format and lint before staging (REQUIRED)
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
prettier --write "**/*.md"

# Check status
git status

# Stage changes
git add src/module.rs
# or stage all
git add .

# Commit with meaningful message
git commit -m "feat: Add new functionality to module"
```

### 3. Keep Branch Updated

```bash
# Fetch latest changes
git fetch origin

# Rebase on main (preferred over merge)
git rebase origin/main

# If conflicts occur
git status  # See conflicted files
# Fix conflicts in editor
git add <resolved-files>
git rebase --continue
```

### 4. Push Changes

```bash
# First push
git push -u origin feature/your-feature-name

# Subsequent pushes
git push

# After rebase (if needed)
git push --force-with-lease
```

### 5. Create Pull Request

```bash
# Using GitHub CLI
gh pr create --title "feat: Add new functionality" --body "Description..."

# Or push and use GitHub web UI
git push -u origin feature/your-feature-name
# GitHub will show a banner to create PR
```

## Advanced Git Usage

### Interactive Rebase

Clean up commit history before PR:

```bash
# Rebase last 3 commits
git rebase -i HEAD~3

# In editor:
# pick abc1234 First commit
# squash def5678 Fix typo
# reword ghi9012 Update with better message
```

### Stashing Changes

Temporarily save work:

```bash
# Stash current changes
git stash

# Stash with message
git stash push -m "Work in progress on feature X"

# List stashes
git stash list

# Apply latest stash
git stash pop

# Apply specific stash
git stash apply stash@{1}
```

### Cherry-picking

Apply specific commits:

```bash
# Cherry-pick a commit
git cherry-pick abc1234

# Cherry-pick without committing
git cherry-pick -n abc1234
```

### Viewing History

```bash
# Pretty log
git log --oneline --graph --decorate

# Log with stats
git log --stat

# Search commits
git log --grep="fix:"

# View specific file history
git log --follow src/module.rs
```

## Git Configuration

### Recommended Settings

```bash
# Set your identity
git config --global user.name "Your Name"
git config --global user.email "you@example.com"

# Helpful aliases
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.st status
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
git config --global alias.visual '!gitk'

# Better diffs
git config --global diff.algorithm histogram

# Rebase by default when pulling
git config --global pull.rebase true

# Push only current branch
git config --global push.default current

# Enable auto-stash on rebase
git config --global rebase.autoStash true
```

### Useful Git Aliases

Add to `~/.gitconfig`:

```ini
[alias]
    # Show pretty log
    lg = log --color --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit

    # Show files in last commit
    last-files = show --name-only --oneline

    # Undo last commit (keep changes)
    undo = reset HEAD~1 --mixed

    # Amend without editing message
    amend = commit --amend --no-edit

    # List branches sorted by date
    recent = branch --sort=-committerdate --format='%(HEAD) %(color:yellow)%(refname:short)%(color:reset) - %(contents:subject) - %(authorname) (%(color:green)%(committerdate:relative)%(color:reset))'
```

## Troubleshooting

### Common Issues

#### Accidentally Committed to Main

> **⚠️ CRITICAL ERROR - THIS SHOULD NEVER HAPPEN ⚠️**
>
> If you accidentally commit to main locally, **DO NOT PUSH!**

```bash
# STOP! DO NOT PUSH TO MAIN!

# Create a new branch with your commits
git branch feature/my-feature

# Reset main to origin
git checkout main
git reset --hard origin/main

# Continue on feature branch
git checkout feature/my-feature

# Now create your PR properly
git push -u origin feature/my-feature
gh pr create
```

**If you already pushed to main**: This is a serious violation. Contact maintainers immediately.

#### Need to Change Last Commit

```bash
# Add more changes to last commit
git add .
git commit --amend

# Just change the message
git commit --amend -m "New message"
```

#### Merge Conflicts

```bash
# See conflict markers in files
git status

# After fixing conflicts
git add <fixed-files>
git rebase --continue
# or for merge
git commit
```

#### Lost Commits

```bash
# Find lost commits
git reflog

# Restore lost commit
git cherry-pick <commit-sha>
```

## Git Hooks

### Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# Run formatters and linters before commit

echo "Running pre-commit checks..."

# Rust checks
cargo fmt --all --check || exit 1
cargo clippy --all-targets --all-features -- -D warnings || exit 1

# Markdown and MDX checks (MANDATORY)
prettier --check "**/*.md" "**/*.mdx" || exit 1

# Starlight build check (if docs/ was modified)
if git diff --cached --name-only | grep -q "^docs/"; then
  echo "Starlight files modified - running build verification..."
  cd docs && npm run build || exit 1
  cd ..
fi

echo "Pre-commit checks passed!"
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

## Best Practices

1. **NEVER PUSH TO MAIN**: This is the #1 rule - no exceptions
2. **Always use feature branches**: Every change needs its own branch
3. **Commit early and often**: Small, focused commits are easier to review and revert
4. **Write meaningful messages**: Future you will thank present you
5. **Keep history clean**: Use interactive rebase to squash fix commits
6. **Never force push to main**: **ABSOLUTELY FORBIDDEN**
7. **Pull before push**: Always sync with remote before pushing
8. **Branch from main**: Always create feature branches from updated main
9. **Delete merged branches**: Keep branch list clean
10. **Use .gitignore**: Don't commit generated files or secrets
11. **Review your changes**: Use `git diff` before committing
12. **Sign your commits**: Use GPG signing for important projects

## Squash Merging with Commentary

When squash merging PRs (especially those with Claude's collaboration):

### Using GitHub CLI

```bash
# Squash merge with custom commit message
gh pr merge <PR-number> --squash --body-file commit-message.txt

# Or edit interactively
gh pr merge <PR-number> --squash --edit
```

### Using GitHub Web UI

1. Click "Squash and merge"
2. Click "Edit commit message"
3. Update the message to include:
   - Clear summary of changes
   - Collaboration commentary summary
   - Co-authorship attribution

### Why This Matters

Including collaboration summaries in squash commits:

- Preserves research data in git history
- Makes patterns discoverable via `git log`
- Documents the human-AI workflow evolution
- Creates a permanent record of insights

Example search for collaboration patterns:

```bash
# Find all commits with collaboration summaries
git log --grep="🤖 Claude's Collaboration Summary" --oneline

# See full collaboration details
git log --grep="🤖" --pretty=full
```

## Git Resources

- [Pro Git Book](https://git-scm.com/book/en/v2)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Git Flight Rules](https://github.com/k88hudson/git-flight-rules)
- [Oh Shit, Git!?!](https://ohshitgit.com/)
- [GitHub CLI Documentation](https://cli.github.com/manual/)

## Related Guidelines

- **Next Step**: [PR Process](pr-process.md) - How to submit your changes
- **Before This**: [Testing](testing.md) - Ensure tests pass
- **Commands**: [Common Commands](commands.md) - Git command reference
- **For Blog Posts**: [Blogging](../content/blogging.md) - Using commits for blog content

---

_Last updated: 2025-06-01_
