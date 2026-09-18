# Small interactions, considered individually

Format: six independent 1080 × 1080, 30 fps, 8-second silent H.264 videos.
Shared frame: warm neutral canvas, small component label, centered interface enlarged by 12%, one short interaction cue. The component is the subject; no large promotional headline, redundant kicker or overlay badge. No externally sourced media, fonts, music, or tracking.

| Composition | Character | Timing |
| --- | --- | --- |
| PhotoStack | Tactile travel postcards, sage / blue / terracotta | 0–1.1s stack; 1.1–2.2s unfold; hold to 4.8s; gather by 6.1s |
| HoldToConfirm | Intentional, reassuring, sage green | 1–1.6s cancelled hold; 3.2–4.4s complete hold (1.2s); success to 6.6s; settle by 7.2s |
| MiniPlayer | Warm geometric artwork, continuous expansion | 1.2–2.3s expand; 3.2s play; 4.9s pause; 5.7–7s collapse |
| ReactionPicker | Friendly social card, smoothly weighted emoji emphasis | 1.4s hold; 2s tray; slide to laugh; 4.5s commit; 6.6s tap to remove |
| SendButton | Focused on async feedback, no separate typing sequence | 1.6s send; 3.1s complete; hold result; 5.8–6.3s return to idle |
| PullToSearch | Stable row identities and colors while filtering | 0.8–1.6s pull; reveal by 2.4s; 3–3.9s type/filter; 6–7s cancel and restore |

Every composition returns to its opening state through its own interaction. No full-frame dissolve or duplicate frozen layer: text and cards stay single and sharp at the loop boundary. The render script verifies decoded first/last-frame SSIM ≥ 0.995 (allowing for H.264 compression).
Touch rings illustrate the user's gesture, not a live cursor or native screen capture.
Native implementations and video demonstrations are separate implementations, explicitly labeled.
