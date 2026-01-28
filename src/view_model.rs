use bevy::{
    color::palettes::tailwind, pbr::NotShadowCaster, prelude::*, render::view::RenderLayers,
};
use bevy_rapier3d::prelude::*;

use crate::entity::{Health, HealthBarFill, Player};
use crate::render_layer::VIEW_MODEL_RENDER_LAYER;
use crate::sensitivity::CameraSensitivity;
use crate::world_model::WorldModelCamera;

pub fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    // Health bar assets
    let bar_bg_mesh = meshes.add(Cuboid::new(0.4, 0.05, 0.01));
    let bar_fill_mesh = meshes.add(Cuboid::new(0.4, 0.05, 0.01));
    let bg_mat = materials.add(Color::BLACK.with_alpha(0.5));
    let fill_mat = materials.add(Color::from(tailwind::GREEN_500));

    commands
        .spawn((
            Player,
            Health {
                current: 10,
                max: 10,
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 1.0, 0.5),
            ActiveEvents::COLLISION_EVENTS,
            Transform::from_xyz(0.0, 1.0, 0.0),
            CameraSensitivity::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            // --- WORLD CAMERA ---
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ));

            // --- VIEW MODEL CAMERA (Overlay) ---
            parent.spawn((
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            // --- ARMS ---
            parent.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                NotShadowCaster,
            ));

            // --- HEALTH BAR CONTAINER ---
            // Positioned in the bottom-ish center of the view model camera
            parent
                .spawn((
                    Mesh3d(bar_bg_mesh),
                    MeshMaterial3d(bg_mat),
                    Transform::from_xyz(0.0, -0.2, -0.5),
                    RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                ))
                .with_children(|bar| {
                    // THE ACTUAL FILL
                    bar.spawn((
                        HealthBarFill,
                        Mesh3d(bar_fill_mesh),
                        MeshMaterial3d(fill_mat),
                        // Slightly in front of the background to avoid flickering
                        Transform::from_xyz(0.0, 0.0, 0.01),
                        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                    ));
                });
        });
}

pub fn update_health_bar(
    // 0.16 still uses Changed<T> for efficient updates
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
    // REPLACED: Parent -> ChildOf
    mut fill_query: Query<(&mut Transform, &ChildOf), With<HealthBarFill>>,
) {
    let bar_width = 0.4;

    // We get the player's current health state
    if let Ok(health) = player_query.single() {
        let health_pct = (health.current as f32 / health.max as f32).clamp(0.0, 1.0);

        for (mut fill_transform, _) in &mut fill_query {
            // child_of_component.get() returns the Entity ID of the parent
            // We can use this if we need to verify it belongs to the Player

            fill_transform.scale.x = health_pct;
            // Alignment math to keep the bar left-anchored
            fill_transform.translation.x = (health_pct - 1.0) * (bar_width / 2.0);
        }
    }
}
