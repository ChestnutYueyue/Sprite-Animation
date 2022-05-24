use bevy::{app::PluginGroupBuilder, prelude::*};
use crate::window::{camera::CameraPlugin, windows::WindowsPlugin};

pub struct WindowsGroupPlugin;

impl PluginGroup for WindowsGroupPlugin {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CameraPlugin).add(WindowsPlugin);
    }
}
