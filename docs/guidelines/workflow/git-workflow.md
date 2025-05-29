# Git Workflow Guidelines

Standardized Git workflow and best practices for FerrisDB development.

## Branch Strategy

### Main Branch

- **Branch name**: `main`
- **Purpose**: Stable, production-ready code
- **Protection**: Protected branch, no direct pushes
- **Merging**: Only through reviewed and approved PRs

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

## Workflow Steps

### 1. Start New Work

```bash
# Update main branch
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Changes

```bash
# Make your changes
vim src/module.rs

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

```bash
# Create a new branch with your commits
git branch feature/my-feature

# Reset main to origin
git checkout main
git reset --hard origin/main

# Continue on feature branch
git checkout feature/my-feature
```

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

# Markdown checks
prettier --check "**/*.md" || exit 1
markdownlint-cli2 "**/*.md" || exit 1

echo "Pre-commit checks passed!"
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

## Best Practices

1. **Commit early and often**: Small, focused commits are easier to review and revert
2. **Write meaningful messages**: Future you will thank present you
3. **Keep history clean**: Use interactive rebase to squash fix commits
4. **Never force push to main**: Only force push your own feature branches
5. **Pull before push**: Always sync with remote before pushing
6. **Branch from main**: Always create feature branches from updated main
7. **Delete merged branches**: Keep branch list clean
8. **Use .gitignore**: Don't commit generated files or secrets
9. **Review your changes**: Use `git diff` before committing
10. **Sign your commits**: Use GPG signing for important projects

## Git Resources

- [Pro Git Book](https://git-scm.com/book/en/v2)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Git Flight Rules](https://github.com/k88hudson/git-flight-rules)
- [Oh Shit, Git!?!](https://ohshitgit.com/)
- [GitHub CLI Documentation](https://cli.github.com/manual/)
