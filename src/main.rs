use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub mod controls;
pub mod data;
pub mod models;
pub mod world;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(30., 30., 30.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        controls::ControlPlugin,
        world::WorldPlugin,
        data::DataPlugin, // bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
    ));

    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    app.add_systems(Startup, setup);

    app.run();
}
