# FerrisDB Guidelines Update Summary

**Date**: January 2025  
**Purpose**: Realign project guidelines with core value propositions

## Recent Maintenance (2025-05-31)

### Guidelines Cleanup Completed

1. **Removed deprecated files**:
   - Deleted `website-design.md` (replaced by `website-design-starlight.md`)
2. **Fixed broken references**:
   - Updated all references from `website-design.md` → `website-design-starlight.md`
   - Fixed references from non-existent `website-maintenance-starlight.md` → `website-maintenance-simple.md`
3. **Updated technology references**:

   - Changed Jekyll commands → Starlight/Astro commands in `commands.md`
   - Updated Jekyll-specific sections → Starlight/MDX in `markdown-standards.md`
   - Updated migration notes to reflect completed Starlight migration

4. **Added missing documentation**:
   - Created `technical/README.md` index file for technical guidelines

## What Changed (Original Update)

### New Core Philosophy

**From**: Building a database (product focus)  
**To**: Documenting the journey of building a database (process focus)

### Key Principles Added

1. **Journey Over Destination** - The building process is the product
2. **Factual vs. Aspirational** - Clear distinction between built and planned
3. **No Premature Documentation** - Don't document features that don't exist
4. **Tutorial-Driven Learning** - Merge concepts into hands-on tutorials

### Major Structural Changes

#### Content Strategy

- Added [content-strategy.md](content/content-strategy.md) as primary guide
- Added [information-architecture.md](content/information-architecture.md) for structure
- Added [website-maintenance-simple.md](workflow/website-maintenance-simple.md) for focused updates
- Added [starlight-technical-reference.md](workflow/starlight-technical-reference.md) for MDX troubleshooting

#### Content Approach

- **Remove**: Installation, Configuration, API docs (nothing to install/configure)
- **Transform**: Database concepts → Integrated into tutorials
- **Emphasize**: Blog posts, tutorials, current status, journey documentation

### Information Architecture

**Current Structure**:

```
Start Here (Story, Status, Vision)
├── Learn by Building (Tutorials with concepts)
├── The Journey (Blogs, insights, decisions)
├── Deep Dives (Optional technical details)
└── Get Involved (Contributing)
```

## Technology Stack

- **Documentation**: Astro with Starlight theme (migrated from Jekyll)
- **Blog**: starlight-blog plugin
- **Formatting**: Prettier with MDX support
- **Deployment**: GitHub Actions → GitHub Pages

## Why These Changes

1. **Alignment with value props** - We're teaching, not selling
2. **Honesty** - Don't pretend features exist
3. **Engagement** - Story + hands-on learning > dry documentation
4. **Uniqueness** - Human-AI collaboration is our differentiator

## Impact on Existing Work

- Blog posts remain unchanged (already journey-focused)
- Tutorials enhanced to include concepts inline
- Remove speculative documentation
- Add clear status indicators everywhere

## File Format Standards and Patterns

### Core Patterns Established

1. **Magic Number Convention**: `FDB_XXX\0` pattern for all file formats

   - 8-byte identifier at start of every FerrisDB file
   - Null-terminated for binary safety
   - Examples: `FDB_WAL\0`, `FDB_SST\0`, `FDB_MAN\0`

2. **64-Byte Header Standard**:

   - Fixed-size headers for predictable parsing
   - Version field for forward compatibility
   - Metadata fields padded for alignment
   - Reserved bytes for future extensions

3. **CRC32 Checksums**:

   - Data integrity verification for all persistent structures
   - Calculated over entire records/blocks
   - Stored in little-endian format
   - Required for WAL entries, SSTable blocks, manifests

4. **Trait-Based Design**:

   - `Serialize`/`Deserialize` traits for all persistent types
   - Separation of concerns between format and implementation
   - Enables testing and alternative implementations
   - Standard error handling with `FerrisDBError`

5. **Binary Format Documentation**:
   - Every format requires detailed byte-level documentation
   - ASCII art diagrams showing structure
   - Example hex dumps for clarity
   - Version compatibility notes

### Implementation Requirements

- All file formats must be documented in code
- Binary compatibility tests required
- Endianness explicitly specified (little-endian default)
- Version upgrade paths must be considered

## Guidelines Health Status

✅ **Healthy**:

- All file references are valid
- Technology stack references updated
- Consistent navigation structure
- GOVERNANCE.md principles followed
- File format standards established

⚠️ **Needs Attention**:

- Database concepts phase-out timeline needs clarification
- Some guidelines could use standardized headers
- Consider automated link checking in CI

## Next Steps

1. Continue database concepts integration into tutorials
2. Standardize file headers across all guidelines
3. Add automated guideline validation to CI
4. Regular quarterly guideline audits

## Success Metrics

- Readers understand we're building, not selling
- Clear learning path from CRUD dev to database builder
- Human-AI collaboration visible throughout
- No confusion about what exists vs. planned
- Guidelines remain accurate and up-to-date

---

_Guidelines updated collaboratively by Human and Claude to better reflect our educational mission and maintain information architecture integrity_
