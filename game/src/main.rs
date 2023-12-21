use bevy::prelude::*;
use std::ops::Div;

const SPRITE_SIZE: f32 = 100.0;
const MOVE_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Farming Roguelike".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            texture,
            ..default()
        },
        Player { speed: MOVE_SPEED },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let mut movement_amount = player.speed * time.delta_seconds();

        if up(&input) && (left(&input) || right(&input))
            || down(&input) && (left(&input) || right(&input))
        {
            movement_amount = player.speed.powf(2.0).div(2.0).sqrt() * time.delta_seconds();
        }

        if up(&input) {
            transform.translation.y += movement_amount;
        }

        if down(&input) {
            transform.translation.y -= movement_amount;
        }

        if left(&input) {
            transform.translation.x -= movement_amount;
        }

        if right(&input) {
            transform.translation.x += movement_amount;
        }
    }
}

fn up(input: &Res<Input<KeyCode>>) -> bool {
    input.pressed(KeyCode::W) || input.pressed(KeyCode::Up)
}

fn down(input: &Res<Input<KeyCode>>) -> bool {
    input.pressed(KeyCode::S) || input.pressed(KeyCode::Down)
}

fn left(input: &Res<Input<KeyCode>>) -> bool {
    input.pressed(KeyCode::A) || input.pressed(KeyCode::Left)
}

fn right(input: &Res<Input<KeyCode>>) -> bool {
    input.pressed(KeyCode::D) || input.pressed(KeyCode::Right)
}
