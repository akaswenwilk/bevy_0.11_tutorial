mod pig;
mod ui;

use crate::{pig::*, ui::*};
use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorOptions;
use std::ops::Div;

const MOVE_SPEED: f32 = 300.0;

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
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_plugins((PigPlugin, GameUI))
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
        Name::new("Player"),
    ));
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

pub fn character_movement(
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
