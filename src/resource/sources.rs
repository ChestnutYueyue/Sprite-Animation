use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::role::player::Player;
#[derive(Clone)]
pub struct PlayerSheet {
    pub texture: Handle<TextureAtlas>,
}
#[derive(Default)]
pub struct Source;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AnimatedSprite {
    pub current_frame: usize,
    pub timer: Timer,
}

#[derive(Component, Inspectable)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}
pub struct PlayerAnimations {
    pub walk_down: Vec<usize>,
    pub walk_up: Vec<usize>,
    pub walk_left: Vec<usize>,
    pub walk_right: Vec<usize>,
}

impl Plugin for Source {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_source)
            .add_system(animate_player)
            .add_system(animate_sprites);
    }
}

fn load_source(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("player.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(96.0), 4, 4, Vec2::splat(1.0));
    let character_handle = atlases.add(atlas);
    commands.insert_resource(PlayerSheet {
        texture: character_handle,
    });
    commands.insert_resource(PlayerAnimations {
        walk_down: vec![0, 1, 2, 3],
        walk_left: vec![4, 5, 6, 7],
        walk_right: vec![8, 9, 10, 11],
        walk_up: vec![12, 13, 14, 15],
    });
}

fn animate_sprites(
    mut sprites: Query<&mut AnimatedSprite>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
) {
    for mut sprite in sprites.iter_mut() {
        sprite.timer.tick(time.delta());
        if sprite.timer.just_finished()
            && keyboard.any_pressed([
                KeyCode::W,
                KeyCode::S,
                KeyCode::A,
                KeyCode::D,
                KeyCode::Up,
                KeyCode::Down,
                KeyCode::Left,
                KeyCode::Right,
            ])
        {
            sprite.current_frame += 1;
        } else if sprite.current_frame > 3
            || !keyboard.any_pressed([
                KeyCode::W,
                KeyCode::S,
                KeyCode::A,
                KeyCode::D,
                KeyCode::Up,
                KeyCode::Down,
                KeyCode::Left,
                KeyCode::Right,
            ])
        {
            sprite.current_frame = 0;
        }
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlasSprite, &AnimatedSprite, &Player)>,
    animations: Res<PlayerAnimations>,
) {
    let (mut sprite, animated_sprite, player) = player_query.single_mut();
    //FIXME clean this up
    match player.current_direction {
        FacingDirection::Up => {
            sprite.index =
                animations.walk_up[animated_sprite.current_frame % animations.walk_up.len()];
        }
        FacingDirection::Down => {
            sprite.index =
                animations.walk_down[animated_sprite.current_frame % animations.walk_down.len()];
        }
        FacingDirection::Left => {
            sprite.index =
                animations.walk_left[animated_sprite.current_frame % animations.walk_left.len()];
        }
        FacingDirection::Right => {
            sprite.index =
                animations.walk_right[animated_sprite.current_frame % animations.walk_right.len()];
        }
    }
}
