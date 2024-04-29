use bevy::{
    asset::{AssetServer, Assets},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        query::{Changed, With},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Children},
    input::{keyboard::KeyCode, ButtonInput},
    pbr::StandardMaterial,
    prelude::default,
    render::{color::Color, mesh::Mesh},
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, BorderColor, Interaction, JustifyContent, PositionType, Style,
        UiRect, Val,
    },
};

use super::{PhsicaSimulationScheduler, SimulationStatus};

pub fn keyboard_control(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut q: Query<&mut PhsicaSimulationScheduler>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::Space) {
        let mut scheduler = q.single_mut();
        if scheduler.status == SimulationStatus::Running {
            scheduler.parse_scheduler();
        } else if scheduler.status == SimulationStatus::Paused {
            scheduler.resume_scheduler();
        } else if scheduler.status == SimulationStatus::Stopped {
            scheduler.init_scheduler(&mut commands, meshes, materials, true);
        }
    } else if kbd.just_pressed(KeyCode::Escape) {
        let mut scheduler = q.single_mut();
        scheduler.stop_scheduler(&mut commands, meshes);
    } else if kbd.just_pressed(KeyCode::KeyN) {
        let mut scheduler = q.single_mut();
        if scheduler.status == SimulationStatus::Running {
            scheduler.parse_scheduler()
        } else if scheduler.status == SimulationStatus::Paused {
            scheduler.singlestep_scheduler();
        } else if scheduler.status == SimulationStatus::Stopped {
            scheduler.init_scheduler(&mut commands, meshes, materials, false);
            scheduler.singlestep_scheduler();
        }
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut q: Query<&mut PhsicaSimulationScheduler>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                let mut scheduler = q.single_mut();
                if scheduler.status == SimulationStatus::Running {
                    scheduler.parse_scheduler();
                    text.sections[0].value = "Start".to_string();
                } else if scheduler.status == SimulationStatus::Paused {
                    scheduler.resume_scheduler();
                    text.sections[0].value = "Pause".to_string();
                } else if scheduler.status == SimulationStatus::Stopped {
                    scheduler.init_scheduler(&mut commands, meshes, materials, true);
                    text.sections[0].value = "Pause".to_string();
                }
                break;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Percent(1.),
                left: Val::Percent(1.),
                padding: UiRect::all(Val::Px(4.0)),
                // align_items: AlignItems::Center,
                // justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("fonts/FiraCodeNerdFont-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
