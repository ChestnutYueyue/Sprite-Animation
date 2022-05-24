use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::const_ant::constant::TILE_SIZE;
use crate::resource::sources::{AnimatedSprite, FacingDirection, PlayerAnimations, PlayerSheet};

#[derive(Component, Inspectable)]
pub struct Player {
    pub current_direction: FacingDirection,
    speed: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
    min_time: f32,
    max_time: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(camera_follow_player);
    }
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transforms = player_query.single();
    let mut camera_transforms = camera_query.single_mut();
    camera_transforms.translation = player_transforms.translation;
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    if keyboard.any_pressed([KeyCode::W, KeyCode::Up]) {
        player.current_direction = FacingDirection::Up;
        transform.translation.y += player.speed * time.delta_seconds() * TILE_SIZE;
    }
    if keyboard.any_pressed([KeyCode::S, KeyCode::Down]) {
        player.current_direction = FacingDirection::Down;
        transform.translation.y -= player.speed * time.delta_seconds() * TILE_SIZE;
    }
    if keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
        player.current_direction = FacingDirection::Left;
        transform.translation.x -= player.speed * time.delta_seconds() * TILE_SIZE;
    }
    if keyboard.any_pressed([KeyCode::D, KeyCode::Right]) {
        player.current_direction = FacingDirection::Right;
        transform.translation.x += player.speed * time.delta_seconds() * TILE_SIZE;
    }
}
pub fn spawn_player(
    mut commands: Commands,
    graphics: Res<PlayerSheet>,
    animations: Res<PlayerAnimations>,
) {
    let mut sprite = TextureAtlasSprite::new(animations.walk_down[0]);
    sprite.custom_size = Some(Vec2::splat(0.3));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.texture.clone(),
            transform: Transform {
                translation: Vec3::new(1.0 * TILE_SIZE, 1.0 * TILE_SIZE, 600.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            current_direction: FacingDirection::Down,
            speed: 6.0,
        })
        .insert(AnimatedSprite {
            current_frame: 0,
            timer: Timer::from_seconds(0.15, true),
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(2.0, true),
            min_time: 0.5,
            max_time: 1.5,
        });
}
