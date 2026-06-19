use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
// FIX: Import from hashbrown or standard library (using standard library here works perfectly)
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::collections::HashSet;

use crate::entity::Player;

// --- CONFIGURATION ---
pub const CHUNK_SIZE: f32 = 16.0;
pub const RENDER_DISTANCE: i32 = 2;

#[derive(Resource, Default)]
pub struct SpawnedChunks {
    pub keys: HashSet<(i32, i32)>,
}

pub fn manage_infinite_world(
    mut commands: Commands,
    mut spawned_chunks: ResMut<SpawnedChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // FIX: Replaced `Query<&Transform, With<Player>>` + `.get_single()` with the modern `Single` system parameter wrapper
    player_transform: Single<&Transform, With<Player>>,
) {
    // Determine which chunk coordinate the player is currently standing in
    let player_chunk_x = (player_transform.translation.x / CHUNK_SIZE).floor() as i32;
    let player_chunk_z = (player_transform.translation.z / CHUNK_SIZE).floor() as i32;

    let floor_mesh = meshes.add(Plane3d::default().mesh().size(CHUNK_SIZE, CHUNK_SIZE));
    let floor_material = materials.add(Color::from(Srgba::gray(0.2)));

    for x_offset in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for z_offset in -RENDER_DISTANCE..=RENDER_DISTANCE {
            let chunk_x = player_chunk_x + x_offset;
            let chunk_z = player_chunk_z + z_offset;

            if spawned_chunks.keys.insert((chunk_x, chunk_z)) {
                spawn_chunk(
                    &mut commands,
                    &floor_mesh,
                    &floor_material,
                    &mut meshes,
                    &mut materials,
                    chunk_x,
                    chunk_z,
                );
            }
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    floor_mesh: &Handle<Mesh>,
    floor_material: &Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk_x: i32,
    chunk_z: i32,
) {
    let chunk_center_x = chunk_x as f32 * CHUNK_SIZE;
    let chunk_center_z = chunk_z as f32 * CHUNK_SIZE;

    // --- 1. SPAWN THE COLLIDABLE FLOOR CHUNK ---
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(CHUNK_SIZE / 2.0, 0.05, CHUNK_SIZE / 2.0),
        Mesh3d(floor_mesh.clone()),
        MeshMaterial3d(floor_material.clone()),
        Transform::from_xyz(chunk_center_x, -0.05, chunk_center_z),
    ));

    // Safe zone around world spawn origin (0,0) to prevent trapping the player
    if chunk_x.abs() <= 1 && chunk_z.abs() <= 1 {
        return;
    }

    let mut rng = rand::thread_rng();

    let obstacle_count = rng.gen_range(1..=3);
    for _ in 0..obstacle_count {
        let local_x = rng.gen_range((-CHUNK_SIZE / 2.0)..(CHUNK_SIZE / 2.0));
        let local_z = rng.gen_range((-CHUNK_SIZE / 2.0)..(CHUNK_SIZE / 2.0));

        let pillar_height = rng.gen_range(1.5..5.0);
        let pillar_width = rng.gen_range(0.8..2.2);

        let pillar_mesh = meshes.add(Cuboid::new(pillar_width, pillar_height, pillar_width));
        let pillar_material = materials.add(Color::from(Srgba::gray(0.4)));

        let world_x = chunk_center_x + local_x;
        let world_z = chunk_center_z + local_z;
        let world_y = pillar_height / 2.0;

        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(pillar_width / 2.0, pillar_height / 2.0, pillar_width / 2.0),
            Mesh3d(pillar_mesh),
            MeshMaterial3d(pillar_material),
            Transform::from_xyz(world_x, world_y, world_z),
        ));
    }
}
