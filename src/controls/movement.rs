use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

/// Allows for moving the camera around a single pivot point which can also be moved. 
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlData::default());
        app.add_systems(Update, update_controls);
    }
}

#[derive(Resource)]
struct ControlData {
    pivot: Vec3,
    radius: f32,
    polar_angle: f32,
    azimuthal_angle: f32,
    move_speed: f32,
}

impl Default for ControlData {
    fn default() -> Self {
        Self {
            pivot: Vec3::ZERO,
            radius: 200.,
            polar_angle: 2.,
            azimuthal_angle: 1.,
            move_speed: 0.5,
        }
    }
}

fn update_controls(
    mut q: Query<&mut Transform, (With<Camera>, With<Camera3d>)>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut control_config: ResMut<ControlData>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>
) {
    let mut transform = q.single_mut();

    let pvt = control_config.pivot;
    let r = control_config.radius;
    let p_angle = control_config.polar_angle;
    let a_angle = control_config.azimuthal_angle;

    let new_pos = Vec3::new(
        pvt.x + r * a_angle.sin() * p_angle.cos(),
        pvt.y + r * a_angle.cos(),
        pvt.z + r * a_angle.sin() * p_angle.sin(),
    );

    transform.translation = new_pos;
    transform.look_at(pvt, Vec3::Y);

    let speed = control_config.move_speed;

    for ev in scroll_evr.iter() {
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                let mut n = control_config.radius + 2.0 * -ev.y;

                if n >= 250. {
                    n = 250.;
                }
                if n <= 3. {
                    n = 3.;
                }

                control_config.radius = n;
            }
            _ => {}
        }
    }
    for ev in motion_evr.iter() {
        if buttons.pressed(MouseButton::Middle) && !keys.pressed(KeyCode::AltLeft) {
            control_config.polar_angle += ev.delta.x * 0.02 * speed;
            let mut d_azi_a = control_config.azimuthal_angle - ev.delta.y * 0.02 * speed;

            if d_azi_a >= PI {
                d_azi_a = PI - 0.01
            } else if d_azi_a < 0. {
                d_azi_a = 0.01
            }

            control_config.azimuthal_angle = d_azi_a;
        }

        if buttons.pressed(MouseButton::Middle) && keys.pressed(KeyCode::AltLeft) {
            let mut m = transform.rotation * (Vec3::new(ev.delta.x, 0., ev.delta.y) * -speed * 0.3);
            m.y = 0.;

            control_config.pivot += m;
        }
    }    
}
