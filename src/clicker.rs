use bevy::prelude::{info, Res, ButtonInput, MouseButton};

pub fn log_mouse_clicks(
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Left) {
        info!("Left mouse button clicked");
    }
    if input.just_pressed(MouseButton::Right) {
        info!("Right mouse button clicked");
    }
}
