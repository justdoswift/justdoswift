import { spawn } from "node:child_process";
import { mkdir, readFile, writeFile, stat } from "node:fs/promises";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

const directory = fileURLToPath(new URL(".", import.meta.url));
const destination = resolve(directory, "../public/previews");
const entries = [
  ["PhotoStack", "photo-stack", 86], ["HoldToConfirm", "hold-to-confirm", 158],
  ["MiniPlayer", "mini-player", 125], ["ReactionPicker", "reaction-picker", 110],
  ["SendButton", "send-button", 135], ["PullToSearch", "pull-to-search", 135],
];
function run(command, args) {
  return new Promise((resolveRun, reject) => {
    const child = spawn(command, args, { cwd: directory });
    let output = "";
    child.stdout.on("data", chunk => { output += chunk; });
    child.stderr.on("data", chunk => { output += chunk; });
    child.on("error", reject);
    child.on("exit", code => code === 0 ? resolveRun(output) : reject(new Error(`${command} exited ${code}\n${output.slice(-8000)}`)));
  });
}
await mkdir(destination, { recursive: true });
const results = [];
async function render(entry) {
  const [id, slug, posterFrame] = entry;
  const path = resolve(destination, `${slug}.mp4`);
  console.log(`Rendering ${id}…`);
  await run(process.execPath, [resolve(directory, "node_modules/@remotion/cli/remotion-cli.js"), "render", "src/index.ts", id, path, "--codec=h264", "--crf=20", "--concurrency=2", "--log=error"]);
  await run("ffmpeg", ["-hide_banner", "-loglevel", "error", "-y", "-ss", String(posterFrame / 30), "-i", path, "-frames:v", "1", "-vf", "scale=720:-1", "-c:v", "png", resolve(destination, `${slug}.png`)]);
  const metadata = JSON.parse(await run("ffprobe", ["-v", "error", "-show_streams", "-show_format", "-of", "json", path]));
  const stream = metadata.streams.find(item => item.codec_type === "video");
  if (stream?.width !== 1080 || stream.height !== 1080 || stream.codec_name !== "h264" || !["yuv420p", "yuvj420p"].includes(stream.pix_fmt) || Number(stream.nb_frames) !== 240 || stream.r_frame_rate !== "30/1" || Number(metadata.format.duration) !== 8) {
    throw new Error(`Unexpected output metadata for ${id}`);
  }
  const loopReport = await run("ffmpeg", ["-hide_banner", "-i", path, "-filter_complex", "[0:v]split[a][b];[a]select=eq(n\\,0),setpts=PTS-STARTPTS[first];[b]select=eq(n\\,239),setpts=PTS-STARTPTS[last];[first][last]ssim", "-an", "-f", "null", "-"]);
  const loopSimilarity = Number(loopReport.match(/All:([0-9.]+)/)?.[1]);
  // Compression can introduce tiny differences even for identical source frames.
  if (!(loopSimilarity >= 0.995)) throw new Error(`Loop boundary mismatch for ${id}: ${loopSimilarity}`);
  const size = (await stat(path)).size;
  results.push({ id, slug, width: stream.width, height: stream.height, fps: 30, frames: 240, duration: 8, bytes: size, sha256: createHash("sha256").update(await readFile(path)).digest("hex"), loopSimilarity, kind: "remotion-design-demonstration" });
  console.log(`Verified ${slug}.mp4 · ${(size / 1024).toFixed(0)} KB · 1080 × 1080 · 8s`);
}
// Two independent renders, each bounded to two Chromium tabs.
let cursor = 0;
const errors = [];
await Promise.all(Array.from({ length: 2 }, async () => {
  while (cursor < entries.length) {
    try { await render(entries[cursor++]); }
    catch (error) { errors.push(error); console.error(error.message); }
  }
}));
if (errors.length) throw new AggregateError(errors, "Some compositions failed to render.");
await writeFile(resolve(destination, "manifest.json"), JSON.stringify(results.sort((a,b) => a.slug.localeCompare(b.slug)), null, 2) + "\n");
console.log("Six videos and posters rendered and verified.");
