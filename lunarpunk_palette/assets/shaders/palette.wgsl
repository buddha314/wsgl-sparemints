#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::color_operations::hsv_to_linear_rgb

// Number of palette roles and hue families.
const ROLE_COUNT: f32 = 4.0;
const FAMILY_COUNT: f32 = 12.0;


// Must exactly match the field order/layout of the Rust Palette.
struct Palette {
    background: vec4<f32>,
    body: vec4<f32>,
    primary: vec4<f32>,
    secondary: vec4<f32>,
}


// PaletteMaterial #[uniform(0)]
@group(2) @binding(0)
var<uniform> palette: Palette;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    // Four source-palette roles.
    let row = floor(in.uv.y * ROLE_COUNT);

    // Twelve hue families at 30-degree intervals.
    let column = floor(in.uv.x * FAMILY_COUNT);

    var hsv: vec3<f32>;

    if row < 1.0 {
        hsv = palette.background.xyz;
    } else if row < 2.0 {
        hsv = palette.body.xyz;
    } else if row < 3.0 {
        hsv = palette.primary.xyz;
    } else {
        hsv = palette.secondary.xyz;
    }

    // Rotate the entire source palette rigidly around the hue wheel.
    //
    //  0 =   0 degrees (original palette)
    //  1 =  30 degrees
    //  2 =  60 degrees
    // ...
    // 11 = 330 degrees
    //
    // Hue is normalized to [0, 1], so dividing the column
    // by the family count gives the required rotation.
    let hue_offset = column / FAMILY_COUNT;

    // Preserve saturation and value; rotate only hue.
    // fract() wraps the hue into [0, 1).
    hsv.x = fract(hsv.x + hue_offset);

    // Convert normalized HSV to the linear RGB expected
    // by Bevy's rendering pipeline.
    let rgb = hsv_to_linear_rgb(hsv);

    return vec4<f32>(rgb, 1.0);
}