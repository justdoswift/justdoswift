# Just Do Swift

A curated library of production-minded SwiftUI components and motion recipes. The site is built with Next.js, Tailwind CSS, Motion and selected open-source beUI primitives.

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

## Adding MP4 previews

1. Export a silent H.264 MP4 from the iOS Simulator.
2. Place it under `public/previews/<slug>.mp4`.
3. Add `video: "/previews/<slug>.mp4"` to the component entry.
4. Optionally add a WebP poster and set `poster: "/previews/<slug>.webp"`.

When a video is present, the existing live CSS preview is automatically replaced by an autoplaying, muted, looping and inline video.

Recommended capture format: 1080×1080 or 1080×1350, 30 fps, 4–8 seconds, no audio. Keep each catalog preview below roughly 3 MB where practical.

## Deployment

The repository is configured for Vercel with no required environment variables. Pushes to `main` deploy to production after the Vercel Git connection is enabled.

## Credits

The animated copy button is adapted from the MIT-licensed [beUI Action Swap](https://beui.dev/components/motion/action-swap). See `THIRD_PARTY_NOTICES.md`.
