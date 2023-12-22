pub mod component;

use bevy::{prelude::*, render::camera::ScalingMode};

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
        .insert_resource(component::player::Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                component::player::character_movement,
                component::pig::spawn_pig,
                component::pig::pig_lifetime,
            ),
        )
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
        component::player::Player { speed: MOVE_SPEED },
    ));
}
