# FerrisDB Content Audit Action Plan

Based on our content strategy audit, here's the transformation plan:

## 🗑️ Files to Remove

These files document features that don't exist:

1. **reference/configuration.mdx** - No configuration system exists
2. **reference/architecture.mdx** - Describes unbuilt distributed system
3. **guides/installation.mdx** - Nothing to install (no server/client)
4. **benchmarks.mdx** - No benchmarks exist

## 🔄 Files to Transform

### High Priority (Core Misalignment)

1. **reference/storage-engine.mdx**

   - Keep sections on WAL, MemTable, SSTable Writer (implemented)
   - Remove/mark as planned: Compaction, Block Cache, Bloom Filters, Read Path
   - Add clear status indicators

2. **index.mdx (homepage)**

   - Update progress tabs to show only what's built
   - Remove references to unimplemented features
   - Add prominent "Current Status" section

3. **getting-started.mdx** → **exploring-ferrisdb.mdx**
   - Focus on exploring the existing codebase
   - Remove promises of operations that don't exist
   - Guide through actual code that exists

### Medium Priority (Concepts to Merge)

4. **concepts/database-internals/\*.mdx**

   - Transform into tutorials that explore our actual implementation
   - Move theory into tutorial context
   - Clear "what we built" vs "how databases work in general"

5. **project/roadmap.mdx**
   - Add clear BUILT vs PLANNED sections
   - Show as journey documentation, not promises

### Low Priority (Minor Updates)

6. **project/faq.mdx**
   - Update architecture answers to reflect current state
   - Remove detailed promises

## ✨ New Content to Create

1. **status.mdx** - Clear, honest current state
2. **tutorials/02-exploring-wal.mdx** - Deep dive into our WAL
3. **tutorials/03-understanding-memtable.mdx** - Skip list exploration
4. **vision.mdx** - Move aspirational content here (clearly marked)

## 📝 Implementation Order

### Phase 1: Remove Misleading Content (Immediate)

- Delete configuration.mdx
- Delete architecture.mdx
- Delete installation.mdx
- Delete benchmarks.mdx

### Phase 2: Create Honest Status (Today)

- Create status.mdx
- Update homepage with current reality
- Transform storage-engine.mdx

### Phase 3: Transform Concepts (This Week)

- Merge database concepts into tutorials
- Create new tutorial structure
- Update navigation

### Phase 4: Polish (Next Week)

- Update all cross-references
- Ensure consistent status indicators
- Final review against guidelines

## Success Criteria

After transformation:

- ✅ No documentation for unbuilt features
- ✅ Clear distinction between built and planned
- ✅ Journey-focused narrative throughout
- ✅ Tutorial-driven learning path
- ✅ Honest representation of current state

---

_This plan aligns with our core principle: Document the journey of building, not a product that doesn't exist._
