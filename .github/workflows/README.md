# GitHub Actions Workflows

This directory contains all GitHub Actions workflows for the FerrisDB project.

## Workflows Overview

### CI (`ci.yml`)

- **Triggers**: Push to main, pull requests, merge groups
- **Purpose**: Run all continuous integration checks
- **Jobs**:
  - Quick checks (formatting, clippy)
  - Markdown linting
  - Spell checking
  - Test suite (multiple OS and Rust versions)
  - Documentation build (Rust docs)
  - MSRV testing (1.81.0)
  - Jekyll site build validation (builds but doesn't deploy)
  - Security audit

### Deploy Docs (`deploy-docs.yml`)

- **Triggers**:
  - Push to main (only when docs/\*\* or workflow changes)
  - Manual workflow dispatch
- **Purpose**: Build and deploy documentation site to GitHub Pages
- **Jobs**:
  - Build Jekyll site with just-the-docs theme
  - Deploy to GitHub Pages using Actions
- **Note**: Requires GitHub Pages source to be set to "GitHub Actions" in repo settings

### PR Review Check (`pr-review-check.yml`)

- **Purpose**: Enforce pull request review requirements

### Release (`release.yml`)

- **Purpose**: Handle release automation

### Security (`security.yml`)

- **Purpose**: Run security-specific checks

## No Conflicts

The workflows are designed to work together without conflicts:

- **CI** validates that the docs build correctly but doesn't deploy
- **Deploy Docs** only runs after changes are merged to main and actually deploys the site
- Both use the same Ruby/Jekyll setup for consistency

## GitHub Pages Configuration

After enabling the Deploy Docs workflow, ensure:

1. Go to Settings → Pages
2. Set Source to "GitHub Actions" (not "Deploy from a branch")
3. Custom domain is already configured (CNAME file exists)
