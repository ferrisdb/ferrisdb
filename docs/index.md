---
layout: default
title: "Building a Database, Learning in Public"
nav_order: 1
permalink: /
---

{: .fs-9 .text-center }

FerrisDB: Where a CRUD developer and an AI collaborate to build a real database from scratch, documenting every lesson learned along the way.
{: .fs-6 .fw-300 .text-center }

<div class="text-center mb-8">
  <a href="/deep-dive/" class="btn btn-primary fs-5 mb-4 mb-md-0 mr-2">
    <i class="fas fa-graduation-cap mr-2"></i>Start Learning
  </a>
  <a href="/blog/" class="btn btn-outline fs-5 mb-4 mb-md-0 mr-2">
    <i class="fas fa-book-open mr-2"></i>Read Our Story
  </a>
  <a href="https://github.com/ferrisdb/ferrisdb" class="btn fs-5 mb-4 mb-md-0">
    <i class="fab fa-github mr-2"></i>Star on GitHub
  </a>
</div>

<div class="d-flex flex-justify-around flex-wrap mt-6 mb-6">
  <div class="text-center">
    <span class="label label-blue fs-3">🎯 Mission</span>
    <p class="mt-2">Prove that anyone can<br>understand database internals</p>
  </div>
  <div class="text-center">
    <span class="label label-green fs-3">🤝 Approach</span>
    <p class="mt-2">Human creativity + AI knowledge<br>= Better learning</p>
  </div>
  <div class="text-center">
    <span class="label label-purple fs-3">📚 Result</span>
    <p class="mt-2">The most transparent database<br>development ever attempted</p>
  </div>
</div>

---

## Choose Your Learning Path

<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
  <div class="article-card">
    <h3 class="text-purple-300 mt-0">🔍 "I want to understand databases"</h3>
    <p>Explore how databases actually work under the hood, from storage engines to distributed systems.</p>
    <a href="/deep-dive/" class="btn btn-purple btn-sm">Deep Dive Articles →</a>
  </div>
  
  <div class="article-card">
    <h3 class="text-purple-300 mt-0">🦀 "I want to learn Rust"</h3>
    <p>Master Rust through real database code, with comparisons to JavaScript, Python, Java, and Go.</p>
    <a href="/rust-by-example/" class="btn btn-purple btn-sm">Rust by Example →</a>
  </div>
  
  <div class="article-card">
    <h3 class="text-purple-300 mt-0">🏗️ "I want to build with you"</h3>
    <p>Set up FerrisDB locally, run tests, and contribute to an open-source database project.</p>
    <a href="/getting-started/" class="btn btn-purple btn-sm">Getting Started →</a>
  </div>
</div>

## Why We Built This

{: .important }
> **Have you ever wondered how databases really work? We did too.**
>
> As a CRUD developer, I spent years using databases without understanding their magic. Then I partnered with Claude, an AI assistant, to build one from scratch. Not because the world needs another database, but because **learning in public changes everything**.

<div class="d-flex flex-justify-around flex-wrap mt-4 mb-4">
  <div class="p-4 text-center">
    <div class="fs-1">💡</div>
    <strong>Complex systems can be understood</strong><br>
    <span class="text-grey-dk-100">with the right explanations</span>
  </div>
  <div class="p-4 text-center">
    <div class="fs-1">🤝</div>
    <strong>AI amplifies human potential</strong><br>
    <span class="text-grey-dk-100">it doesn't replace developers</span>
  </div>
  <div class="p-4 text-center">
    <div class="fs-1">📖</div>
    <strong>Learning together is better</strong><br>
    <span class="text-grey-dk-100">every mistake becomes a lesson</span>
  </div>
</div>

{: .note-title }
> **Human Developer's Insight**
>
> "Working with Claude showed me that AI isn't here to take our jobs - it's here to help us tackle projects we never thought possible."

## What You'll Learn

<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-6">
  <div class="p-4 border-left border-blue-300 pl-4">
    <h3 class="mt-0">💾 Database Internals</h3>
    <p class="text-grey-dk-100">LSM-trees, WAL, SSTables, MVCC, and distributed consensus - all explained through working code</p>
  </div>
  
  <div class="p-4 border-left border-green-300 pl-4">
    <h3 class="mt-0">🦀 Rust in Practice</h3>
    <p class="text-grey-dk-100">Memory safety, fearless concurrency, and zero-cost abstractions in a real systems project</p>
  </div>
  
  <div class="p-4 border-left border-purple-300 pl-4">
    <h3 class="mt-0">🏗️ System Design</h3>
    <p class="text-grey-dk-100">Architecture decisions, trade-offs, and patterns used in production databases</p>
  </div>
  
  <div class="p-4 border-left border-yellow-300 pl-4">
    <h3 class="mt-0">🤝 AI Collaboration</h3>
    <p class="text-grey-dk-100">How to effectively partner with AI tools to tackle complex engineering challenges</p>
  </div>
</div>

## Development Progress

### Building in Public: Day by Day

{: .fs-6 .fw-500 }

Follow our journey as we build a production-quality database from scratch. Every success, failure, and "aha!" moment documented.

<div class="mt-4">
  <div class="mb-4">
    <div class="d-flex flex-justify-between mb-2">
      <span class="text-green-300 fw-500">✅ Project Foundation</span>
      <span class="text-grey-dk-100">Complete</span>
    </div>
    <div class="progress-bar">
      <div class="progress-fill" style="width: 100%"></div>
    </div>
    <p class="text-grey-dk-100 fs-2 mt-1">Architecture design, Rust workspace setup, development guidelines</p>
  </div>
  
  <div class="mb-4">
    <div class="d-flex flex-justify-between mb-2">
      <span class="text-green-300 fw-500">✅ Storage Engine Foundation</span>
      <span class="text-grey-dk-100">Complete</span>
    </div>
    <div class="progress-bar">
      <div class="progress-fill" style="width: 100%"></div>
    </div>
    <p class="text-grey-dk-100 fs-2 mt-1">Write-Ahead Log, MemTable with concurrent skip list, MVCC support</p>
  </div>
  
  <div class="mb-4">
    <div class="d-flex flex-justify-between mb-2">
      <span class="text-green-300 fw-500">✅ SSTable Implementation</span>
      <span class="text-grey-dk-100">Complete</span>
    </div>
    <div class="progress-bar">
      <div class="progress-fill" style="width: 100%"></div>
    </div>
    <p class="text-grey-dk-100 fs-2 mt-1">Binary format design, writer/reader with binary search, 4KB blocks with checksums</p>
  </div>
  
  <div class="mb-4">
    <div class="d-flex flex-justify-between mb-2">
      <span class="text-yellow-300 fw-500">🚧 Compaction & Optimization</span>
      <span class="text-grey-dk-100">In Progress</span>
    </div>
    <div class="progress-bar">
      <div class="progress-fill" style="width: 40%"></div>
    </div>
    <p class="text-grey-dk-100 fs-2 mt-1">Background compaction, bloom filters, block cache</p>
  </div>
  
  <div class="mb-4">
    <div class="d-flex flex-justify-between mb-2">
      <span class="text-grey-dk-100">⏳ Transaction System</span>
      <span class="text-grey-dk-100">Planned</span>
    </div>
    <div class="progress-bar">
      <div class="progress-fill" style="width: 0%"></div>
    </div>
    <p class="text-grey-dk-100 fs-2 mt-1">MVCC transactions, conflict detection, distributed coordination</p>
  </div>
</div>

## The AI Collaboration Experiment

### 🤖 + 👨‍💻 = Something Special

{: .fs-6 .fw-500 .text-center .mt-6 }

<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
  <div class="article-card bg-blue-000">
    <h4 class="mt-0">Claude's Perspective</h4>
    <blockquote class="mb-4">
      "I've discovered patterns in how humans learn complex systems. My blog documents these insights to help future human-AI teams collaborate better."
    </blockquote>
    <a href="/claude-blog/" class="btn btn-purple btn-sm">Read Claude's Blog →</a>
  </div>
  
  <div class="article-card bg-green-000">
    <h4 class="mt-0">Human's Perspective</h4>
    <blockquote class="mb-4">
      "Claude doesn't just write code - it teaches, explains, and sometimes surprises me with insights I never considered. This is the future of development."
    </blockquote>
    <a href="/blog/" class="btn btn-purple btn-sm">Read Development Blog →</a>
  </div>
</div>

<div class="text-center mt-6 p-4 bg-grey-lt-000 rounded-2">
  <h4 class="mt-0">Latest Collaboration Metrics</h4>
  <div class="d-flex flex-justify-around flex-wrap">
    <div class="m-2">
      <div class="fs-1 fw-700 text-purple-300">47</div>
      <div class="text-grey-dk-100">Pattern recognitions</div>
    </div>
    <div class="m-2">
      <div class="fs-1 fw-700 text-blue-300">12</div>
      <div class="text-grey-dk-100">Human intuition saves</div>
    </div>
    <div class="m-2">
      <div class="fs-1 fw-700 text-green-300">8/10</div>
      <div class="text-grey-dk-100">Collaboration score</div>
    </div>
    <div class="m-2">
      <div class="fs-1 fw-700 text-yellow-300">55+</div>
      <div class="text-grey-dk-100">Tests passing</div>
    </div>
  </div>
</div>

## Educational Resources

<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
  <div>
    <h3>📚 Deep Dive Articles</h3>
    <p class="text-grey-dk-100">In-depth technical articles explaining database concepts through FerrisDB's implementation.</p>

    <ul class="mt-4">
      <li><a href="/deep-dive/wal-crash-recovery/">WAL and Crash Recovery</a> - How databases survive crashes</li>
      <li><a href="/deep-dive/lsm-trees/">LSM-Trees Explained</a> - The secret to fast writes</li>
      <li><a href="/deep-dive/sstable-design/">SSTable Format Design</a> - Efficient on-disk storage</li>
      <li><a href="/deep-dive/concurrent-skip-list/">Lock-Free Skip Lists</a> - Concurrent data structures</li>
    </ul>

    <a href="/deep-dive/" class="btn btn-sm mt-4">View All Deep Dives</a>
  </div>

  <div>
    <h3>🦀 Rust by Example</h3>
    <p class="text-grey-dk-100">Learn Rust through real database code with comparisons to familiar languages.</p>

    <ul class="mt-4">
      <li><a href="/rust-by-example/ownership-memtable-sharing/">Ownership & MemTable Sharing</a> - Rust's killer feature explained</li>
      <li class="text-grey-dk-100">Error Handling in WAL Operations - <em>Coming Soon</em></li>
      <li class="text-grey-dk-100">Concurrent Programming Patterns - <em>Coming Soon</em></li>
      <li class="text-grey-dk-100">Zero-Cost Abstractions - <em>Coming Soon</em></li>
    </ul>

    <a href="/rust-by-example/" class="btn btn-sm mt-4">Start Learning Rust</a>
  </div>
</div>

---

## Join the Journey

{: .text-center .fs-6 .mt-8 }

Whether you're here to learn Rust, understand databases, or explore human-AI collaboration, we have something for you.
{: .text-center .mb-4 }

<div class="text-center">
  <a href="/deep-dive/" class="btn btn-primary fs-5 mr-2">Explore Deep Dives</a>
  <a href="https://github.com/ferrisdb/ferrisdb" class="btn fs-5 mr-2">⭐ Star on GitHub</a>
  <a href="/getting-started/" class="btn btn-outline fs-5">Get Started</a>
</div>

{: .warning }
> **Educational Project**
>
> FerrisDB is designed for learning, not production use. For production databases, consider FoundationDB, TiKV, or CockroachDB.
