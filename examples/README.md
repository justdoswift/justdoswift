# SwiftUI interaction studies

Six original interface studies, each distributed as one standalone Swift file.
Requires Swift 6 and iOS 17+. Each file includes its component, a Demo view, and `#Preview`.

| Source | Demo | Important behavior / boundary |
| --- | --- | --- |
| `swiftui/PhotoStack.swift` | `PhotoStackDemo()` | Tap to unfold; swipe to cycle the first three items. Replace the original procedural postcards with your photos. |
| `swiftui/HoldToConfirm.swift` | `HoldToConfirmDemo()` | Early release / pointer drift cancels; one callback per hold; accessible activation does not require holding. |
| `swiftui/MiniPlayer.swift` | `MiniPlayerDemo()` | Expand/collapse and bind playback state. Presentation only: connect your own audio engine. |
| `swiftui/ReactionPicker.swift` | `ReactionPickerDemo()` | Hold, drag, release to select; tap to remove. Named accessibility actions for each option and removal. Reserve space above the control. |
| `swiftui/SendButton.swift` | `SendButtonDemo()` | Async success/failure, retry, duplicate-submit guard, task cancellation. Demo includes a simulated failure switch. |
| `swiftui/PullToSearch.swift` | `PullToSearchDemo()` | Pull or tap to reveal search; clear, cancel, empty results. Host owns filtering / network debounce. |

## Run in Xcode

Create an iOS App using SwiftUI, with an iOS 17 deployment target. Copy the desired file into the project and set its Demo as the root view. It does not require any assets or packages.

To try all six, replace the generated App file with `swiftui/DemoApp.swift` and add the other six files. Keep only one `@main` App in the target.

## Validate from macOS

With Xcode selected:

```sh
npm run check:swift
zsh scripts/check-swift.sh --build
```

The first command type-checks all files with Swift 6 against the installed iOS Simulator SDK and an iOS 17 minimum target. The second builds an ad-hoc-signed Simulator app under `examples/.build/SwiftUIStudies.app` (ignored by Git). No device signing certificate is needed. This script targets Apple Silicon simulators.

Compilation is not a substitute for testing VoiceOver, Dynamic Type, networking, or your production data. These are adaptable examples, not a dependency with a compatibility guarantee.

Simulator spot checks covered photo expansion, player state changes, accessible confirmation/reaction/removal actions, send failure/retry, search filtering/clear/cancel and empty results. Automated pointer dragging did not reliably generate touch gestures in the simulator: the pull gesture and hold/slide paths (including sustained-hold cancellation) still need manual device testing. Accessibility alternatives were exercised, but a full VoiceOver audit was not performed.

## Keep website downloads in sync

Edit the Swift files here, then run `npm run sync:swift`. `npm run lint` checks that website source strings still match the canonical files byte-for-byte.

## Videos

The corresponding `public/previews/*.mp4` files are **Remotion-rendered design demonstrations**, not SwiftUI Simulator recordings. They communicate interaction intent; typography, exact timing, platform controls and layout can differ from the native examples. Each video carries that label, and the website repeats it. The Mini Player clip is silent and does not imply audio playback support.

See `../videos/README.md` for editable compositions and reproduction commands.
