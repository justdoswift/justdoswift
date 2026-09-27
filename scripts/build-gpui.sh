#!/usr/bin/env bash
set -euo pipefail
project_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$project_root/examples/gpui-command"
if ! command -v wasm-bindgen >/dev/null || [[ "$(wasm-bindgen --version)" != "wasm-bindgen 0.2.121" ]]; then
  echo 'Install bindings: cargo +nightly-2026-09-25 install wasm-bindgen-cli --version 0.2.121 --locked' >&2
  exit 1
fi
cargo build --locked --lib --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/justdo_command.wasm \
  --out-dir "$project_root/public/gpui-command/pkg" --target web --no-typescript
cd "$project_root"
python3 scripts/package-gpui.py
