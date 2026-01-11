use bevy::{
    prelude::*, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef,
};
use bevy::color::Color::Srgba;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;


const SHADER_ASSET_PATH: &str = "shaders\\animate_shader.wgsl";
fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();

}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>
) {
    let cube_posiion = Transform::from_xyz(0.0, 1.0, -2.0);
    commands.spawn(
        (
            PointLight{
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(0.0, 5.0, -2.0),
            )
    );
    commands.spawn(
        (
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        )
    );

    commands.spawn(
        (
            Mesh3d(meshes.add(Cuboid::new(10.0,0.0, 10.))),
                   MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )

    );

    commands.spawn(
        (
            Mesh3d(meshes.add(Cuboid::new(1.0,1.0, 1.0))),
            MeshMaterial3d(custom_materials.add(CustomMaterial {})),
            cube_posiion )

    );
    commands.spawn(
        (
            Mesh3d(meshes.add(Cuboid::new(1.0,0.1, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(-5.5, 0.0, 0.0),
            Name::new("Source")
        )

    );

    commands.spawn(
        (
            Mesh3d(meshes.add(Cuboid::new(1.0,0.1, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(5.5, 0.0, 0.0),
            Name::new("Sink")
        )

    );
    commands.spawn(
        (
            Mesh3d(meshes.add(Cuboid::new(3.0,0.1, 1.0))),
            MeshMaterial3d(custom_materials.add(CustomMaterial {})),
            Transform::from_xyz(5.5, 0.0, 0.0),
            Name::new("Prat")
        )

    );

}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)] struct CustomMaterial {}
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef{
        SHADER_ASSET_PATH.into()
    }
}