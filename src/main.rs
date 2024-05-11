mod hair_simulation;
mod physic_simulation;
mod plugins;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use hair_simulation::HairSimulationPlugin;
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
            // FullViewportPlugin,
            OnScreenFpsPlugin,
            PhysicSimulationPlugin,
            CustomMaterialPlugin,
            HairSimulationPlugin,
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
    let camera_position = Transform {
        translation: Vec3::new(5.0, 5.0, 5.0),
        ..default()
    };

    let camera_target = Vec3::new(0.0, 2.0, 0.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    commands.spawn((
        Camera3dBundle {
            transform: camera_position.looking_at(camera_target, camera_up),
            ..default()
        },
        PanOrbitCamera {
            focus: camera_target,
            radius: Some(3.0),

            ..Default::default()
        },
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });

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
