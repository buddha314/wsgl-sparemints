# AGENTS.md

## Project

This repository contains experiments with WGSL shaders, primarily using Bevy.

The active project is:

`lunarpunk_palette/`

It is a Bevy 0.19.x application for exploring algorithmically extended color palettes in WGSL.

## Working rules

Before editing:

1. Inspect the current repository and relevant source files.
2. Treat the checked-out code as authoritative; do not reconstruct files from assumptions.
3. Check the installed Bevy version before using version-sensitive APIs.
4. Prefer Bevy's built-in color and shader utilities over custom implementations.
5. Run `cargo check` after Rust changes.
6. Run or validate shaders when WGSL changes affect pipeline compatibility.
7. Keep changes small and testable.

Do not replace working Bevy APIs with hand-written equivalents without a specific reason.

## Design principles

Keep these layers separate:

- Palette definitions = data/assets.
- Runtime environment/theme state = Bevy resources.
- GPU palette representation = material uniforms.
- Palette transformation/rendering = WGSL.
- Application startup/wiring = `main.rs`.

Do not hard-code individual named palettes in `main.rs`.

The goal is for new palettes to be addable without modifying shader logic.

## Color handling

Canonical palette colors should normally be stored as sRGB hex values such as:

`#C0A8FF`

Use Bevy color facilities for parsing and color-space conversion.

Do not confuse HSL and HSV.

The current shader works in normalized HSV:

- H: 0..1
- S: 0..1
- V: 0..1

Bevy's CPU-side `Hsva` hue is expressed in degrees, so normalize hue at the CPU/GPU boundary when required.

Use Bevy's WGSL color operations, such as `hsv_to_linear_rgb`, instead of implementing color conversion manually.

Remember that shader output is linear RGB.

## Palette model

A source palette currently has four semantic roles:

- background
- body
- primary
- secondary

Avoid assuming that `secondary` is white or a highlight. That happens to be true of some palettes but is not a general rule.

The initial reference palettes are:

### Lunarpunk / night

- background: `#0A0A1A`
- body: `#4A3888`
- primary: `#C0A8FF`
- secondary: `#FFFFFF`

Source:
https://ihatecolors.com/palette/lunarpunk/

### Solarpunk / day

Use the values stored in the project's palette asset as authoritative.

Source:
https://ihatecolors.com/palette/solarpunk/

## Palette extension experiment

The current viewer projects a four-color source palette around the HSV hue wheel.

For family `n`:

`hue_offset = n / family_count`

and:

`H' = fract(H + hue_offset)`

Saturation and value are preserved.

The current exploration view uses 12 families, giving 30-degree hue intervals.

This 12-family grid is an authoring/exploration space, not necessarily the final production palette.

Do not assume production palettes need evenly spaced hue families.

## Important invariant

Family/column 0 must reproduce the original source palette.

If column 0 does not visually match the source palette, investigate color-space conversion, palette loading, uniform layout, or shader output before tuning derived colors.

## Current direction

The palette system is being generalized for environmental day/night use:

- Lunarpunk = night source palette
- Solarpunk = day source palette

Eventually the runtime should be able to transition between these palettes.

Do not implement naive linear interpolation of HSV hue. Hue is circular; interpolation must take the intended path around the hue wheel.

## Code style

Favor clear code over premature abstraction.

Use descriptive shader variables such as:

- `family_count`
- `hue_offset`
- `background`
- `primary`

Keep Rust structs and corresponding WGSL uniform structs in identical field order and compatible layout.

Comment color-space boundaries where they are not obvious.

## Validation

For Rust changes:

`cargo fmt`
`cargo check`

When practical:

`cargo run`

For palette changes, visually verify:

1. Column 0 matches the source palette.
2. White/achromatic colors remain achromatic under hue rotation.
3. Dark source colors remain appropriately dark.
4. Derived families preserve the source palette's saturation/value relationships.