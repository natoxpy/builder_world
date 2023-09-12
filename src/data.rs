use std::fs;

use bevy::prelude::*;

use crate::world::point::Point;

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (save_key, load_key));
    }
}

fn save_key(keys: Res<Input<KeyCode>>, points: Query<&Point>) {
    let mut points_vecs = vec![];

    for point in points.iter() {
        points_vecs.push(point);
    }

    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::S) {
        let content = serde_json::to_string(&points_vecs).unwrap();

        fs::write("./data.json", content).expect("Data writen!");
    }
}

fn load_key(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    points: Query<Entity, With<Point>>,
    asset_server: Res<AssetServer>,
) {
    let content = fs::read_to_string("./data.json").expect("Data for the BG menu not found!");

    let points_data: Vec<Point> = serde_json::from_str(&content).unwrap();

    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::L) {
        for point in points.iter() {
            commands.entity(point).despawn_recursive();
        }

        for point in points_data.iter() {
            let model: Handle<Scene> = asset_server.load(point.has.meta().path);

            let mut tf = Transform::from_xyz(
                point.position.x as f32 * 20.,
                0.,
                point.position.y as f32 * 20.,
            );

            tf.rotation = Quat::from_rotation_y(point.orientation.rotation());

            commands.spawn((
                SceneBundle {
                    scene: model,
                    transform: tf,
                    ..default()
                },
                point.clone(),
            ));
        }
    }
}
