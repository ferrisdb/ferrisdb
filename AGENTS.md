# FerrisDB AI Contributor Guidelines

Welcome AI contributors! This short guide summarises the most important rules for working in this repository. It complements `CLAUDE.md` and the detailed documents under `guidelines/`.

## Critical Rules

- **Never push directly to `main`.** Always create a feature branch and open a Pull Request.
- **All changes go through a PR**, even typo fixes or documentation updates.
- **Every commit must follow [Conventional Commits](guidelines/workflow/git-workflow.md#commit-message-format)** and include the collaboration commentary section from the same document.
- **Keep commit history clean.** Squash merge feature branches and include a collaboration summary in the final commit message as described in [PR Process](guidelines/workflow/pr-process.md#squash-merge-commit-message-format).

## Required Checks Before Committing

Run these commands from the repository root:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
prettier --write "**/*.md" "**/*.mdx"
```

If you changed files under `docs/`, verify the site builds:

```bash
cd docs && npm run build
```

## Pull Request Checklist

Your PR description should follow the template from [PR Process](guidelines/workflow/pr-process.md#pr-description-template). Include:

1. **Summary** – a short overview of the goal.
2. **Changes Made** – bullet list of modifications.
3. **Why This Matters** – context and motivation.
4. **Testing** – how you verified the change.
5. **Breaking Changes** – note if APIs changed.
6. **🤖 [AgentName]'s Collaboration Summary** – total iterations, key insights and pattern. Replace `AgentName` with your agent's name.

## Quick Links

- [Guidelines Index](guidelines/README.md)
- [Git Workflow](guidelines/workflow/git-workflow.md)
- [Testing Standards](guidelines/workflow/testing.md)
- [Commands](guidelines/workflow/commands.md)
- [Content Strategy](guidelines/content/content-strategy.md)

## Suggested Improvement

The current guidelines are spread across many files. Consider creating a dedicated `guidelines/ai/` directory to house AI-specific references and templates. Keeping all AI instructions in one place will make it easier for multiple agents to stay consistent.

## Collaboration Commentary

All commits by AI agents must include the commentary block described in [Git Workflow](guidelines/workflow/git-workflow.md#claudes-collaboration-commentary). Substitute `Claude` with your own agent name in the header and attribution lines. Always state your agent name in commentary sections and in any messages indicating actions performed on behalf of a human.
