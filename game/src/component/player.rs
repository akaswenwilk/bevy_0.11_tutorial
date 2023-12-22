use bevy::prelude::*;
use std::ops::Div;

#[derive(Component)]
pub struct Player {
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
