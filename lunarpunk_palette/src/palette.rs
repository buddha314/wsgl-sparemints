use bevy::{
    asset::Asset,
    color::{Hsva, Srgba},
    prelude::*,
    reflect::TypePath,
    render::render_resource::ShaderType,
};

#[derive(Asset, TypePath, Debug, Clone)]
pub struct ColorPalette {
    pub background: String,
    pub body: String,
    pub primary: String,
    pub secondary: String,
}

#[derive(Clone, Copy, Debug, ShaderType)]
pub struct GpuPalette {
    pub background: Vec4,
    pub body: Vec4,
    pub primary: Vec4,
    pub secondary: Vec4,
}

pub type Palette = GpuPalette;

pub fn lunarpunk() -> Palette {
    ColorPalette {
        background: "#0d1021".to_string(),
        body: "#5a4b8a".to_string(),
        primary: "#ff6b6b".to_string(),
        secondary: "#ffd166".to_string(),
    }
    .to_gpu()
}

impl ColorPalette {
    pub fn to_gpu(&self) -> GpuPalette {
        GpuPalette {
            background: hex_to_hsv(&self.background),
            body: hex_to_hsv(&self.body),
            primary: hex_to_hsv(&self.primary),
            secondary: hex_to_hsv(&self.secondary),
        }
    }
}

fn hex_to_hsv(hex: &str) -> Vec4 {
    let srgb = Srgba::hex(hex).expect("Invalid palette color");
    let hsv = Hsva::from(srgb);

    Vec4::new(
        hsv.hue / 360.0,
        hsv.saturation,
        hsv.value,
        hsv.alpha,
    )
}