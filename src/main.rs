use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {}
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
