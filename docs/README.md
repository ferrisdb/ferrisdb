# FerrisDB Documentation

[![Built with Starlight](https://astro.badg.es/v2/built-with-starlight/tiny.svg)](https://starlight.astro.build)

This is the documentation site for FerrisDB, built with [Astro Starlight](https://starlight.astro.build/).

## 🚀 Development

### Prerequisites

- Node.js 20+
- npm

### Running locally

```bash
npm install
npm run dev
```

The site will be available at `http://localhost:4321/`

### Building for production

```bash
npm run build
```

The built site will be in `dist/`

## 📁 Project Structure

```
src/
├── components/       # Custom Astro components
├── content/
│   ├── docs/        # Documentation pages
│   └── blog/        # Blog posts
├── styles/          # Custom CSS
└── assets/          # Images and static files
```

## ✏️ Writing Content

### Documentation Pages

Create MDX files in `src/content/docs/`:

```mdx
---
title: Your Page Title
description: Brief description
---

Your content here...
```

### Blog Posts

Create markdown files in `src/content/blog/`:

```markdown
---
title: "Your Blog Post"
date: 2025-01-30
authors: [human] # or [claude]
tags: [rust, databases]
---

Your post content...
```

## 🎨 Customization

- **Colors**: Edit `src/styles/custom.css`
- **Navigation**: Edit sidebar in `astro.config.mjs`
- **Components**: Use Starlight's built-in components

## 🧞 Commands

All commands are run from the root of the project, from a terminal:

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `npm run astro -- --help` | Get help using the Astro CLI                     |

## 👀 Want to learn more?

Check out [Starlight’s docs](https://starlight.astro.build/), read [the Astro documentation](https://docs.astro.build), or jump into the [Astro Discord server](https://astro.build/chat).

## 🚢 Deployment

The site automatically deploys to GitHub Pages when changes are pushed to `main` that affect the `docs/` directory.

## 📚 Resources

- [Starlight Documentation](https://starlight.astro.build/)
- [Astro Documentation](https://docs.astro.build)
- [FerrisDB Repository](https://github.com/ferrisdb/ferrisdb)
