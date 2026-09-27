"""Bake the three fixed labels to SVG paths so GPUI can blur text + icon together.
Optional authoring tool: python -m pip install fonttools; not needed to build/run.
The outlines come from the bundled OFL-licensed IBM Plex Sans at 12 px.
"""
from pathlib import Path
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen

root = Path(__file__).resolve().parent.parent
assets = root / 'examples/gpui-command/assets'
font = TTFont(assets / 'IBMPlexSans-Regular.ttf')
glyphs = font.getGlyphSet()
cmap = font.getBestCmap()
scale = 12 / font['head'].unitsPerEm
icons = {
    'copy': '<rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>',
    'copied': '<path d="m20 6-11 11-5-5"/>',
    'retry': '<path d="M3 11a9 9 0 1 1 3 7M3 4v7h7"/>',
}
for name, label in [('copy', 'Copy code'), ('copied', 'Copied'), ('retry', 'Try again')]:
    width = sum(glyphs[cmap[ord(char)]].width for char in label) * scale
    left = (112 - width - 22) / 2
    paths = []
    cursor = 0
    for char in label:
        glyph = glyphs[cmap[ord(char)]]
        pen = SVGPathPen(glyphs)
        glyph.draw(pen)
        if pen.getCommands():
            paths.append(f'<path transform="translate({cursor} 0)" d="{pen.getCommands()}"/>')
        cursor += glyph.width
    svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 112 42">
<!-- Label outlines: IBM Plex Sans, SIL OFL 1.1 (see OFL.txt). -->
<defs><filter id="soften" x="-30%" y="-100%" width="160%" height="300%" color-interpolation-filters="sRGB"><feGaussianBlur stdDeviation="__BLUR__"/></filter></defs>
<g filter="url(#soften)">
<g transform="translate({left:.4f} 14) scale({14/24:.6f})" fill="none" stroke="black" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">{icons[name]}</g>
<g transform="translate({left+22:.4f} 25.2) scale({scale} {-scale})" fill="black">{''.join(paths)}</g>
</g></svg>
'''
    (assets / f'swap-{name}.svg').write_text(svg)
