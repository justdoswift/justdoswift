#!/bin/zsh
set -euo pipefail
cd "${0:A:h}/.."
swift_sdk="$(xcrun --sdk iphonesimulator --show-sdk-path)"
xcrun swiftc -typecheck -swift-version 6 -sdk "$swift_sdk" -target arm64-apple-ios17.0-simulator examples/swiftui/*.swift
if [[ "${1:-}" == "--build" ]]; then
  mkdir -p examples/.build/SwiftUIStudies.app
  cp examples/Info.plist examples/.build/SwiftUIStudies.app/Info.plist
  xcrun swiftc -parse-as-library -swift-version 6 -sdk "$swift_sdk" -target arm64-apple-ios17.0-simulator examples/swiftui/*.swift -o examples/.build/SwiftUIStudies.app/SwiftUIStudies
  codesign --force --sign - examples/.build/SwiftUIStudies.app
fi
print "Swift 6 / iOS 17 Simulator validation passed."
