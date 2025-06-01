# Technical Guidelines

Technical architecture and implementation guidelines for the FerrisDB project.

**Purpose**: Index of all technical guidelines covering architecture decisions, system design, and implementation standards.

## Foundational Standards

### [Common Standards](common-standards.md)

Essential standards that MUST be followed across all FerrisDB components:

- Binary encoding and endianness
- File format structures
- Timestamp and key formats
- Error handling patterns
- Compression and checksum standards
- Resource limits and naming conventions

## Core Architecture

### [Architecture Guidelines](architecture.md)

Core architectural principles and design decisions for FerrisDB, including:

- High-level system architecture
- Component responsibilities
- Design patterns and principles
- Technology choices and rationale
- Future architecture evolution

### [Storage Engine Guidelines](storage-engine.md)

Detailed guidelines for implementing FerrisDB's storage engine components:

- Write-Ahead Log (WAL) design
- MemTable implementation with Skip List
- SSTable format and operations
- Compaction strategies
- Key encoding and ordering

### [Magic Numbers Standard](magic-numbers.md)

Standard for file signatures and magic numbers across FerrisDB:

- Magic number conventions and format
- Component-specific magic numbers
- Implementation guidelines
- Migration strategies
- Security considerations

## Quality Attributes

### [Performance Guidelines](performance.md)

Standards and best practices for performance optimization:

- Benchmarking requirements
- Performance targets by component
- Optimization techniques
- Memory and CPU efficiency
- Concurrent operation guidelines

### [Security Guidelines](security.md)

Security practices and requirements for FerrisDB:

- Input validation standards
- Resource limit enforcement
- Secure coding practices
- Dependency security
- Future security considerations

### [System Invariants](invariants.md)

Critical system properties that must be maintained:

- Data consistency guarantees
- Ordering requirements
- Concurrency invariants
- Error handling principles
- System state transitions

## Navigation

- **For New Contributors**: Start with [Architecture](architecture.md) → [Storage Engine](storage-engine.md)
- **For Implementation**: Review [Invariants](invariants.md) → Component-specific guidelines
- **For Optimization**: See [Performance](performance.md) guidelines
- **For Security Review**: Check [Security](security.md) practices

## Related Sections

- [Development Standards](../development/) - Code style and documentation
- [Testing Guidelines](../workflow/testing.md) - Quality assurance
- [Tutorial Implementation](../content/tutorials.md) - Educational component building
