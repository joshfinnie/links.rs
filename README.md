# Josh Finnie Personal Landing Page

A high-performance personal landing page built with Astro and Tailwind CSS.

## Technical Overview

- Framework: Astro 5.0+ (Static Site Generation)
- Styling: Tailwind CSS
- Icons: Astro Icon (SVG-based, zero-client-side JS)
- Architecture: Component-based with a centralized data-driven configuration

## Core Features

- Static Generation: The entire site is pre-rendered at build time for instant loading.
- Zero Runtime JavaScript: No JavaScript is shipped to the end user, ensuring a small payload and high performance.
- Data-Driven UI: Links, social media handles, and brand colors are managed in a single TypeScript file.
- SEO Optimized: Includes Open Graph meta tags and semantic HTML for better visibility and accessibility.

## Installation and Development

### Prerequisites

- Node.js (Latest LTS recommended)
- npm, pnpm, or yarn

### Setup

Clone the repository:

```bash
git clone https://github.com/joshfinnie/links.jfin.us.git
cd links.jfin.us
```

### Install dependencies:

```bash
npm install
```

### Start the development server:

```bash
npm run dev
```

The local environment will be available at http://localhost:4321.

### Configuration

To modify the content of the page, navigate to src/data/links.ts.
This file controls the labels, URLs, icons, and specific brand hex colors for each card.

```typescript
export const personalLinks = [
  {
    label: "joshfinnie.com",
    url: "https://www.joshfinnie.com",
    icon: "mingcute:cloud-line",
    color: "#22d3ee",
  },
];
```

### Deployment

This project is ready for one-click deployment to Vercel, Netlify, or Cloudflare Pages. Because it is a static Astro site, it can also be hosted on any standard web server or GitHub Pages.

## License

MIT Copyright (c) 2026 Josh Finnie
