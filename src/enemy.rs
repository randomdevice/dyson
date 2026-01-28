use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::ball::{Ball, BallBundle};
use crate::entity::{Health, Player, Projectile};

#[derive(Component)]
pub struct Enemy {
    pub move_timer: Timer,
    pub shoot_timer: Timer,
    pub current_direction: Vec3,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
}

pub fn enemy_ai(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Query the enemy
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
    // Query the player position
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation;

    for (mut enemy_transform, mut enemy) in &mut enemy_query {
        // --- 1. RANDOM MOVEMENT ---
        enemy.move_timer.tick(time.delta());
        if enemy.move_timer.just_finished() {
            // Pick a random direction on the XZ plane
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            enemy.current_direction = Vec3::new(angle.cos(), 0.0, angle.sin());
        }

        // Move the enemy
        enemy_transform.translation += enemy.current_direction * time.delta_secs() * 2.0;

        // --- 2. SHOOTING AT PLAYER ---
        enemy.shoot_timer.tick(time.delta());
        if enemy.shoot_timer.just_finished() {
            let spawn_pos = enemy_transform.translation + Vec3::Y * 0.5; // Spawn slightly above

            // Calculate direction vector: (Target - Start) normalized
            let shoot_dir = (player_pos - spawn_pos).normalize();
            let velocity = shoot_dir * 20.0; // Speed of the ball

            // Use the BallBundle from the previous step
            commands.spawn((
                BallBundle {
                    ball: Ball {
                        lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                    },
                    rigid_body: RigidBody::Dynamic,
                    mesh: Mesh3d(meshes.add(Sphere::new(0.3))),
                    material: MeshMaterial3d(materials.add(Color::from(Srgba::RED))),
                    collider: Collider::ball(0.3),
                    restitution: Restitution::coefficient(0.8),
                    friction: Friction::coefficient(5.0),
                    damping: Damping {
                        linear_damping: 0.5,
                        angular_damping: 0.5,
                    },
                    transform: Transform::from_translation(spawn_pos),
                    velocity: Velocity::linear(velocity),
                },
                Projectile { damage: 1 },
            ));
        }
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(EnemyBundle {
        enemy: Enemy {
            move_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            shoot_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
            current_direction: Vec3::ZERO,
        },
        mesh: Mesh3d(meshes.add(Cuboid::from_size(Vec3::ONE))),
        material: MeshMaterial3d(materials.add(Color::from(Srgba::BLUE))),
        transform: Transform::from_xyz(5.0, 0.5, 5.0),
    });
}

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<&Projectile>,
    mut player_query: Query<&mut Health, With<Player>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = collision_event {
            // 1. Try to see if A is projectile and B is player
            // 2. Try to see if B is projectile and A is player
            let hit_data = if let Ok(p) = projectile_query.get(*entity_a) {
                player_query.get_mut(*entity_b).map(|h| (h, p)).ok()
            } else if let Ok(p) = projectile_query.get(*entity_b) {
                player_query.get_mut(*entity_a).map(|h| (h, p)).ok()
            } else {
                None
            };

            // If we found a valid Player + Projectile pair, apply damage
            if let Some((mut health, projectile)) = hit_data {
                health.current -= projectile.damage;
                info!("Player hit! Health remaining: {}", health.current);
            }
        }
    }
}
