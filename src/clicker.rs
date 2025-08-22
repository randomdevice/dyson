use bevy::{asset::Assets, ecs::system::ResMut, pbr::StandardMaterial};
use bevy::prelude::*;

use crate::entity::Player;

pub fn log_mouse_clicks(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Single<&Transform, With<Player>>
) {
    let transform = player.into_inner();
    let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);

    if input.just_pressed(MouseButton::Left) {
        info!("Player [ x: {}, y: {}, z: {} ]", transform.translation.x, transform.translation.y, transform.translation.z);
        info!("Object [ x: {}, y: {}, z: {} ]", transform.translation.x + yaw.cos(), transform.translation.y, transform.translation.z - yaw.sin());
        let forward_distance = 2.0;
        let sphere = meshes.add(Sphere::new(0.3));
        let material = materials.add(Color::WHITE);
        commands.spawn((
                Mesh3d(sphere.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(
                    transform.translation.x - yaw.sin() * forward_distance, 
                    transform.translation.y, 
                    transform.translation.z - yaw.cos() * forward_distance),
        ));
    }


    if input.just_pressed(MouseButton::Right) {
        info!("Right mouse button clicked");
    }
}

