# Current Work / Handoff

## Current state

The `lunarpunk_palette` Bevy application successfully renders a four-row palette projected into 12 hue families.

The WGSL shader:

- receives a source palette through a material uniform
- selects one of four palette roles by UV row
- selects one of 12 hue families by UV column
- rotates hue by 30-degree increments
- preserves saturation and value
- wraps hue with `fract()`
- converts HSV using Bevy's `hsv_to_linear_rgb()`

The current Lunarpunk source colors are derived from canonical hex values rather than manually entered HSV/HSL values.

## Immediate task

Generalize palette definitions so they are data/assets rather than definitions in `main.rs`.

Support at least:

- Lunarpunk / night
- Solarpunk / day

Keep shader logic palette-agnostic.

## After that

Add runtime selection between day and night palettes.

A simple initial control is sufficient, for example:

- keyboard toggle
- startup configuration
- small Bevy UI control

Do not begin interpolation until both palettes render correctly independently.

## Validation for Solarpunk

When Solarpunk is selected:

1. Column 0 must reproduce the original Solarpunk palette.
2. The remaining 11 columns should be rigid hue rotations.
3. Verify that the multi-hue source palette remains coherent when rotated.
4. Do not assume results will behave like Lunarpunk; Solarpunk has a different hue structure.

## Next experiment

After independent day/night palette selection works, introduce:

`day_amount: f32`

with:

- `0.0` = night/Lunarpunk
- `1.0` = day/Solarpunk

Investigate correct interpolation between the two source palettes.

Hue must be treated as circular.

Compare:

- HSV interpolation
- RGB/linear RGB interpolation
- OKLCH interpolation

Do not assume HSV interpolation is the best-looking transition.

## Keep

Keep the 12-column palette explorer even if the final production palette uses fewer families.

It is useful as a diagnostic/authoring visualization.