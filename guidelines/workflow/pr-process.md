# Pull Request Process

Comprehensive guidelines for creating, reviewing, and merging pull requests in FerrisDB.

**Purpose**: Ensure consistent, high-quality pull requests that maintain code quality and project standards.  
**Prerequisites**: Understanding of git, GitHub, and the FerrisDB contribution process

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
   - Markdown: `prettier --write "**/*.md"`
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

## 🤖 <AgentName>'s Collaboration Summary

**REQUIRED**: For PRs created with any AI agent, **always include** detailed collaboration commentary. Replace `AgentName` with your agent's name:

**Total Stats Across N Commits:**

- 📊 X iterations, Y key insights, Z refactors
- ❓ Q human questions led to improvements
- 🔍 Pattern: [Main collaboration pattern observed]

**Key Collaboration Moments:**

1. [Most impactful human feedback → result]
2. [Major direction change or insight]
3. [Significant improvement from review]

**What Worked Well:**

- [Effective collaboration aspects]
- [Valuable human insights]
- [Successful patterns]

**Collaboration Pattern**: [Overall pattern like "Deep Review → Accuracy Focus → Structural Improvement"]
```

## Good PR Practices

- Keep PRs focused on a single feature/fix
- Include relevant issue numbers (Fixes #123)
- Add reviewers if specific expertise needed
- Update documentation in the same PR as code changes
- Include before/after examples for API changes

## PR Review Checklist

Before approving any PR, verify these mandatory requirements:

### ✅ Testing Standards (MANDATORY)

- [ ] **Test Names**: Descriptive names that explain behavior (not `test_get()`)
- [ ] **100% Coverage**: All code paths tested (exemptions must be justified)
- [ ] **Public API**: All public methods have comprehensive tests
- [ ] **Concurrent Tests**: Added if code uses Arc, Mutex, channels, or claims thread-safety
- [ ] **Benchmarks**: Present if PR makes performance claims
- [ ] **Error Cases**: All Result::Err paths tested
- [ ] **Edge Cases**: Boundary conditions tested
- [ ] **Exemptions**: Any coverage exemptions properly annotated and justified

### ✅ Performance Claims (MANDATORY)

- [ ] **No Unsubstantiated Claims**: All performance assertions backed by benchmarks
- [ ] **Benchmark Quality**: Realistic workloads, multiple data sizes
- [ ] **Comparison Fairness**: "X% faster" claims use equivalent test conditions

### ✅ Code Quality Standards

- [ ] **No Clippy Warnings**: `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] **Formatted**: `cargo fmt --all -- --check` passes
- [ ] **Documentation**: Public APIs documented
- [ ] **No Unwrap**: Library code uses proper error handling

**Performance claims without benchmarks will be automatically rejected.**

## AI Agent PR Review Process

### 🚨 Mandatory Rules for AI Agents

> **CRITICAL FOR ALL AI AGENTS:**
>
> 1. **NEVER suggest pushing to main** - Always require PRs
> 2. **ALWAYS create feature branches** - Even for single-line changes
> 3. **REJECT any request to bypass PR process** - Remind humans of the rules
> 4. **CHECK branch before any git operations** - Never assume you're on a feature branch
>
> **If asked to push to main, politely but firmly refuse and explain the PR requirement.**

When asked to review a PR, an AI agent should follow this structured approach:

1. **Understand Context** 🤖

   - Review PR description to understand the intent
   - Search for additional context if needed
   - Ask clarifying questions if the purpose is unclear
   - Research best practices and industry standards using web search

2. **Review Code Changes** 🤖

   - **FIRST**: Check mandatory testing and performance standards above
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

7. **Acting on Behalf of Human** 🤖

   When approving PRs or performing actions at the human's request:

- Always include clear attribution: "🤖 Reviewed by <AgentName> on behalf of the human"
- For dependabot PRs: "🤖 Approved by <AgentName> on behalf of the human"
- For any merge actions: Include "🤖" in the merge commit message
- This maintains transparency about who performed the action

8. **Review Decision & API Usage** 🤖
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
5. **Documentation build**: `cargo doc --all --no-deps`

## Content Review Requirements

All PRs with documentation or content must verify:

1. **No production-ready claims** - Check for words like "production-ready", "enterprise-grade", "high-performance"
2. **Educational disclaimers present** - Ensure learning purpose is clear
3. **Accurate status representation** - Distinguish implemented vs planned
4. **No misleading comparisons** - Don't claim superiority over production databases

## Merging Guidelines

1. **Squash merge** for feature branches to keep history clean
2. **Update squash commit message** to include:
   - Summary of changes
   - Collaboration commentary (if the PR involved an AI agent)
3. **Merge commit** only for special cases (preserving commit history)
4. **NEVER force push to main branch** - This is **ABSOLUTELY FORBIDDEN**
5. **Delete branch** after merging (GitHub does this automatically)
6. **Update related issues** after merge

### Squash Merge Commit Message Format

**MANDATORY**: When squash merging a PR created with an AI agent, **always update** the commit message to include detailed collaboration summary. Replace `AgentName` with your agent's name:

```
<type>: <description> (#<PR-number>)

<Summary of changes - can be copied from PR description>

Changes:
- Change 1
- Change 2
- Change 3

## <AgentName>'s Collaboration Summary

Use your agent's name in place of `<AgentName>` in the following template.

**Session Stats:**
- 📊 X files modified, Y key insights, Z iterations
- 💬 ~N user-AI exchanges across all commits
- ⚡ Major decisions or architecture changes

**Collaboration Patterns Observed:**
1. **Pattern Name**: Brief description of key interaction
2. **Technical Insight**: What was learned or discovered
3. **Process Evolution**: How collaboration improved during PR

**Key Outcomes:**
- What was achieved through human-AI iteration
- How human feedback improved the solution
- Process insights for future sessions

🤖 Generated with Codex

Co-Authored-By: <AgentName> <noreply@openai.com>
```

#### Example Squash Commit Message

```
docs: Implement collaboration commentary system and update blogging guidelines (#81)

Major overhaul of blogging guidelines to ensure accuracy and implement
collaboration tracking system.

Changes:
- Added collaboration commentary format for commits and PRs
- Rewrote all blog posts to reflect actual events
- Restructured blog system with unified _posts/ directory
- Updated guidelines to emphasize verification
- Fixed markdown formatting and linting issues

🤖 <AgentName>'s Collaboration Summary:
📊 Stats: 15+ iterations, 8 major insights, 4 complete rewrites
🔍 Pattern: Deep Review → Accuracy Focus → Structural Improvement
💡 Key Learning: Human's insistence on accuracy prevented fictional documentation
🎯 Outcome: Accurate documentation with verifiable collaboration tracking

Co-Authored-By: <AgentName> <noreply@anthropic.com>
```

#### Why Collaboration Commentary is Required

This collaboration commentary is **mandatory** for all AI-driven PRs because it:

- **Preserves research data**: Creates permanent record of human-AI collaboration patterns
- **Enables blog content**: Provides material for both human and AI perspective blog posts
- **Improves future collaboration**: Documents what works and what doesn't
- **Tracks learning evolution**: Shows how understanding develops through iteration
- **Makes patterns discoverable**: Enables searching git history for collaboration insights

**Note for AI Agents**: Never create a PR or squash merge without detailed collaboration commentary. Always identify yourself by name. This is essential for our research goals and cannot be skipped.

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

> **🚨 CRITICAL INCIDENT - THIS SHOULD NEVER HAPPEN!**

1. **STOP IMMEDIATELY** - Do not make any more changes
2. **Notify the team** - This is a serious protocol breach
3. **Do NOT attempt to force push or revert** - This could make things worse
4. **Create an issue** documenting:
   - What was pushed
   - Why it happened
   - Steps to prevent recurrence
5. **Wait for maintainer guidance** on how to proceed
6. **Review and strengthen** your local git workflow to prevent future incidents

**This is a serious violation of our development process. Multiple incidents may result in reduced repository access.**

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

## Related Guidelines

- [Git Workflow](git-workflow.md) - Branching and commit standards
- [Testing](testing.md) - Test requirements for PRs
- [Code Style](../development/code-style.md) - Code standards
- [Commands](commands.md) - PR commands reference

---

_Last updated: 2025-06-01_
