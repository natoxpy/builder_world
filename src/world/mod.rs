use bevy::prelude::*;

pub mod point;

use point::{Point, Position};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::default());
    }
}

#[derive(Resource, Default)]
pub struct World {
    pub points: Vec<Point>,
}

impl World {
    pub fn get_point(&self, pos: &Position) -> Option<&Point> {
        let mut p: Option<&Point> = None;

        for point in &self.points {
            if &point.position == pos {
                p = Some(point);
                break;
            }
        }

        p
    }

    pub fn set_point(&mut self, point: Point) {
        let p_in = self.get_point(&point.position);

        if let Some(_) = p_in {
            return;
        }

        self.points.push(point);
    }
}