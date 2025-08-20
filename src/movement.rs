use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*
};

use std::f32::consts::FRAC_PI_2;

use crate::sensitivity::CameraSensitivity;
use crate::entity::Player;

pub fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn translate_player(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>, 
    ) {
    
    let (mut transform, _) = player.into_inner();

    let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);

    if input.pressed(KeyCode::KeyW) {
        transform.translation.x = transform.translation.x - yaw.sin() * 0.3;
        transform.translation.z = transform.translation.z - yaw.cos() * 0.3;
    }

    if input.pressed(KeyCode::KeyA) {
        transform.translation.x = transform.translation.x - yaw.cos() * 0.3;
        transform.translation.z = transform.translation.z + yaw.sin() * 0.3;
    }

    if input.pressed(KeyCode::KeyD) {
        transform.translation.x = transform.translation.x + yaw.cos() * 0.3;
        transform.translation.z = transform.translation.z - yaw.sin() * 0.3;
    }

    if input.pressed(KeyCode::KeyS) {
        transform.translation.x = transform.translation.x + yaw.sin() * 0.3;
        transform.translation.z = transform.translation.z + yaw.cos() * 0.3;
    }
        
} 
