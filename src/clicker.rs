use bevy::prelude::*;
use bevy::{asset::Assets, ecs::system::ResMut, pbr::StandardMaterial};
use bevy_rapier3d::prelude::*;

use crate::ball::Ball;
use crate::ball::BallBundle;
use crate::entity::Player;

pub fn log_mouse_clicks(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Single<&Transform, With<Player>>,
) {
    let transform = player.into_inner();
    let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    if input.just_pressed(MouseButton::Left) {
        let forward_distance = 1.0;
        let forward_velocity = 15.0;
        let radius = 0.3;
        //let sphere = meshes.add(Sphere::new(radius));
        //let material = materials.add(Color::WHITE);

        let x_translation = transform.translation.x - yaw.sin() * forward_distance;
        let y_translation = transform.translation.y + pitch.sin() * pitch.cos() * forward_distance;
        let z_translation = transform.translation.z - yaw.cos() * forward_distance;
        //info!("Object [ x: {}, y: {}, z: {} ]", x_translation, y_translation , z_translation);

        let x_velocity = -yaw.sin() * forward_velocity;
        let y_velocity = pitch.sin() * pitch.cos() * forward_velocity;
        let z_velocity = -yaw.cos() * forward_velocity;

        let damping = Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        };

        commands.spawn(BallBundle {
            ball: Ball {
                // Initialize the 5-second timer
                lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            },
            rigid_body: RigidBody::Dynamic,
            mesh: Mesh3d(meshes.add(Sphere::new(radius))),
            material: MeshMaterial3d(materials.add(Color::WHITE)),
            collider: Collider::ball(radius),
            restitution: Restitution::coefficient(0.8),
            friction: Friction::coefficient(5.0),
            damping: damping,
            transform: Transform::from_xyz(x_translation, y_translation, z_translation),
            velocity: Velocity::linear(Vec3::new(x_velocity, y_velocity, z_velocity)),
        });
    }

    if input.just_pressed(MouseButton::Right) {
        info!("Right mouse button clicked");
    }
}

pub fn despawn_balls(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Ball)>,
) {
    for (entity, mut ball) in &mut query {
        // Tick the timer with the elapsed time since the last frame
        ball.lifetime.tick(time.delta());

        if ball.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
