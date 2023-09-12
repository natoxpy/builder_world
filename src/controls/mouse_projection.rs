use bevy::{math::Vec4Swizzles, prelude::*, window::PrimaryWindow};

use crate::world::point::Position;

/// This plugin projects the X, Y position from the screen onto the 3d world and
/// returns the X, Z position on Y intersect. It also provides a snap normalized prosition
/// which means that it turns anything between `0 - 20` to `1`, and anything between `20 - 40` to `2`, and so on.
pub struct ProjectionPlugin;

/// Information about mouse projection 
/// 
/// `normal` The position in chunks of 20, meaning `0 - 20` is `1`, `21 - 40` is `2` etc.
/// 
/// `position` The real position of the mouse on the world at the Y intersect. 
#[derive(Resource, Default, Debug)]
pub struct MouseProjection {
    pub normal: Position,
    pub position: Vec3,
}

impl Plugin for ProjectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseProjection::default());
        app.add_systems(Startup, setup);
        app.add_systems(PostUpdate, mouse_on_y_intersection);
    }
}

/// This is the object at which the mouse is pointing
#[derive(Component, Debug)]
pub struct MousePointObject;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene: Handle<Scene> = asset_server.load("./models/roads/road_prop_concrete.glb#Scene0");

    commands.spawn((SceneBundle { scene, ..default() }, MousePointObject));
}

fn mouse_on_y_intersection(
    camera_q: Query<
        (&Camera, &GlobalTransform, &Transform),
        (With<Camera3d>, Without<MousePointObject>),
    >,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut mouse_point: Query<&mut Transform, With<MousePointObject>>,
    mut mouse_projection: ResMut<MouseProjection>,
) {
    let window = window_q.single();
    let mut mouse_point_transform = mouse_point.single_mut();

    let (camera, c_g_transform, c_transform) = camera_q.single();

    let mouse_pos = if let Some(pos) = window.cursor_position() {
        pos
    } else {
        return;
    };

    let mouse_ndc = Vec3::new(
        (2. * mouse_pos.x) / window.width() - 1.,
        1. - (2. * mouse_pos.y) / window.height(),
        1.,
    );

    let ray_clip = mouse_ndc.extend(1.);

    let mut ray_eye = camera.projection_matrix().inverse() * ray_clip;

    ray_eye = Vec4::new(ray_eye.x, ray_eye.y, -1.0, 0.0);

    let ray_direction = (c_g_transform.compute_matrix() * ray_eye).xyz().normalize();

    let t = -c_transform.translation.y / ray_direction.y;

    let pos = c_transform.translation + t * ray_direction;


    let snap_normal_area = 20.;

    let real_snap_position = Vec3::new(
        (pos.x / snap_normal_area).round() * snap_normal_area,
        0.,
        (pos.z / snap_normal_area).round() * snap_normal_area,
    );

    let normalized_position = Vec3::new((pos.x / snap_normal_area).round(), 0., (pos.z / snap_normal_area).round());

    mouse_projection.normal =
        Position::new(normalized_position.x as i32, normalized_position.z as i32);

    mouse_point_transform.translation = real_snap_position;
}
