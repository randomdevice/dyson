pub mod ball;
pub mod clicker;
pub mod crosshair;
pub mod enemy;
pub mod entity;
pub mod movement;
pub mod render_layer;
pub mod sensitivity;
pub mod view_model;
pub mod world_model;

use clicker::despawn_balls;
use clicker::log_mouse_clicks;
use crosshair::spawn_crosshair;
use enemy::enemy_ai;
use enemy::handle_collisions;
use enemy::spawn_enemy;
use movement::move_player;
use movement::translate_player;
use view_model::spawn_view_model;
use world_model::change_fov;
use world_model::spawn_lights;
use world_model::spawn_text;
use world_model::spawn_world_model;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::view_model::update_health_bar;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(
            Startup,
            (
                spawn_view_model,
                spawn_world_model,
                spawn_lights,
                spawn_text,
                spawn_crosshair,
                spawn_enemy,
            ),
        )
        .add_systems(
            Update,
            (
                move_player,
                translate_player,
                change_fov,
                despawn_balls,
                log_mouse_clicks,
                enemy_ai,
                handle_collisions,
                update_health_bar,
            ),
        )
        .run();
}
