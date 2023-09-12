use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    controls::{model_cursor::ModelCursor, place_model::Orientation},
    models::{BuildingModel, FloorModel, Meta},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PointType {
    Grass,
    Concrete,
    RoadStraight,
    RoadStraightWalkable,
    RoadEnd,
    RoadStraightSideOpen,
    RoadCorner,
    RoadCornerWalkable,
    RoadIntersection,
    RoadIntersectionWalkable,

    Blgd01_01,
    Blgd02_01,
}

impl From<FloorModel> for PointType {
    fn from(value: FloorModel) -> Self {
        match value {
            FloorModel::Grass(_) => Self::Grass,
            FloorModel::Concrete(_) => Self::Concrete,

            FloorModel::RoadStraight(_) => Self::RoadStraight,
            FloorModel::RoadStraightWalkable(_) => Self::RoadStraightWalkable,
            FloorModel::RoadEnd(_) => Self::RoadEnd,
            FloorModel::RoadStraightSideOpen(_) => Self::RoadStraightSideOpen,
            FloorModel::RoadCorner(_) => Self::RoadCorner,
            FloorModel::RoadCornerWalkable(_) => Self::RoadCornerWalkable,
            FloorModel::RoadIntersection(_) => Self::RoadIntersection,
            FloorModel::RoadIntersectionWalkable(_) => Self::RoadIntersectionWalkable,
            _ => panic!("not implemented!"),
        }
    }
}

impl From<BuildingModel> for PointType {
    fn from(value: BuildingModel) -> Self {
        match value {
            BuildingModel::Blgd01_01(_) => Self::Blgd01_01,
            BuildingModel::Blgd02_01(_) => Self::Blgd02_01,
        }
    }
}

impl From<Res<'_, ModelCursor>> for PointType {
    fn from(value: Res<'_, ModelCursor>) -> Self {
        match *value {
            ModelCursor::Floor(v) => FloorModel::index(v).into(),
            ModelCursor::Buildings(v) => BuildingModel::index(v).into(),
        }
    }
}

impl PointType {
    pub fn meta(&self) -> Meta {
        if let Ok(a) = FloorModel::try_from(self.clone()) {
            return a.get_meta().clone();
        };

        if let Ok(a) = BuildingModel::try_from(self.clone()) {
            return a.get_meta().clone();
        };

        panic!("This type does not have meta {:?}", self);
    }
}

#[derive(Default, Clone, Deserialize, Serialize, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Clone, Deserialize, Serialize)]
pub struct Point {
    pub has: PointType,
    pub position: Position,
    pub orientation: Orientation,
}

impl Point {
    pub fn new(has: PointType, position: Position, orientation: Orientation) -> Self {
        Self {
            has,
            position,
            orientation,
        }
    }
}
