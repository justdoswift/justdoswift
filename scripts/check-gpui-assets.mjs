import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
const root = new URL('../', import.meta.url);
const manifest = JSON.parse(await readFile(new URL('public/gpui-command/manifest.json', root), 'utf8'));
for (const [file, expected] of Object.entries(manifest)) {
  const base = file.startsWith('pkg/') || file === 'source.zip' ? 'public/gpui-command/' : 'examples/gpui-command/';
  const bytes = await readFile(new URL(base + file, root));
  if (createHash('sha256').update(bytes).digest('hex') !== expected) {
    throw new Error(`GPUI asset/source changed: ${file}. Regenerate with npm run build:gpui.`);
  }
}
console.log('GPUI source, WASM assets and source archive match their build manifest.');
