# Migration Plan: Jekyll to Astro Starlight

**Status**: IN PROGRESS  
**Started**: 2025-05-31  
**Target Completion**: 2025-06-07

## Overview

This document tracks the migration of FerrisDB's documentation website from Jekyll to Astro Starlight, including all content, templates, and guidelines.

## Why We're Migrating

1. **Better Developer Experience**: Starlight is designed specifically for technical documentation
2. **MDX Support**: Enables interactive components and better code examples
3. **Modern Stack**: Better performance, type safety, and developer tools
4. **Tutorial Support**: Built-in features for our "Learn by Building" series

## Migration Phases

### Phase 1: Infrastructure Setup ✅ COMPLETE
- [x] Set up Astro Starlight project structure
- [x] Configure navigation and branding
- [x] Implement developer-focused homepage

### Phase 2: Content Migration (IN PROGRESS)
- [x] Migrate blog post templates to MDX
- [x] Create tutorial template and tracking system
- [x] Implement first tutorial
- [ ] Migrate all blog posts
- [ ] Migrate database concept articles
- [ ] Migrate Rust by Example articles

### Phase 3: Guidelines Update (IN PROGRESS)
- [x] Create Starlight-specific guidelines
- [x] Mark old guidelines as deprecated
- [x] Update CLAUDE.md references
- [ ] Update all cross-references
- [ ] Complete update cascade

### Phase 4: Cleanup (PLANNED)
- [ ] Remove deprecated Jekyll files
- [ ] Remove old guideline files
- [ ] Final cross-reference audit
- [ ] Deploy to production

## File Mapping

### Deprecated Files (to be removed after migration)
- `guidelines/content/website-design.md` → `guidelines/content/website-design-starlight.md`
- `guidelines/workflow/website-maintenance.md` → `guidelines/workflow/website-maintenance-starlight.md`
- Jekyll blog templates → MDX templates in `guidelines/content/templates/`

### New Files Added
- `guidelines/content/tutorials.md` - Tutorial creation guidelines
- `guidelines/content/RUST-CONCEPTS-TAUGHT.md` - Concept tracking
- `guidelines/content/DATABASE-CONCEPTS-TAUGHT.md` - Database concept tracking
- `guidelines/content/LEARNING-PROGRESS.md` - Tutorial progress dashboard
- `guidelines/content/templates/*.mdx` - All MDX templates

## Migration Guidelines

### For Active Development
1. Use only Starlight versions of guidelines
2. Create new content in MDX format
3. Follow new tutorial structure for educational content

### For Existing Content
1. Content will be migrated in batches
2. Old content remains accessible during migration
3. URLs will be preserved where possible

## Success Criteria

- [ ] All content migrated and accessible
- [ ] No broken links or references
- [ ] Build and deploy successful
- [ ] Performance metrics improved
- [ ] Developer feedback positive

## Rollback Plan

If critical issues arise:
1. Jekyll site remains deployable
2. Content is version controlled
3. Can revert infrastructure changes

## Notes

- Maintaining both sites during migration per GOVERNANCE.md deprecation process
- Following single source principle - each piece of content has ONE location
- Documenting all changes for future reference

---

_Last updated: 2025-05-31 by Human + Claude_