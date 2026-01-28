use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct HealthBarFill; // Tag for the green part

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
}
