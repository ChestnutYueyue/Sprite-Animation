use bevy::{prelude::*,render::camera::ScalingMode};
use crate::RESOLUTION;

pub struct CameraPlugin;
// 窗口设置插件
impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(camera);
}