use crate::component::player::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

pub fn spawn_pig(
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

pub fn pig_lifetime(
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
