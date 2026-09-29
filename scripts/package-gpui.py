"""Bundle only public source inputs; never package target/, hidden files or secrets."""
from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED
import hashlib
import json

root = Path(__file__).resolve().parent.parent
crate = root / "examples/gpui-command"
output = root / "public/gpui-command"
files = ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "README.md", "src/lib.rs", "src/action_swap.rs", "src/main.rs", "assets/IBMPlexSans-Regular.ttf", "assets/OFL.txt", "assets/swap-copy.svg", "assets/swap-copied.svg", "assets/swap-retry.svg"]
files += ["src/motion_button.rs", "src/button_metallic.rs", "assets/Geist-Medium.ttf", "assets/Geist-OFL.txt"]
files += ["src/motion_tabs.rs", "assets/Geist-Regular.ttf"]
files += ["src/accordion.rs", "src/alert.rs", "src/alert_dialog.rs", "src/aspect_ratio.rs", "src/attachment.rs", "src/avatar.rs", "src/badge.rs", "src/breadcrumb.rs", "src/bubble.rs", "src/button.rs", "src/button_group.rs", "src/calendar.rs", "src/card.rs", "src/carousel.rs", "src/chart.rs", "src/checkbox.rs", "src/collapsible.rs", "src/combobox.rs", "src/command.rs", "src/context_menu.rs", "assets/avatar-1.jpg", "assets/avatar-12.jpg", "assets/avatar-32.jpg", "assets/avatar-47.jpg"]
with ZipFile(output / "source.zip", "w", ZIP_DEFLATED) as archive:
    for file in files:
        archive.write(crate / file, "gpui-command/" + file)
    archive.write(root / "LICENSE", "gpui-command/LICENSE")
    archive.write(root / "THIRD_PARTY_NOTICES.md", "gpui-command/THIRD_PARTY_NOTICES.md")
manifest = {file: hashlib.sha256((crate / file).read_bytes()).hexdigest() for file in files}
for file in sorted((output / "pkg").glob("*")):
    manifest["pkg/" + file.name] = hashlib.sha256(file.read_bytes()).hexdigest()
manifest["source.zip"] = hashlib.sha256((output / "source.zip").read_bytes()).hexdigest()
(output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print("Packaged GPUI source and asset checksums.")
