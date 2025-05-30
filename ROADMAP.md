# FerrisDB Roadmap

Building a distributed, transactional key-value database from scratch

## 🏗️ Storage Engine Foundation

- [x] Write-Ahead Log (WAL)
- [x] MemTable (Skip List)
- [x] SSTable format
- [x] SSTable reader
- [ ] Compaction
- [ ] Bloom filters
- [ ] Block cache
- [ ] Compression

## 🔄 Basic Operations

- [ ] Get/Put/Delete operations
- [ ] Batch writes
- [ ] Range queries
- [ ] Prefix scans
- [ ] Reverse iteration

## 🎯 ACID Transactions

- [ ] MVCC foundation
- [ ] Snapshot isolation
- [ ] Optimistic concurrency
- [ ] Transaction protocol
- [ ] Conflict detection
- [ ] Serializable isolation

## 🌐 Distribution Layer

- [ ] Raft consensus
- [ ] Data partitioning
- [ ] Leader election
- [ ] Distributed transactions
- [ ] Two-phase commit
- [ ] Auto-rebalancing

## 📊 Performance & Monitoring

- [ ] Metrics collection
- [ ] Performance profiling
- [ ] Query statistics
- [ ] Benchmarks
- [ ] Load testing

## 🔧 Operations & Management

- [ ] CLI client
- [ ] Backup/restore
- [ ] Import/export
- [ ] Configuration
- [ ] Health checks

## 🚀 Client & API

- [ ] Binary protocol
- [ ] Client library
- [ ] Connection pooling
- [ ] Retry logic
- [ ] Client routing

## 🛡️ Production Readiness

- [ ] Error handling
- [ ] Graceful shutdown
- [ ] Resource limits
- [ ] Authentication
- [ ] Chaos testing

## 📚 Documentation & Examples

- [ ] Architecture docs
- [ ] API reference
- [ ] Deployment guide
- [ ] Performance guide
- [ ] Examples

## 🎓 Educational Content

- [ ] Tutorial series
- [ ] Interactive demos
- [ ] Database comparisons
- [ ] Design decisions
- [ ] Common pitfalls
