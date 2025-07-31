# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal portfolio website built with SvelteKit and deployed to Cloudflare Pages. The project consists of:
- A main website in the `www/` directory
- Code examples in the `code-examples/` directory

## Development Commands

### Frontend (www/)

```bash
# Install dependencies
pnpm install

# Run development server
pnpm dev

# Build for production
pnpm build

# Preview production build
pnpm preview

# Type checking
pnpm check
pnpm check:watch

# Linting and formatting
pnpm lint        # Check formatting
pnpm format      # Auto-format code
```

## Architecture

### Frontend Stack
- **Framework**: SvelteKit with Svelte 5
- **Build Tool**: Vite
- **Styling**: Tailwind CSS v4 with PostCSS
- **Package Manager**: pnpm (v10.14.0)
- **Deployment**: Static adapter for Cloudflare Pages

### Project Structure
```
www/
├── src/
│   ├── routes/          # SvelteKit routes
│   │   ├── +page.svelte # Homepage
│   │   ├── blog/        # Blog section
│   │   └── photos/      # Photos section
│   ├── app.css         # Global styles
│   └── app.html        # HTML template
├── static/
│   └── assets/         # Static images
└── build/              # Production build output
```

### Key Configuration Files

- `svelte.config.js`: SvelteKit configuration with static adapter
- `vite.config.ts`: Vite configuration with enhanced images, Tailwind CSS, and chunking strategy
- `tsconfig.json`: TypeScript configuration

## Deployment

The site is automatically deployed to Cloudflare Pages on push to the `main` branch via GitHub Actions (`.github/workflows/deploy.yaml`).

## Important Notes

- The project uses Tailwind CSS v4 with the new PostCSS-based setup
- Enhanced images are configured for optimized image loading
- Service worker is included for offline functionality
- The build is optimized with Terser minification and manual chunking for better performance