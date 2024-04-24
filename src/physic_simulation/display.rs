use bevy::{
    app::{App, Plugin, Startup},
    ecs::{
        component::Component,
        query::With,
        schedule,
        system::{Commands, Query},
    },
    hierarchy::BuildChildren,
    prelude::default,
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        BackgroundColor, PositionType, Style, UiRect, Val, ZIndex,
    },
};

use super::PhsicaSimulationScheduler;

#[derive(Component)]
struct PhysicDisplayRoot;

#[derive(Component)]
pub struct PhysicDisplayText;

pub fn setup_display(mut commands: Commands) {
    let root = commands
        .spawn((
            PhysicDisplayRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Auto,
                    top: Val::Percent(1.),
                    bottom: Val::Auto,
                    left: Val::Percent(1.),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_display = commands
        .spawn((
            PhysicDisplayText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "Iteration: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: "\nElapsed: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_display]);
}

pub fn simulation_text_update_system(
    mut query_text: Query<&mut Text, With<PhysicDisplayText>>,
    mut query_scheduler: Query<&mut PhsicaSimulationScheduler>,
) {
    for mut text in &mut query_text {
        let result = query_scheduler.get_single_mut();
        match result {
            Ok(s) => {
                let iteration_cnt = s.iteration_cnt;
                let last_elapsed = s.last_elapsed.as_millis();
                text.sections[1].value = format!("{iteration_cnt:>4.0}");
                text.sections[3].value = format!("{last_elapsed:>4.0} ms");
            }
            Err(_) => {
                text.sections[1].value = " N/A".into();
                text.sections[1].style.color = Color::WHITE;
                text.sections[3].value = " N/A".into();
                text.sections[3].style.color = Color::WHITE;
            }
        }
    }
}
