# Just Do Swift

A desktop component library built with Rust and GPUI. The website currently publishes **Accordion** — a vertically stacked set of interactive headings that reveal sections of content, modeled on [shadcn/ui](https://ui.shadcn.com/docs/components/base/accordion) and built on the unstyled `gpui-base` primitives. Its live WebAssembly preview compiles the same Rust component used by the native example.

- `/gpui/accordion`: Basic, Multiple, Disabled and Card accordions.

The homepage, navigation, search and sitemap use `lib/gpui-components.ts` as the published component list. Earlier component pages are no longer published; their implementation files remain in the repository.

## Development

```sh
npm install
npm run dev
```

Open `http://localhost:3000`.

```sh
npm run dev:gpui:accordion -- card
npm run build:gpui   # regenerate optimized WASM and the source download
npm run check:gpui   # Rust formatting and tests
npm run lint        # source/asset consistency and TypeScript
npm run build       # production website
```

See [the native example guide](examples/gpui-command/README.md) for the pinned Rust toolchain, platform SDKs and integration API. Each published component includes a complete source archive and an explicit command for its native demo. Prebuilt WASM assets are included, so website builds require only Node.js.

## Deployment

The project is linked to Vercel project `justdoswift-web`, serving `https://justdoswift.com`. No application environment variables are required.

After `npm run lint` and `npm run build` pass:

```sh
vercel deploy --prod --yes --scope just-do-swifts-projects
```

`.vercelignore` excludes Rust targets, video projects, old preview assets, dependency folders and local environment files. Keep `public/gpui-command` and `examples/gpui-command/src` in the upload: they provide the WASM runtime, source archive and documentation source panels.

## Credits

Accordion's structure and visual language are modeled on the MIT-licensed [shadcn/ui](https://ui.shadcn.com) documentation (Base UI composition). Earlier Button and Tabs were native GPUI adaptations of MIT-licensed [beUI](https://beui.dev) components. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for source attribution and bundled asset licenses.
