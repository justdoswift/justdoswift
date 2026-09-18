# Just Do Swift · six interaction studies

Editable, frame-driven Remotion compositions. These are design demonstrations, not native iOS screen recordings. The canonical native source lives in `../examples/swiftui/`.

## Commands

**Install Dependencies**

```console
npm ci
```

**Start Preview**

```console
npm run dev -- --no-open
```

**Render video**

```console
npm run render:all
```

This renders six 1080×1080 H.264 MP4s and PNG posters into `../public/previews/`. The script verifies dimensions, frame count, codec, frame rate, duration and first/last-frame similarity, then writes SHA-256 hashes and loop scores to `manifest.json`. Requires `ffmpeg` and `ffprobe` on PATH. Versions of Remotion packages are pinned in the lockfile.

To render only one composition:

```console
npx remotion render src/index.ts PhotoStack out/photo-stack.mp4
```

Composition IDs: `PhotoStack`, `HoldToConfirm`, `MiniPlayer`, `ReactionPicker`, `SendButton`, `PullToSearch`.

`src/Scene.tsx` contains the quiet shared frame, original SVG artwork and touch indicators. Each study owns its animation and returns naturally to its opening state; there is no full-frame crossfade. Animation is driven exclusively by Remotion frames, with no CSS animation timers or runtime network assets.

Run `npm run lint` before rendering. See `DESIGN.md` for the visual direction and timing plan.

## License

Note that for some entities a company license is needed. [Read the terms here](https://github.com/remotion-dev/remotion/blob/main/LICENSE.md).
