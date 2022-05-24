#![allow(clippy::redundant_allocation)]
use bevy::{prelude::*, window::PresentMode};
mod window;
mod const_ant;
mod group;
mod resource;
mod role;
use const_ant::constant::{HEIGHT, RESOLUTION};
use group::{windows_group::WindowsGroupPlugin, role_group::RolesPlugin};
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Game".to_string(),
            resizable: false,
            present_mode: PresentMode::Mailbox,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(WindowsGroupPlugin)
        .add_plugins(RolesPlugin)
        .run();
}
