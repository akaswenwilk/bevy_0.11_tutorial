use bevy::prelude::*;

use crate::Money;

pub struct GameUI;

#[derive(Component)]
pub struct MoneyText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_money_ui);
    }
}

fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Money UI"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "Money!",
                                TextStyle {
                                    font_size: 32.0,
                                    font: Handle::from(asset_server.load("font.ttf")),
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                        MoneyText,
                    ));
                });
        });
}

fn update_money_ui(money: Res<Money>, mut texts: Query<&mut Text, With<MoneyText>>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Money: ${:?}", money.0);
    }
}
