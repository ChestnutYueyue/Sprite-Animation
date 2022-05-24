use crate::{resource::sources::Source, role::player::PlayerPlugin};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct RolesPlugin;

impl PluginGroup for RolesPlugin {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Source).add(PlayerPlugin);
    }
}
