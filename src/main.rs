mod web_fullscreen;

use bevy::prelude::*;
use web_fullscreen::FullViewportPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FullViewportPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // spawn cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1., 1., 1.)),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

    // spawn light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });
}
