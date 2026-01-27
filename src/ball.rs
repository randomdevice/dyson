use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub lifetime: Timer,
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub rigid_body: RigidBody,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub collider: Collider,
    pub restitution: Restitution,
    pub friction: Friction,
    pub damping: Damping,
    pub transform: Transform,
    pub velocity: Velocity,
}
