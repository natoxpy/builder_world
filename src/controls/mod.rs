use bevy::prelude::*;

pub mod model_cursor;
pub mod mouse_projection;
pub mod movement;
pub mod place_model;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            movement::MovementPlugin,
            mouse_projection::ProjectionPlugin,
            place_model::PlacePlugin,
        ));
    }
}
