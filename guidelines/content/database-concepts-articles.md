# Database Concepts Articles Guidelines

Comprehensive technical articles that thoroughly explain database concepts through FerrisDB's implementation. These are the authoritative technical references for understanding database internals.

**Purpose**: Create comprehensive educational articles that explain database concepts using FerrisDB as a concrete example.  
**Prerequisites**: Strong understanding of database systems and technical writing

## Purpose

Database concepts articles serve as the primary educational resource for understanding how databases work "under the hood" using FerrisDB as a concrete implementation example.

## Target Audience

CRUD developers who want to understand database systems deeply but may not have systems programming background. Explain complex concepts accessibly while maintaining technical depth.

## Article Structure (REQUIRED)

Every database concepts article must follow this exact structure:

1. **Problem & Why It Matters**: Fundamental database problem and real-world impact
2. **Conceptual Overview**: Core idea with analogies and visual diagrams
3. **FerrisDB Implementation**: Actual code with detailed explanations and design decisions
4. **Performance Analysis**: Benchmarks, measurements, and trade-off analysis
5. **Advanced Topics**: Cutting-edge concepts and future improvements (optional)
6. **Hands-On Exploration**: Practical exercises and debugging techniques
7. **Real-World Context**: Industry comparison and historical evolution
8. **Common Pitfalls**: Implementation mistakes and best practices
9. **Summary & Takeaways**: Key concepts and when to apply them
10. **Further Reading**: Curated resources and FerrisDB code references

## Content Quality Standards

- **Use actual FerrisDB code**: Always reference real implementation with file paths
- **Include file references**: `// ferrisdb-[crate]/src/[component]/[file].rs:[line-range]`
- **Provide performance data**: Include concrete measurements, not theoretical claims
- **Explain design decisions**: Why FerrisDB chose this approach over alternatives
- **Show visual diagrams**: Use Mermaid diagrams based on ASCII originals in technical guidelines
- **Include hands-on exercises**: Practical ways to explore the concepts
- **Mark unimplemented features**: Use [PLANNED], [CONCEPTUAL], or [FUTURE] tags clearly

### Implementation Status Requirements

**CRITICAL**: Articles often discuss both implemented and planned features. Always be clear:

1. **For Implemented Features**

   ```markdown
   ## Current Implementation

   FerrisDB currently uses a concurrent skip list for the MemTable:
   ```

2. **For Planned Features**

   ```markdown
   ## Future Optimizations [PLANNED]

   > **Note**: This section describes optimizations we plan to implement.

   In future versions, we will add bloom filters to reduce disk reads...
   ```

3. **For Conceptual Discussions**

   ```markdown
   ## Alternative Approaches [CONCEPTUAL]

   > **Note**: This explores approaches FerrisDB might adopt in the future.

   Some databases use B-trees instead of LSM-trees. If we were to...
   ```

## Writing Guidelines

- **Write for CRUD developers**: Assume familiarity with basic programming but not systems programming or advanced CS
- **No PhD required**: Complex concepts should be learnable by someone who builds REST APIs, not just database researchers
- **Define all jargon**: Never use technical terms without clear definitions and examples
- **Use relatable analogies**: Compare database concepts to everyday experiences (filing cabinets, restaurants, traffic)
- **Progressive complexity**: Start with familiar concepts, gradually introduce database-specific ideas
- **Show, don't tell**: Concrete examples rather than abstract descriptions
- **Conversational tone**: Like explaining to a colleague over coffee, not lecturing in a classroom
- **Honest trade-offs**: Don't oversell - acknowledge limitations and alternatives clearly

## Technical Requirements

- **File naming**: `database-concepts/[concept-slug].md`
- **Permalink**: `/database-concepts/[concept-slug]/`
- **Difficulty levels**: Use guidelines below to determine
- **Estimated reading time**: Use calculation method below
- **Prerequisites**: Link to required background articles
- **Code testing**: All code examples must be tested and working
- **Last updated date**: Manually update the date whenever content is modified (don't use dynamic dates)

## Estimated Reading Time Calculation

Use this formula as a baseline:

- **Average reading speed**: 200 words per minute (technical content is slower than regular text)
- **Code comprehension**: Add 30 seconds per code block
- **Diagram analysis**: Add 1 minute per ASCII diagram or visual
- **Exercise completion**: Add time specified in exercise (if hands-on)

Example calculation:

- 3000 words ÷ 200 = 15 minutes
- 10 code blocks × 0.5 = 5 minutes
- 3 diagrams × 1 = 3 minutes
- Total: ~23 minutes → Round to nearest 5 → "25 minutes"

## Difficulty Level Guidelines

### Visual Indicators

- 📗 **Beginner**: Green book emoji
- 📙 **Intermediate**: Orange book emoji
- 📕 **Advanced**: Red book emoji
- ⏱️ **Reading time**: Clock emoji

### Format for Articles

```markdown
📗 **Beginner** • ⏱️ **15 minutes**
```

### Beginner

- Assumes only CRUD development experience
- No systems programming knowledge required
- Concepts explained with everyday analogies
- Code examples use basic Rust syntax only
- Topics: Basic storage concepts, simple algorithms, introductory database ideas

### Intermediate

- Assumes understanding of basic database concepts
- Some familiarity with Rust syntax helpful
- May introduce concurrent programming basics
- Code examples include generics and error handling
- Topics: Concurrency patterns, optimization techniques, complex data structures

### Advanced

- Assumes solid understanding of systems programming concepts
- Comfortable with Rust ownership and lifetimes
- Discusses low-level implementation details
- Code examples include unsafe blocks, advanced traits
- Topics: Lock-free algorithms, memory management, distributed systems

## Performance Analysis Standards

- **Only real measurements**: Use only verified data from actual tests, mathematical proofs, or cited research papers
- **No fictional numbers**: Never make up benchmarks, percentages, or performance claims
- **Benchmark methodology**: When presenting test results, explain exactly how measurements were taken
- **Citation requirements**: Reference research papers or industry sources for external claims
- **Theoretical analysis**: Use mathematical complexity analysis (O notation) when actual measurements aren't available
- **Honest limitations**: If we don't have performance data, explicitly state that rather than inventing numbers

## Visual Guidelines

- **ASCII diagrams**: For architectural overviews and data flow
- **Code structure**: Show before/after for refactoring examples
- **Performance graphs**: When beneficial (prefer numbers in text)
- **Consistent formatting**: Use template spacing and structure

## Topic Examples

### Storage Engine Topics

- LSM Tree fundamentals and FerrisDB's implementation
- Skip list internals and lock-free programming
- SSTable format and binary serialization
- Write-ahead logging and crash recovery
- Compaction strategies and performance trade-offs

### Concurrency Topics

- Lock-free data structures in practice
- Epoch-based memory reclamation
- Atomic operations and memory ordering
- Message passing vs shared memory
- Concurrent testing strategies

### System Design Topics

- Binary format design decisions
- Memory management in database systems
- I/O optimization techniques
- Cache-friendly data structures
- Distributed consensus basics

## Quality Checklist

- [ ] Uses actual FerrisDB code with file references
- [ ] Explains database concepts for non-experts
- [ ] Includes verified performance measurements or mathematical analysis (no fictional numbers)
- [ ] Provides hands-on exercises or exploration
- [ ] References specific FerrisDB implementation decisions
- [ ] Follows template structure exactly
- [ ] All code examples tested and working
- [ ] Includes industry context and alternatives
- [ ] Clear about limitations and trade-offs
- [ ] Links to related articles and resources

## Examples of Quality Standards

### ❌ Poor explanation

"Uses epoch-based reclamation to avoid ABA problems"

### ✅ Good explanation

"Uses epoch-based reclamation to avoid the ABA problem - a concurrency issue where a memory location is changed from A to B and back to A, making it appear unchanged when it actually was modified. In FerrisDB's skip list, this prevents a thread from accessing freed memory that appears valid."

### ❌ Vague performance claim

"This approach is much faster"

### ✅ Concrete measurement

"Binary search reduces comparisons from O(n) to O(log n) complexity. For a 100-item block, this means at most 7 comparisons (⌈log₂ 100⌉) instead of an average of 50 comparisons for linear search."

**Note**: Only include actual benchmark numbers if we have conducted real tests or can cite specific research. Mathematical complexity analysis is always acceptable.

## Template Usage

- **ALWAYS** use the [Database Concept Template](templates/database-concept.mdx)
- Template now uses Starlight MDX format with component imports
- Fill in all sections - never leave template placeholders
- Include all required metadata in frontmatter
- Follow exact section structure for consistency
- Use Starlight components (Card, Tabs, Aside) for enhanced presentation

## Publishing Process

1. Create article using template
2. Include actual performance measurements where possible
3. Test all code examples and exercises
4. Review for technical accuracy with FerrisDB maintainers
5. Format with prettier (`prettier --write "**/*.md"`)
6. Submit PR with "database-concepts" label

## Related Guidelines

- [Visualization](../development/visualization.md) - Creating diagrams
- [Documentation](../development/documentation.md) - Technical writing standards
- [Architecture](../technical/architecture.md) - System design reference
- [Website Design](website-design-starlight.md) - Article layout and styling
