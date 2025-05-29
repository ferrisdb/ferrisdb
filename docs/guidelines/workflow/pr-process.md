# Pull Request Process

Comprehensive guidelines for creating, reviewing, and merging pull requests in FerrisDB.

## Pull Request Policy

- **All changes must go through PRs** - This includes:
  - Code changes (features, bug fixes, refactoring)
  - Documentation updates (README, guides, comments)
  - Configuration changes (Cargo.toml, CI files)
  - Any file in the repository
- **NO EXCEPTIONS**: Even single-line typo fixes must use PRs
- **CRITICAL**: Never push directly to main branch - always use PRs
- **Maintainers**: Can merge PRs after all CI checks pass (no review required)
- **External contributors**: Require review from a maintainer
- All PRs must pass CI checks before merging
- Use squash merge to keep history clean
- **No direct pushes to main** - Admin privileges are for emergencies only
- **If you accidentally push to main**: Leave it as is, but be more careful in the future

## Development Process (REQUIRED FOR ALL CHANGES)

**Every change, no matter how small, must follow this process:**

1. **Create feature branch**: `git checkout -b <branch-type>/<description>`
2. **Make changes**: Edit files, add tests, update documentation
3. **Lint and format**:
   - Rust: `cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings`
   - Markdown: `prettier --write "**/*.md" && markdownlint-cli2 "**/*.md"`
4. **Commit changes**: Use conventional commit messages
5. **Push branch**: `git push -u origin <branch-name>`
6. **Open PR**: `gh pr create` with descriptive title and body
7. **Iterate if needed**: Push more commits to the feature branch
8. **Merge when ready**: Only after all CI checks pass

### Example Workflow

```bash
# Step 1: Create feature branch
git checkout -b docs/update-readme

# Step 2-3: Make changes, lint, and commit
prettier --write README.md
markdownlint-cli2 README.md
git add README.md
git commit -m "docs: Update installation instructions"

# Step 4: Push branch
git push -u origin docs/update-readme

# Step 5: Create PR
gh pr create --title "docs: Update installation instructions" --body "..."

# Step 6: If changes requested, add more commits
git add .
git commit -m "docs: Address review feedback"
git push

# Step 7: Merge (only after CI passes)
gh pr merge <PR-number> --squash
```

## PR Description Guidelines

**Every PR should include:**

1. **Summary** - Brief overview of changes (2-3 sentences)
2. **Changes Made** - Bullet points of specific modifications
3. **Why This Matters** - Context and motivation
4. **Testing** - What tests were added/modified
5. **Breaking Changes** - Note any API changes (if applicable)

### PR Description Template

```markdown
## Summary

Brief description of what this PR accomplishes and why.

## Changes Made

- Change 1: Description
- Change 2: Description
- Change 3: Description

## Why This Matters

Explain the motivation and benefits of these changes.

## Testing

- Added unit tests for X
- Updated integration tests for Y
- All existing tests pass

## Breaking Changes

None / List any breaking changes here
```

## Good PR Practices

- Keep PRs focused on a single feature/fix
- Include relevant issue numbers (Fixes #123)
- Add reviewers if specific expertise needed
- Update documentation in the same PR as code changes
- Include before/after examples for API changes

## Claude's PR Review Process

When asked to review a PR, Claude follows this structured approach:

1. **Understand Context** 🤖

   - Review PR description to understand the intent
   - Search for additional context if needed
   - Ask clarifying questions if the purpose is unclear
   - Research best practices and industry standards using web search

2. **Review Code Changes** 🤖

   - Examine commit diffs carefully
   - Search for similar patterns in other databases or Rust projects
   - Verify against Rust idioms and database design patterns
   - Check for potential security issues or performance concerns
   - Provide constructive criticism and suggestions
   - Use suggestion code blocks for easy acceptance:

   ```suggestion
   // Your improved code here
   ```

3. **Leverage External Knowledge** 🤖

   - Search for relevant research papers or blog posts
   - Compare with industry best practices
   - Look up unfamiliar patterns or libraries
   - Reference authoritative sources when suggesting improvements
   - Share helpful resources and documentation links

4. **Make a Decision** 🤖

   - **Approve**: Changes look good, only minor nitpicks
   - **Comment**: On the fence, need discussion
   - **Request Changes**: Significant issues need addressing

5. **Follow-Up Reviews** 🤖

   - Check if previous concerns were addressed
   - Verify fixes are appropriate
   - Update review status accordingly

6. **Review Style** 🤖

   - Always include robot emoji in comments
   - Ask hard questions but be constructive
   - Focus on code quality, performance, and maintainability
   - Consider architectural implications
   - Share relevant external resources

7. **Review Decision & API Usage** 🤖
   - If reviewing own PR (same GitHub user): Comment with decision
   - If reviewing others' PRs: Use GitHub API for approve/reject/comment
   - Always clearly state decision: APPROVED ✅, REQUEST CHANGES ❌, or COMMENT 💭
   - Include summary reasoning for decision

### Example Review Comments

```text
🤖 This looks good overall! A few suggestions based on my research:

1. According to the RocksDB implementation, using `Arc<Mutex<T>>` here could cause 
   contention. Consider using a lock-free approach like crossbeam's epoch-based 
   memory reclamation: https://docs.rs/crossbeam-epoch/

2. The error handling pattern here reminds me of how TiKV handles similar cases. 
   They use a custom error type with context:

   return Err(StorageError::InvalidChecksum {
       expected: checksum,
       actual: calculated,
       context: format!("Block at offset {}", offset)
   });

3. I found this excellent article about LSM-tree compaction strategies that might 
   be relevant: [link to article]

These changes would improve both performance and debugging experience.

📊 Review Decision

APPROVED ✅ - The implementation looks solid and follows our patterns well.
```

## Branch Naming Conventions

- **Feature branches**: `feature/description`
- **Bug fixes**: `fix/description`
- **Documentation**: `docs/description`
- **Refactoring**: `refactor/description`
- **Performance**: `perf/description`
- **Tests**: `test/description`
- **CI/Build**: `ci/description` or `build/description`

## Commit Message Format

Use conventional commits format:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `style:` Code style (formatting, missing semicolons, etc)
- `refactor:` Code change that neither fixes a bug nor adds a feature
- `perf:` Performance improvement
- `test:` Adding missing tests
- `chore:` Changes to build process or auxiliary tools

Examples:

- `feat: Add SSTable reader implementation`
- `fix: Correct off-by-one error in skip list`
- `docs: Update API documentation for MemTable`
- `refactor: Extract common logic to utils module`
- `perf: Optimize binary search in index block`

## CI Requirements

All PRs must pass these checks:

1. **Rust formatting**: `cargo fmt --all --check`
2. **Rust linting**: `cargo clippy --all-targets --all-features -- -D warnings`
3. **Rust tests**: `cargo test --all`
4. **Markdown formatting**: `prettier --check "**/*.md"`
5. **Markdown linting**: `markdownlint-cli2 "**/*.md"`
6. **Documentation build**: `cargo doc --all --no-deps`

## Merging Guidelines

1. **Squash merge** for feature branches to keep history clean
2. **Merge commit** only for special cases (preserving commit history)
3. **Never force push** to main branch
4. **Delete branch** after merging (GitHub does this automatically)
5. **Update related issues** after merge

## Review Checklist

Before approving a PR, ensure:

- [ ] Code follows Rust idioms and project guidelines
- [ ] All public APIs have documentation
- [ ] Tests cover new functionality and edge cases
- [ ] No clippy warnings or formatting issues
- [ ] Error messages are descriptive and helpful
- [ ] Performance implications considered
- [ ] Breaking changes are documented
- [ ] TODOs are tracked in TODO.md

## Handling Conflicts

1. **Rebase preferred** over merge for updating feature branches
2. **Communicate** if conflicts affect multiple PRs
3. **Test thoroughly** after resolving conflicts
4. **Document** conflict resolution if complex

## Emergency Procedures

### Accidental Push to Main

1. **Don't panic** - Leave the commit as is
2. **Create a PR** for any additional fixes needed
3. **Document** what happened for transparency
4. **Learn** and be more careful next time

### Broken Main Branch

1. **Create fix PR immediately**
2. **Tag maintainers** for expedited review
3. **Communicate** in relevant channels
4. **Post-mortem** after fix is merged

### Reverting Changes

1. Use `gh pr revert` or GitHub UI
2. Create clear revert message explaining why
3. Link to original PR and issues
4. Plan proper fix in new PR

## Best Practices Summary

1. **Small, focused PRs** are easier to review
2. **Clear descriptions** save reviewer time
3. **Test locally** before pushing
4. **Respond promptly** to review feedback
5. **Update regularly** from main to avoid conflicts
6. **Communicate** if PR is blocked or needs help
7. **Be patient** with reviews and CI
8. **Learn** from review feedback
