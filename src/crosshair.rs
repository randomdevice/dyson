use bevy::prelude::*;

pub fn spawn_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            bottom: Val::Percent(50.0),
            ..default()
        })
        .with_child(Text::new(concat!("+")));
}
