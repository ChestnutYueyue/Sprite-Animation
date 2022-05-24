use bevy::prelude::*;

pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
    }
}
