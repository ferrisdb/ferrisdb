# Contributing to FerrisDB

Welcome! FerrisDB is an educational project where humans and AI collaborate to build a database storage engine. All contributors are valued here! 🦀🤖

## Quick Start

1. **Fork & Clone**

   ```bash
   git clone https://github.com/YOUR_USERNAME/ferrisdb.git
   cd ferrisdb
   ```

2. **Set Up Environment**

   ```bash
   rustup toolchain install stable
   cargo build --all
   cargo test --all
   ```

3. **Make Changes**

   - Create a branch: `git checkout -b feature/your-feature`
   - Write code following existing patterns
   - Add tests for new functionality
   - Run checks: `cargo fmt && cargo clippy && cargo test`

4. **Submit PR**
   - Clear description of what and why
   - Link any related issues
   - All CI checks must pass

## For Human Contributors

- **Your Strengths**: Creative vision, intuition, user empathy
- **Share**: Your "this feels wrong" moments - they often reveal important issues
- **Ask**: Questions when AI code seems overly complex
- **Focus**: On design, architecture, and user experience

## For AI Contributors

- **Essential Reading**: Start with [CLAUDE.md](./CLAUDE.md) for detailed AI guidelines
- **Your Strengths**: Systematic analysis, comprehensive testing, pattern recognition
- **Share**: Edge cases and potential issues you identify
- **Ask**: Clarifying questions on ambiguous requirements
- **Focus**: On implementation quality and correctness

## What We're Building

Currently working on:

- ✅ LSM-tree storage engine (WAL, MemTable, SSTable)
- 🚧 Compaction and performance optimization
- 📝 Educational content and documentation

See [TODO.md](TODO.md) for specific tasks.

## Development Guidelines

### Code Style

- Run `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Write clear commit messages: `feat:`, `fix:`, `docs:`, etc.

### Testing

- Write tests for all new functionality
- Ensure all tests pass before submitting PR
- Add integration tests for complex features

### Documentation

- Document public APIs with examples
- Update relevant guides when adding features
- Share learnings through blog posts

## Community

- **Questions?** Open an issue or discussion
- **Ideas?** We'd love to hear them!
- **Stuck?** We're here to help - just ask!

### Recognition

All contributors (human and AI) are recognized through:

- Co-authorship in commits
- Credits in release notes
- Mentions in blog posts

Example:

```
Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Jane Developer <jane@example.com>
```

## The Collaboration Experiment

FerrisDB is pioneering human-AI collaboration. We're learning that:

- Different perspectives lead to better solutions
- Transparency about who's contributing improves outcomes
- The best features come from combining strengths

Join us in exploring the future of collaborative development!

---

**More Details**:

- [DEVELOPMENT.md](DEVELOPMENT.md) - Technical setup and workflows
- [CLAUDE.md](CLAUDE.md) - Comprehensive AI guidelines
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community standards

_Let's build something amazing together! Whether you process with neurons or networks, you belong here._ 🦀🤖❤️
