use bevy::{
    color::palettes::tailwind,
    prelude::*, render::view::RenderLayers,
};
use bevy_rapier3d::prelude::*;
use crate::render_layer::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};

#[derive(Debug, Component)]
pub struct WorldModelCamera;

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);
    let friction = Friction { coefficient: 1.0, combine_rule: CoefficientCombineRule::Max };

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.

    // floor (a y is needed for the collider for a surface to have friction on)
    commands.spawn((
        RigidBody::Fixed,
        Mesh3d(floor),
        Collider::cuboid(100.0, 0.001, 100.0),
        MeshMaterial3d(material.clone()),
        friction
    ));

    commands.spawn((
        RigidBody::Fixed,
        Mesh3d(cube.clone()),
        Collider::cuboid(1.0, 0.25, 0.5),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    // top cube
    commands.spawn((
        RigidBody::Fixed,
        Mesh3d(cube),
        Collider::cuboid(1.0, 0.25, 0.5),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.75, 1.75, 0.0),
    ));

}

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

pub fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Move the camera with your mouse.\n",
            "Press arrow up to decrease the FOV of the world model.\n",
            "Press arrow down to increase the FOV of the world model."
        )));
}

pub fn change_fov(
    input: Res<ButtonInput<KeyCode>>,
    mut world_model_projection: Single<&mut Projection, With<WorldModelCamera>>,
) {
    let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
        unreachable!(
            "The `Projection` component was explicitly built with `Projection::Perspective`"
        );
    };

    if input.pressed(KeyCode::ArrowUp) {
        perspective.fov -= 1.0_f32.to_radians();
        perspective.fov = perspective.fov.max(20.0_f32.to_radians());
    }
    if input.pressed(KeyCode::ArrowDown) {
        perspective.fov += 1.0_f32.to_radians();
        perspective.fov = perspective.fov.min(160.0_f32.to_radians());
    }
}
