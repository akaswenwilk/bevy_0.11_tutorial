use bevy::{prelude::*, render::camera::ScalingMode};
use std::ops::Div;

const MOVE_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
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
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_pig, pig_lifetime))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
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

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) || money.0 < 10.0 {
        return;
    }

    money.0 -= 10.0;
    info!("Spent $10 on a pig, remaining money: {}", money.0);

    let player_transform = player.single();

    let texture = asset_server.load("pig.png");
    commands.spawn((
        SpriteBundle {
            texture,
            transform: *player_transform,
            ..default()
        },
        Pig {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
    ));
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(pig_entity).despawn();

            info!("Sold pig for $15, remaining money: {}", money.0);
        }
    }
}
