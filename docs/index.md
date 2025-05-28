---
layout: home
title: Home
---

## Learning in the Open

<div class="features-grid">
  <div class="feature-card">
    <div class="feature-icon">🦀</div>
    <h3>Rust Systems Programming</h3>
    <p>Learning memory safety, concurrency, and performance optimization through building database components</p>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🏗️</div>
    <h3>Distributed Systems</h3>
    <p>Understanding consensus, replication, partitioning, and fault tolerance in distributed databases</p>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">💾</div>
    <h3>Storage Engines</h3>
    <p>Building LSM-trees, WAL, MemTables, and SSTables from scratch to understand database internals</p>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">🤖</div>
    <h3>AI-Assisted Development</h3>
    <p>Exploring how Claude Code can accelerate learning and development of complex systems</p>
  </div>
</div>

## Architecture Overview {#architecture}

FerrisDB follows a layered architecture with separate components for transactions, storage, and cluster management, heavily inspired by FoundationDB:

- **Transaction Coordinator** - ACID transaction management
- **Storage Servers** - LSM-tree based data storage
- **Cluster Controller** - Membership and coordination
- **Client Library** - Simple key-value API

[Read Full Design Document]({{ '/architecture/' | relative_url }}){: .btn .btn-outline}
[Future Architecture Ideas]({{ '/future-architecture/' | relative_url }}){: .btn .btn-outline}

## Technical Deep Dives {#deep-dives}

Learn database internals through FerrisDB's implementation:

[Understanding WAL and Crash Recovery]({{ '/deep-dive/wal-crash-recovery/' | relative_url }}){: .btn .btn-outline}
[LSM-Trees and Storage Performance]({{ '/deep-dive/lsm-trees/' | relative_url }}){: .btn .btn-outline}

## Development Progress

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
  
  <div class="progress-item in-progress">
    <span class="progress-icon">🚧</span>
    <div class="progress-details">
      <h4>SSTable Implementation</h4>
      <p>Persistent sorted files, compression, bloom filters</p>
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
