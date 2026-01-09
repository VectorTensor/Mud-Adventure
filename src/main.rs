use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();

}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
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
            MeshMaterial3d(materials.add(Color::WHITE)),
            cube_posiion
        )

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
}