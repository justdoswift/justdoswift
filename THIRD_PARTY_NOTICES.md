# Third-party notices

## beUI Action Swap

The files under `components/motion/` contain an adaptation of the Action Swap component from [starc007/ui-components](https://github.com/starc007/ui-components), distributed under the MIT License.

Copyright © Saurabh Chauhan and contributors.

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

# GPUI experiment

The GPUI command menu uses GPUI Kit / GPUI Base and GPUI (Apache-2.0). Source: https://github.com/longbridge/gpui-kit and https://github.com/zed-industries/zed.

## Accordion / GPUI

`examples/gpui-command/src/accordion.rs` is an original implementation whose structure and visual language follow the [shadcn/ui Accordion](https://ui.shadcn.com/docs/components/base/accordion) documentation (Base UI composition), published under the MIT License. The unstyled primitives come from GPUI Base (Apache-2.0). It bundles the same Geist fonts described below.

The bundled IBM Plex Sans font is from GPUI Kit's gallery and is licensed under SIL Open Font License 1.1. Its license is included in `examples/gpui-command/assets/OFL.txt` and in the downloadable source archive.

## beUI Button / GPUI port

`examples/gpui-command/src/motion_button.rs` and `button_metallic.rs` adapt the button behavior, motion parameters and metallic material from [beUI Button](https://beui.dev/components/motion/button), part of [starc007/ui-components](https://github.com/starc007/ui-components), under the MIT License. The Saurabh Chauhan and contributors copyright and MIT permission notice above apply to this adaptation as well.

The Button example bundles Geist Medium from [vercel/geist-font](https://github.com/vercel/geist-font). Copyright 2024 The Geist Project Authors. It is distributed under SIL Open Font License 1.1; the complete license is in `examples/gpui-command/assets/Geist-OFL.txt` and the source archive.

## beUI Tabs / GPUI port

`examples/gpui-command/src/motion_tabs.rs` adapts the layout, indicator physics,
label clipping and overflow behavior of [beUI Tabs](https://beui.dev/components/motion/tabs),
part of [starc007/ui-components](https://github.com/starc007/ui-components),
under the MIT License. The Saurabh Chauhan and contributors copyright and MIT
permission notice above also apply to this adaptation. It uses the same bundled
Geist font described above, with Regular added for panel text.
