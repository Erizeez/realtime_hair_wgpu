mod physic_simulation;
mod plugins;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use physic_simulation::PhysicSimulationPlugin;
use plugins::{
    instanced_mesh::CustomMaterialPlugin, on_screen_fps::OnScreenFpsPlugin,
    web_fullscreen::FullViewportPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PanOrbitCameraPlugin,
            FullViewportPlugin,
            OnScreenFpsPlugin,
            PhysicSimulationPlugin,
            CustomMaterialPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });

    // spawn light
    // commands
    //     .spawn(PointLightBundle {
    //         point_light: PointLight {
    //             shadows_enabled: true,
    //             intensity: 10_000_000.0,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0., 8., 0.),
    //         ..default()
    //     })
    //     .with_children(|builder| {
    //         builder.spawn(PbrBundle {
    //             mesh: meshes.add(Sphere::new(0.1).mesh().uv(32, 18)),
    //             material: materials.add(StandardMaterial {
    //                 base_color: Color::WHITE,
    //                 emissive: Color::rgba_linear(10.0, 10.0, 10.0, 0.0),
    //                 ..default()
    //             }),
    //             ..default()
    //         });
    //     });

    // directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10_000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(
            -std::f32::consts::FRAC_PI_2 * 0.75,
        )),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });
}
