# Just Do Swift

A curated library of SwiftUI components and motion recipes. The site is built with Next.js, Tailwind CSS, Motion and selected open-source beUI primitives. Its light-first, documentation-oriented layout is inspired by beUI: a persistent component directory, compact gallery, and preview/source workbench.

The included interactive previews are browser approximations, not native SwiftUI recordings. Swift source examples should be compiled and adapted in Xcode before use. Native MP4 recordings can replace the web previews per component.

## Development

```bash
npm install
npm run dev
```

Open `http://localhost:3000`.

Before shipping:

```bash
npm run lint
npm run build
```

## Adding a component

Component content lives in `lib/components.ts`. Add a new item to `swiftComponents` with its metadata, Swift source, usage example and preview variant. The dynamic route under `app/components/[slug]` will generate its detail page and sitemap entry automatically.

Add its implementation notes to `lib/component-notes.ts`. The site includes category filtering, a keyboard-accessible global search (`⌘K` / `Ctrl+K`), light/dark themes, a getting-started guide, and Swift source copy/download controls.

## Adding MP4 previews

1. Export a silent H.264 MP4 from the iOS Simulator.
2. Place it under `public/previews/<slug>.mp4`.
3. Add `video: "/previews/<slug>.mp4"` to the component entry.
4. Optionally add a WebP poster and set `poster: "/previews/<slug>.webp"`.

When a video is present, the existing web preview is replaced by a muted, looping, inline video. Detail pages include playback controls. Autoplay is disabled when the visitor prefers reduced motion.

Recommended capture format: 1080×1080 or 1080×1350, 30 fps, 4–8 seconds, no audio. Keep each catalog preview below roughly 3 MB where practical.

## Deployment

The repository is configured for Vercel with no required environment variables. Pushes to `main` deploy to production after the Vercel Git connection is enabled.

## Credits

The animated copy button is adapted from the MIT-licensed [beUI Action Swap](https://beui.dev/components/motion/action-swap). See `THIRD_PARTY_NOTICES.md`.
