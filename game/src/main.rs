use bevy::prelude::*;

const SPRITE_SIZE: f32 = 100.0;
const MOVE_SPEED: f32 = 500.0;

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

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
            ..default()
        },
        texture,
        ..default()
    });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            transform.translation.y += MOVE_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            transform.translation.y -= MOVE_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            transform.translation.x -= MOVE_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            transform.translation.x += MOVE_SPEED * time.delta_seconds();
        }
    }
}