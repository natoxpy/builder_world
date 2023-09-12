use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{controls::mouse_projection::MousePointObject, world::point::Point};

use super::{model_cursor::ModelCursor, mouse_projection::MouseProjection};

#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub enum Orientation {
    North(f32),
    South(f32),
    East(f32),
    West(f32),
}

impl Orientation {
    pub fn len() -> usize {
        4
    }

    pub fn index(i: usize) -> Self {
        match i {
            0 => Self::North((0. as f32).to_radians()),
            1 => Self::East((90. as f32).to_radians()),
            2 => Self::South((180. as f32).to_radians()),
            3 => Self::West((270. as f32).to_radians()),
            _ => panic!("no orientation index"),
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            Self::North(_) => 0,
            Self::East(_) => 1,
            Self::South(_) => 2,
            Self::West(_) => 3,
        }
    }

    pub fn next(&mut self) {
        let i = (self.get_index() + 1) % 4;
        *self = Orientation::index(i);
    }

    pub fn previous(&mut self) {
        let mut i = self.get_index();

        if i == 0 {
            i = 4;
        }

        *self = Orientation::index(i - 1)
    }

    pub fn rotation(&self) -> f32 {
        match self {
            Self::North(o) => o.clone(),
            Self::East(o) => o.clone(),
            Self::South(o) => o.clone(),
            Self::West(o) => o.clone(),
        }
    }

    pub fn get_rotation(i: usize) -> f32 {
        match Self::index(i) {
            Self::North(o) => o,
            Self::East(o) => o,
            Self::South(o) => o,
            Self::West(o) => o,
        }
    }
}

pub struct PlacePlugin;

/// This tracks whether something changed so that it can be updated
#[derive(Default, Resource, Debug)]
pub enum PlaceDelta {
    #[default]
    None,
    Update,
}

impl PlaceDelta {
    pub fn requested_update(&self) -> bool {
        match self {
            Self::None => false,
            Self::Update => true,
        }
    }
}

impl Plugin for PlacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ModelCursor::default());
        app.insert_resource(PlaceDelta::default());
        app.insert_resource(Orientation::index(2));

        app.add_systems(
            Update,
            (
                control_cursor,
                update_mouse_point_object,
                place_model,
                remove_model,
                invisible_cursor,
            ),
        );
    }
}

fn control_cursor(
    keys: Res<Input<KeyCode>>,
    mut model_cursor: ResMut<ModelCursor>,
    mut orientation: ResMut<Orientation>,
    mut place_delta: ResMut<PlaceDelta>,
) {
    let mut report_change = || {
        *place_delta = PlaceDelta::Update;
    };

    if (keys.just_pressed(KeyCode::Right) || keys.just_pressed(KeyCode::D))
        && keys.pressed(KeyCode::AltLeft)
    {
        report_change();

        orientation.next();
    }

    if (keys.just_pressed(KeyCode::Left) || keys.just_pressed(KeyCode::A))
        && keys.pressed(KeyCode::AltLeft)
    {
        report_change();

        orientation.previous();
    }

    if (keys.just_pressed(KeyCode::Right) || keys.just_pressed(KeyCode::D))
        && !keys.pressed(KeyCode::AltLeft)
    {
        report_change();

        *model_cursor += 1;
    }

    if (keys.just_pressed(KeyCode::Left) || keys.just_pressed(KeyCode::A))
        && !keys.pressed(KeyCode::AltLeft)
    {
        report_change();

        *model_cursor -= 1;
    }

    if keys.just_pressed(KeyCode::Key1) {
        report_change();

        *model_cursor = ModelCursor::Floor(0)
    }

    if keys.just_pressed(KeyCode::Key2) {
        report_change();

        *model_cursor = ModelCursor::Buildings(0)
    }
}

fn update_mouse_point_object(
    mut mouse_point: Query<(&mut Transform, &mut Handle<Scene>), With<MousePointObject>>,
    mut place_delta: ResMut<PlaceDelta>,
    model_cursor: Res<ModelCursor>,
    orientation: ResMut<Orientation>,
    asset_server: Res<AssetServer>,
) {
    let (mut tf, mut scene) = mouse_point.single_mut();

    if !place_delta.requested_update() {
        return;
    }

    *scene = asset_server.load(model_cursor.meta().path);
    tf.rotation = Quat::from_rotation_y(orientation.rotation());

    *place_delta = PlaceDelta::None;
}

fn place_model(
    mut commands: Commands,
    cursor: Res<ModelCursor>,
    mouse_projection: Res<MouseProjection>,
    buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    orientation: Res<Orientation>,
    keys: Res<Input<KeyCode>>,
) {
    let model: Handle<Scene> = asset_server.load(cursor.meta().path);

    if buttons.just_pressed(MouseButton::Left) && !keys.pressed(KeyCode::ShiftLeft) {
        let mut tf = Transform::from_xyz(
            mouse_projection.normal.x as f32 * 20.,
            0.,
            mouse_projection.normal.y as f32 * 20.,
        );

        tf.rotation = Quat::from_rotation_y(orientation.rotation());

        commands.spawn((
            SceneBundle {
                scene: model,
                transform: tf,
                ..default()
            },
            Point {
                has: cursor.into(),
                position: mouse_projection.normal,
                orientation: orientation.clone()
            },
        ));
    }
}

fn remove_model(
    mut commands: Commands,
    mouse_projection: Res<MouseProjection>,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    points: Query<(Entity, &Point)>,
) {
    if buttons.just_pressed(MouseButton::Left) && keys.pressed(KeyCode::ShiftLeft) {
        for (entity, point) in points.iter() {
            if point.position == mouse_projection.normal {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn invisible_cursor(
    mut mouse_point: Query<&mut Visibility, With<MousePointObject>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut visibility = mouse_point.single_mut();

    if keys.just_pressed(KeyCode::ShiftLeft) {
        *visibility = Visibility::Hidden;
    }

    if keys.just_released(KeyCode::ShiftLeft) {
        *visibility = Visibility::Visible;
    }
}
