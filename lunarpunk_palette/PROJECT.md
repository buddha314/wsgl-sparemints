# WGSL Sparemints — Palette Experiment

## Purpose

This project explores systematic palette expansion for 3D/game environments using Bevy and WGSL.

The original question was:

How can a constrained palette such as Lunarpunk be extended to represent semantic objects such as vegetation, fire, water, wood, etc. without choosing unrelated colors by eye?

The current experiment treats the source palette as a structure in HSV color space and rotates that structure around the hue wheel.

## Current implementation

Active crate:

`lunarpunk_palette/`

Technology:

- Rust
- Bevy 0.19.x
- WGSL
- Bevy `Material2d`
- Bevy color-space utilities

The application renders a rectangular palette explorer.

Rows correspond to semantic source-palette roles.

Columns correspond to hue rotations.

Current dimensions:

- 4 source roles
- 12 hue families
- 48 displayed swatches

## Source palette representation

Palette colors originate as sRGB hex values.

Example Lunarpunk:

- Background: `#0A0A1A`
- Body: `#4A3888`
- Primary: `#C0A8FF`
- Secondary: `#FFFFFF`

The CPU side uses Bevy's color facilities to parse/convert source colors.

The GPU representation currently uses HSV packed into vectors.

WGSL uses Bevy's:

`bevy_render::color_operations::hsv_to_linear_rgb`

to produce the linear RGB expected by the renderer.

## Current transformation

For each source color:

`HSV = (H, S, V)`

For each hue family:

`offset = family / family_count`

Then:

`H' = fract(H + offset)`

while:

`S' = S`
`V' = V`

Thus the entire source palette is rigidly rotated around the hue wheel.

This preserves the source palette's internal saturation/value structure and relative hue offsets.

## Why 12 families?

12 families divide the hue wheel into 30-degree intervals.

This is intentionally an exploration resolution rather than a proposed production palette.

It lets us inspect candidate regions such as:

- warm/fire
- amber/earth
- yellow-green
- vegetation green
- emerald vegetation
- cyan/water
- blue

The final environment may use only a subset.

## Findings so far

The first experiments looked washed out because HSL values from the source page were mistakenly treated as HSV.

The current implementation instead treats the source hex values as canonical and lets Bevy perform color conversion.

This materially improved the generated palette.

The Lunarpunk palette is especially interesting because its chromatic colors occupy a relatively narrow hue region while differing strongly in saturation and value.

Rigid hue rotation therefore produces recognizable new color families while retaining much of the original palette's tonal character.

## Day/night direction

The next architectural goal is environmental palette switching.

Night:

Lunarpunk

Day:

Solarpunk

Named palettes should be external palette data/assets rather than hard-coded application logic.

Runtime state should determine which palette or palette blend is active.

Eventually:

`night palette + day palette + time-of-day parameter -> active environment palette`

Hue interpolation needs circular handling.

## Longer-term questions

1. Which generated hue families should become production semantic families?

2. Should vegetation use one family or several related families?

3. Should fire/emissive colors obey the same rigid rotation rule?

4. How should day/night palettes interpolate?

5. Should palette transformations happen at runtime in WGSL or be authored/precomputed?

6. How does HSV compare with perceptually uniform OKLCH for palette extension?

7. Can the same palette system drive Blender authoring and Bevy runtime materials?

8. Should objects reference semantic palette roles rather than literal colors?

Possible future semantic API:

`vegetation.body`
`vegetation.primary`
`fire.primary`
`water.background`

rather than embedding RGB values in materials.