//! This example showcases a 3D first-person camera.
//!
//! The setup presented here is a very common way of organizing a first-person game
//! where the player can see their own arms. We use two industry terms to differentiate
//! the kinds of models we have:
//!
//! - The *view model* is the model that represents the player's body.
//! - The *world model* is everything else.
//!
//! ## Motivation
//!
//! The reason for this distinction is that these two models should be rendered with different field of views (FOV).
//! The view model is typically designed and animated with a very specific FOV in mind, so it is
//! generally *fixed* and cannot be changed by a player. The world model, on the other hand, should
//! be able to change its FOV to accommodate the player's preferences for the following reasons:
//! - *Accessibility*: How prone is the player to motion sickness? A wider FOV can help.
//! - *Tactical preference*: Does the player want to see more of the battlefield?
//!   Or have a more zoomed-in view for precision aiming?
//! - *Physical considerations*: How well does the in-game FOV match the player's real-world FOV?
//!   Are they sitting in front of a monitor or playing on a TV in the living room? How big is the screen?
//!
//! ## Implementation
//!
//! The `Player` is an entity holding two cameras, one for each model. The view model camera has a fixed
//! FOV of 70 degrees, while the world model camera has a variable FOV that can be changed by the player.
//!
//! We use different `RenderLayers` to select what to render.
//!
//! - The world model camera has no explicit `RenderLayers` component, so it uses the layer 0.
//!   All static objects in the scene are also on layer 0 for the same reason.
//! - The view model camera has a `RenderLayers` component with layer 1, so it only renders objects
//!   explicitly assigned to layer 1. The arm of the player is one such object.
//!   The order of the view model camera is additionally bumped to 1 to ensure it renders on top of the world model.
//! - The light source in the scene must illuminate both the view model and the world model, so it is
//!   assigned to both layers 0 and 1.
//!
//! ## Controls
//!
//! | Key Binding          | Action        |
//! |:---------------------|:--------------|
//! | mouse                | Look around   |
//! | arrow up             | Decrease FOV  |
//! | arrow down           | Increase FOV  |

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
