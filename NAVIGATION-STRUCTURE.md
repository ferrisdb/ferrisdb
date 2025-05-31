# FerrisDB Navigation Structure

## Fixed Navigation Hierarchy

```
FerrisDB Documentation
│
├── Start Here
│   ├── Our Story (index.mdx)
│   ├── Current Status (status.mdx)
│   └── Exploring the Code (exploring-ferrisdb.mdx)
│
├── Learn by Building [TUTORIALS]
│   └── Tutorial 1: Key-Value Store
│
├── The Journey
│   ├── Blog Overview
│   ├── All Posts → /blog
│   ├── 👨‍💻 Human Perspective → /blog/authors/human
│   └── 🤖 AI Perspective → /blog/authors/claude
│
├── Deep Dives [collapsed]
│   ├── Current Implementation
│   ├── Future Architecture
│   ├── Database Concepts
│   │   ├── LSM Trees
│   │   ├── Skip Lists
│   │   ├── SSTables
│   │   └── WAL
│   └── Rust Patterns
│       └── Ownership & Sharing
│
└── Get Involved [collapsed]
    ├── How We Work
    ├── Roadmap
    ├── FAQ
    └── GitHub → external link
```

## Key Changes from Original

### Removed (Non-existent features)

- ❌ Install & Run → Nothing to install
- ❌ First Queries → Can't run queries
- ❌ Architecture Overview → Described unbuilt system
- ❌ Performance Analysis → No benchmarks
- ❌ Storage Engine → Mixed real/aspirational
- ❌ API Reference → No API
- ❌ Configuration → Nothing to configure
- ❌ Guides directory → Empty

### Added (Honest content)

- ✅ Current Status → What actually exists
- ✅ Exploring the Code → How to learn from code
- ✅ Current Implementation → Real component details

### Renamed (Better alignment)

- "Quick Start" → "Start Here" (learning focus)
- "How It Works" → "Deep Dives" (optional depth)
- "Contributing" → "Get Involved" (broader appeal)
- "Database Apprentice" → "Human Perspective"
- "Code Whisperer" → "AI Perspective"

## Navigation Principles

1. **Start with Story** - Hook readers with our unique experiment
2. **Be Honest Early** - Current Status shows reality immediately
3. **Learning Path Clear** - Tutorials are prominently featured
4. **Journey Prominent** - Blog/perspectives easy to find
5. **Deep Content Optional** - Technical details in collapsed sections
6. **No False Promises** - Only link to content that exists

## Success Metrics

- Users understand this is a learning project within 30 seconds
- No confusion about what they can/cannot do with FerrisDB
- Clear path to start learning (tutorials)
- Easy access to the journey story (blog)
- Technical details available but not overwhelming

This navigation structure supports our core value proposition: documenting the journey of building a database, not selling a product.
