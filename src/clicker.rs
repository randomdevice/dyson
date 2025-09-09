use bevy::{asset::Assets, ecs::system::ResMut, pbr::StandardMaterial};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::entity::Player;

pub fn log_mouse_clicks(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Single<&Transform, With<Player>>
) {
    let transform = player.into_inner();
    let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    if input.just_pressed(MouseButton::Left) {
        let forward_distance = 2.0;
        let radius = 0.3;
        let sphere = meshes.add(Sphere::new(radius));
        let material = materials.add(Color::WHITE);

        let x_translation = transform.translation.x - yaw.sin() * forward_distance;
        let y_translation = transform.translation.y + pitch.sin() * pitch.cos() * forward_distance; 
        let z_translation = transform.translation.z - yaw.cos() * forward_distance;
        //info!("Object [ x: {}, y: {}, z: {} ]", x_translation, y_translation , z_translation);

        commands.spawn((
                RigidBody::Dynamic,
                Mesh3d(sphere.clone()),
                MeshMaterial3d(material.clone()),
                Collider::ball(radius),
                Restitution::coefficient(0.7),
                Transform::from_xyz(
                    x_translation, 
                    y_translation, 
                    z_translation),
        ));
    }


    if input.just_pressed(MouseButton::Right) {
        info!("Right mouse button clicked");
    }
}

