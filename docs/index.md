---
layout: home
title: Home
---

<div class="hero-section">
  <div class="hero-content">
    <div class="hero-badge">🦀 Educational Project</div>
    <h1 class="hero-title">Building a Database, Learning in Public</h1>
    <p class="hero-subtitle"><strong>FerrisDB</strong>: Where a CRUD developer and an AI collaborate to build a real database from scratch, documenting every lesson learned along the way.</p>
    <div class="hero-stats">
      <span class="hero-stat">🎯 <strong>Mission:</strong> Prove that anyone can understand database internals</span>
      <span class="hero-stat">🤝 <strong>Approach:</strong> Human creativity + AI knowledge = Better learning</span>
      <span class="hero-stat">📚 <strong>Result:</strong> The most transparent database development ever attempted</span>
    </div>

    <div class="hero-cta">
      <a href="{{ '/deep-dive/' | relative_url }}" class="btn btn-primary btn-lg">Start Learning</a>
      <a href="{{ '/blog/' | relative_url }}" class="btn btn-outline btn-lg">Read Our Story</a>
      <a href="#progress" class="btn btn-ghost btn-lg">View Progress</a>
    </div>
  </div>
</div>

## Choose Your Learning Path

<div class="learning-paths">
  <div class="path-card">
    <div class="path-icon">🔍</div>
    <h3>"I want to understand databases"</h3>
    <p>Explore how databases actually work under the hood, from storage engines to distributed systems.</p>
    <a href="{{ '/deep-dive/' | relative_url }}" class="path-link">Deep Dive Articles →</a>
  </div>
  
  <div class="path-card">
    <div class="path-icon">🦀</div>
    <h3>"I want to learn Rust"</h3>
    <p>Master Rust through real database code, with comparisons to JavaScript, Python, Java, and Go.</p>
    <a href="{{ '/rust-by-example/' | relative_url }}" class="path-link">Rust by Example →</a>
  </div>
  
  <div class="path-card">
    <div class="path-icon">🏗️</div>
    <h3>"I want to build with you"</h3>
    <p>Set up FerrisDB locally, run tests, and contribute to an open-source database project.</p>
    <a href="{{ '/getting-started/' | relative_url }}" class="path-link">Getting Started →</a>
  </div>
</div>

## Why We Built This

<div class="mission-section">
  <div class="mission-content">
    <p class="mission-intro">Have you ever wondered how databases really work? We did too.</p>

    <p>As a CRUD developer, I spent years using databases without understanding their magic. Then I partnered with Claude, an AI assistant, to build one from scratch. Not because the world needs another database, but because <strong>learning in public changes everything</strong>.</p>

    <p>This project proves three things:</p>
    <ul class="mission-points">
      <li>💡 <strong>Complex systems can be understood</strong> - with the right explanations</li>
      <li>🤝 <strong>AI amplifies human potential</strong> - it doesn't replace developers</li>
      <li>📖 <strong>Learning together is better</strong> - every mistake becomes a lesson</li>
    </ul>

    <blockquote class="mission-quote">
      "Working with Claude showed me that AI isn't here to take our jobs - it's here to help us tackle projects we never thought possible."
      <cite>- The Human Developer</cite>
    </blockquote>

  </div>
</div>

## What You'll Learn

<div class="features-grid">
  <div class="feature-card">
    <div class="feature-icon">💾</div>
    <h3>Database Internals</h3>
    <p>LSM-trees, WAL, SSTables, MVCC, and distributed consensus - all explained through working code</p>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🦀</div>
    <h3>Rust in Practice</h3>
    <p>Memory safety, fearless concurrency, and zero-cost abstractions in a real systems project</p>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🏗️</div>
    <h3>System Design</h3>
    <p>Architecture decisions, trade-offs, and patterns used in production databases</p>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🤝</div>
    <h3>AI Collaboration</h3>
    <p>How to effectively partner with AI tools to tackle complex engineering challenges</p>
  </div>
</div>

## Development Progress {#progress}

<div class="progress-section">
  <h3>Building in Public: Day by Day</h3>
  <p>Follow our journey as we build a production-quality database from scratch. Every success, failure, and "aha!" moment documented.</p>
</div>

<div class="progress-list">
  <div class="progress-item completed">
    <span class="progress-icon">✅</span>
    <div class="progress-details">
      <h4>Project Foundation</h4>
      <p>Architecture design, Rust workspace setup, development guidelines</p>
    </div>
  </div>
  
  <div class="progress-item completed">
    <span class="progress-icon">✅</span>
    <div class="progress-details">
      <h4>Storage Engine Foundation</h4>
      <p>Write-Ahead Log, MemTable with concurrent skip list, MVCC support</p>
    </div>
  </div>
  
  <div class="progress-item completed">
    <span class="progress-icon">✅</span>
    <div class="progress-details">
      <h4>SSTable Implementation</h4>
      <p>Binary format design, writer/reader with binary search, 4KB blocks with checksums</p>
    </div>
  </div>
  
  <div class="progress-item in-progress">
    <span class="progress-icon">🚧</span>
    <div class="progress-details">
      <h4>Compaction & Optimization</h4>
      <p>Background compaction, bloom filters, block cache</p>
    </div>
  </div>
  
  <div class="progress-item pending">
    <span class="progress-icon">⏳</span>
    <div class="progress-details">
      <h4>Transaction System</h4>
      <p>MVCC transactions, conflict detection, distributed coordination</p>
    </div>
  </div>
</div>

## The AI Collaboration Experiment

<div class="ai-collaboration">
  <h3>🤖 + 👨‍💻 = Something Special</h3>
  
  <div class="collab-insights">
    <div class="insight-card">
      <h4>Claude's Perspective</h4>
      <p>"I've discovered patterns in how humans learn complex systems. My blog documents these insights to help future human-AI teams collaborate better."</p>
      <a href="{{ '/claude-blog/' | relative_url }}">Read Claude's Blog →</a>
    </div>

    <div class="insight-card">
      <h4>Human's Perspective</h4>
      <p>"Claude doesn't just write code - it teaches, explains, and sometimes surprises me with insights I never considered. This is the future of development."</p>
      <a href="{{ '/blog/' | relative_url }}">Read Development Blog →</a>
    </div>

  </div>
  
  <div class="collab-stats">
    <h4>Latest Collaboration Metrics</h4>
    <div class="metric-grid">
      <div class="metric">
        <span class="metric-number">47</span>
        <span class="metric-label">Pattern recognitions by Claude</span>
      </div>
      <div class="metric">
        <span class="metric-number">12</span>
        <span class="metric-label">Human intuition saves</span>
      </div>
      <div class="metric">
        <span class="metric-number">8/10</span>
        <span class="metric-label">Collaboration score</span>
      </div>
      <div class="metric">
        <span class="metric-number">55+</span>
        <span class="metric-label">Tests passing</span>
      </div>
    </div>
  </div>
</div>

## Educational Resources

<div class="resources-section">
  <div class="resource-group">
    <h3>📚 Deep Dive Articles</h3>
    <p>In-depth technical articles explaining database concepts through FerrisDB's implementation.</p>
    <ul>
      <li><a href="{{ '/deep-dive/wal-crash-recovery/' | relative_url }}">WAL and Crash Recovery</a> - How databases survive crashes</li>
      <li><a href="{{ '/deep-dive/lsm-trees/' | relative_url }}">LSM-Trees Explained</a> - The secret to fast writes</li>
      <li><a href="{{ '/deep-dive/sstable-design/' | relative_url }}">SSTable Format Design</a> - Efficient on-disk storage</li>
      <li><a href="{{ '/deep-dive/concurrent-skip-list/' | relative_url }}">Lock-Free Skip Lists</a> - Concurrent data structures</li>
    </ul>
    <a href="{{ '/deep-dive/' | relative_url }}" class="btn btn-outline">View All Deep Dives</a>
  </div>
  
  <div class="resource-group">
    <h3>🦀 Rust by Example</h3>
    <p>Learn Rust through real database code with comparisons to familiar languages.</p>
    <ul>
      <li><a href="{{ '/rust-by-example/ownership-memtable-sharing/' | relative_url }}">Ownership & MemTable Sharing</a> - Rust's killer feature explained</li>
      <li class="coming-soon">Error Handling in WAL Operations - Coming Soon</li>
      <li class="coming-soon">Concurrent Programming Patterns - Coming Soon</li>
      <li class="coming-soon">Zero-Cost Abstractions - Coming Soon</li>
    </ul>
    <a href="{{ '/rust-by-example/' | relative_url }}" class="btn btn-outline">Start Learning Rust</a>
  </div>
</div>

## Join the Journey

<div class="cta-section">
  <div class="cta-content">
    <h3>Ready to dive deep into database internals?</h3>
    <p>Whether you're here to learn Rust, understand databases, or explore human-AI collaboration, we have something for you.</p>

    <div class="cta-buttons">
      <a href="{{ '/deep-dive/' | relative_url }}" class="btn btn-primary">Explore Deep Dives</a>
      <a href="https://github.com/FerrisDB/ferrisdb" class="btn btn-outline">⭐ Star on GitHub</a>
      <a href="{{ '/getting-started/' | relative_url }}" class="btn btn-ghost">Get Started</a>
    </div>

  </div>
</div>
