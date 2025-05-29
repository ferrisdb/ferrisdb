---
layout: default
title: FAQ
nav_order: 6
permalink: /faq/
---

# Frequently Asked Questions
{: .no_toc }

Everything you want to know about FerrisDB and our human-AI collaboration
{: .fs-6 .fw-300 }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## About FerrisDB

### What is FerrisDB?

FerrisDB is an educational distributed database project built in Rust. It's inspired by FoundationDB and implements a real LSM-tree based storage engine, WAL for durability, and eventually will include distributed consensus. The key difference? We're building it completely in public, documenting every decision, mistake, and learning moment.

### Is FerrisDB production-ready?

**No!** FerrisDB is explicitly an educational project. While we're building real database components that work, this is not intended for production use. Think of it as a fully functional teaching tool - it works, but it's optimized for learning, not for your production workload.

### Why build another database?

We're not trying to compete with PostgreSQL, MySQL, or any production database. FerrisDB exists to:

- Demystify how databases actually work
- Show that complex systems can be understood by anyone
- Demonstrate effective human-AI collaboration
- Provide the most transparent database development process ever attempted

### What makes FerrisDB different?

1. **Complete transparency** - Every line of code, every decision, every mistake is documented
2. **Human-AI collaboration** - Built by a CRUD developer and Claude working together
3. **Educational focus** - Optimized for understanding, not just performance
4. **Real implementation** - Not toy examples, but actual working database code

## The Human-AI Collaboration

### Who is building FerrisDB?

FerrisDB is being built by:

- **The Human**: A CRUD developer who spent years using databases without understanding them
- **Claude**: An AI assistant providing knowledge, implementation help, and pattern recognition
- **The Community**: Contributors who help improve code, documentation, and learning materials

### What's Claude's role exactly?

Claude serves multiple roles:

- **Knowledge base**: Explains complex database concepts in understandable terms
- **Implementation partner**: Helps write actual Rust code and debug issues
- **Pattern recognizer**: Identifies common mistakes and learning opportunities
- **Documentation assistant**: Helps create clear, comprehensive documentation
- **Teaching assistant**: Breaks down complex topics for the blog and deep dives

### Does AI replace human developers?

Absolutely not! This project proves the opposite. The human provides:

- Creative vision and project direction
- Domain context and real-world experience
- Decision-making about what to build
- Quality control and code review
- The "why" behind features

Claude amplifies human capabilities but can't replace human judgment, creativity, or domain expertise.

### How do you handle attribution?

We're meticulous about attribution:

- Both blogs document who contributed what
- Git commits clearly show implementation details
- We never swap credit for dramatic effect
- Accurate attribution helps study effective collaboration

## Learning with FerrisDB

### Who is the target audience?

FerrisDB is perfect for:

- **CRUD developers** curious about database internals
- **Rust learners** who want real-world examples
- **Students** studying database systems
- **Anyone** who's wondered "but how does it actually work?"

### Do I need to know Rust?

Not necessarily! We offer multiple learning paths:

- **Deep Dives**: Focus on database concepts (language-agnostic)
- **Rust by Example**: Learn Rust through database code with comparisons to JavaScript, Python, Java, and Go
- **Development Blog**: Follow the journey without diving into code

### How should I use FerrisDB to learn?

1. **Start with Deep Dives** if you want to understand database concepts
2. **Try Rust by Example** if you want to learn Rust through practical code
3. **Follow the blogs** to understand the development process
4. **Run the code locally** to experiment and learn by doing
5. **Contribute** to really solidify your understanding

### What will I learn?

Database concepts:

- How LSM-trees provide fast writes
- Why Write-Ahead Logs ensure durability
- How SSTables organize data on disk
- Lock-free data structures for concurrency
- MVCC for transaction isolation
- Distributed consensus (coming soon)

Rust concepts:

- Ownership and borrowing in practice
- Safe concurrency patterns
- Error handling without exceptions
- Zero-cost abstractions
- Systems programming techniques

## Technical Questions

### Why Rust?

Rust is perfect for databases because:

- **Memory safety** without garbage collection
- **Concurrency** without data races
- **Performance** comparable to C/C++
- **Modern tooling** and ecosystem
- **Learning opportunity** for the human developer

### Why not fork an existing database?

Starting from scratch means:

- Every line of code is understood and documented
- No legacy decisions to work around
- Clear learning progression from simple to complex
- Better teaching opportunity

### What's the architecture?

FerrisDB follows a layered architecture inspired by FoundationDB:

- **Storage Engine**: LSM-tree with MemTable and SSTables
- **Transaction Layer**: MVCC for isolation (in progress)
- **Distribution Layer**: Consensus and sharding (planned)
- **Client Library**: Simple key-value API

### How can I run it?

```bash
# Clone the repository
git clone https://github.com/ferrisdb/ferrisdb
cd ferrisdb

# Run tests
cargo test --all

# Run benchmarks
cargo bench

# Build the project
cargo build --release
```

See our [Getting Started guide](/getting-started/) for detailed instructions.

## Contributing and Community

### Can I contribute?

Yes! We welcome contributions:

- **Code improvements**: Performance, safety, clarity
- **Documentation**: Explanations, examples, corrections
- **Bug reports**: Help us improve
- **Feature ideas**: What should we build next?
- **Learning materials**: Tutorials, exercises, examples

### How do I contribute?

1. Read our [Contributing Guidelines](https://github.com/ferrisdb/ferrisdb/blob/main/CONTRIBUTING.md)
2. Check existing issues or create new ones
3. Fork, branch, implement, test
4. Submit a PR with clear description
5. Iterate based on feedback

### Where can I ask questions?

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and community chat
- **Blog comments**: For specific article discussions

### Is there a roadmap?

Yes! Check our [TODO.md](https://github.com/ferrisdb/ferrisdb/blob/main/TODO.md) for the current roadmap. We update it regularly as we progress.

## The Future

### What's next for FerrisDB?

Immediate priorities:

1. Complete compaction implementation
2. Add bloom filters for read optimization
3. Implement MVCC transactions
4. Build consensus layer
5. Add distribution features

### Will FerrisDB ever be production-ready?

That's not the goal. FerrisDB succeeds if:

- People understand databases better
- Developers feel empowered to tackle complex projects
- Human-AI collaboration patterns improve
- The community learns together

### How long will development continue?

As long as there's something valuable to learn and teach! We're documenting not just building a database, but the entire journey of learning systems programming.

---

## Have another question?

If your question isn't answered here, please:

1. Check our [documentation](/getting-started/)
2. Read the [deep dive articles](/deep-dive/)
3. [Open an issue](https://github.com/ferrisdb/ferrisdb/issues) on GitHub

Remember: every question helps improve this FAQ and helps others learn!
