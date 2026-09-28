mod palette;

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    sprite_render::{
        Material2d,
        Material2dPlugin,
    },
};

use palette::{lunarpunk, Palette};


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<PaletteMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PaletteMaterial {
    #[uniform(0)]
    palette: Palette,
}


impl Material2d for PaletteMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/palette.wgsl".into()
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PaletteMaterial>>,
) {
    let palette = lunarpunk();

    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(
            meshes.add(Rectangle::new(900.0, 500.0))
        ),
        MeshMaterial2d(
            materials.add(PaletteMaterial { palette })
        ),
    ));
}