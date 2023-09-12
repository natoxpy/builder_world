use bevy::prelude::*;

use crate::world::point::PointType;

#[derive(Debug, Clone)]
pub struct Meta {
    pub path: &'static str,
}

impl Meta {
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }
}

#[derive(Debug, Clone, Resource)]
pub enum FloorModel {
    Grass(Meta),
    Concrete(Meta),
    RoadStraight(Meta),
    RoadStraightWalkable(Meta),
    RoadEnd(Meta),
    RoadStraightSideOpen(Meta),

    RoadCornerWalkable(Meta),
    RoadCorner(Meta),
    RoadIntersection(Meta),
    RoadIntersectionWalkable(Meta),
    None,
}

impl FloorModel {
    const CONCRETE: Self =
        Self::Concrete(Meta::new("./models/roads/road_prop_tile_dark.glb#Scene0"));
    const GRASS: Self = Self::Grass(Meta::new("./models/grass_flat.glb#Scene0"));

    const ROAD_STRAIGHT: Self =
        Self::RoadStraight(Meta::new("./models/roads/road_straight.glb#Scene0"));
    const ROAD_STRAIGHT_WALKABLE: Self = Self::RoadStraightWalkable(Meta::new(
        "./models/roads/road_straight_walkable.glb#Scene0",
    ));
    const ROAD_STRAIGHT_SIDE_OPEN: Self = Self::RoadStraightSideOpen(Meta::new(
        "./models/roads/road_straight_side_open.glb#Scene0",
    ));
    const ROAD_END: Self = Self::RoadEnd(Meta::new("./models/roads/road_end.glb#Scene0"));
    const ROAD_CORNER: Self = Self::RoadCorner(Meta::new("./models/roads/road_corner.glb#Scene0"));
    const ROAD_CORNER_WALKABLE: Self =
        Self::RoadCornerWalkable(Meta::new("./models/roads/road_corner_walkable.glb#Scene0"));
    const ROAD_INTERSECTION: Self =
        Self::RoadIntersection(Meta::new("./models/roads/road_intersection.glb#Scene0"));
    const ROAD_INTERSECTION_WALKABLE: Self = Self::RoadIntersectionWalkable(Meta::new(
        "./models/roads/road_intersection_walkable.glb#Scene0",
    ));

    const INDICES: [Self; 10] = [
        Self::CONCRETE,
        Self::GRASS,
        Self::ROAD_STRAIGHT,
        Self::ROAD_STRAIGHT_WALKABLE,
        Self::ROAD_STRAIGHT_SIDE_OPEN,
        Self::ROAD_END,
        Self::ROAD_CORNER,
        Self::ROAD_CORNER_WALKABLE,
        Self::ROAD_INTERSECTION,
        Self::ROAD_INTERSECTION_WALKABLE,
    ];

    pub fn get_meta(&self) -> &Meta {
        match self {
            Self::Grass(meta) => meta,
            Self::Concrete(meta) => meta,
            Self::RoadStraight(meta) => meta,
            Self::RoadCornerWalkable(meta) => meta,
            Self::RoadStraightSideOpen(meta) => meta,
            Self::RoadCorner(meta) => meta,
            Self::RoadEnd(meta) => meta,
            Self::RoadIntersection(meta) => meta,
            Self::RoadIntersectionWalkable(meta) => meta,
            Self::RoadStraightWalkable(meta) => meta,
            Self::None => panic!("None doesn't have meta!"),
        }
    }

    pub fn len() -> usize {
        return Self::INDICES.len();
    }

    pub fn index(index: usize) -> Self {
        Self::INDICES.get(index).unwrap().clone()
    }
}

#[derive(Clone)]
pub enum BuildingModel {
    Blgd01_01(Meta),
    Blgd02_01(Meta),
}

impl BuildingModel {
    const BLGD01_01: Self = Self::Blgd01_01(Meta::new("./models/bldg/bldg_01_01.glb#Scene0"));
    const BLGD02_01: Self = Self::Blgd02_01(Meta::new("./models/bldg/bldg_02_01.glb#Scene0"));

    const INDICES: [Self; 2] = [Self::BLGD01_01, Self::BLGD02_01];

    pub fn get_meta(&self) -> &Meta {
        match self {
            Self::Blgd01_01(meta) => meta,
            Self::Blgd02_01(meta) => meta,
        }
    }

    pub fn len() -> usize {
        Self::INDICES.len()
    }

    pub fn index(index: usize) -> Self {
        Self::INDICES.get(index).unwrap().clone()
    }
}

impl TryFrom<PointType> for FloorModel {
    type Error = &'static str;
    fn try_from(value: PointType) -> Result<Self, Self::Error> {
        match value {
            PointType::Concrete => Ok(Self::CONCRETE),
            PointType::Grass => Ok(Self::GRASS),
            PointType::RoadStraight => Ok(Self::ROAD_STRAIGHT),
            PointType::RoadStraightWalkable => Ok(Self::ROAD_STRAIGHT_WALKABLE),
            PointType::RoadStraightSideOpen => Ok(Self::ROAD_STRAIGHT_SIDE_OPEN),
            PointType::RoadEnd => Ok(Self::ROAD_END),
            PointType::RoadCorner => Ok(Self::ROAD_CORNER),
            PointType::RoadCornerWalkable => Ok(Self::ROAD_CORNER_WALKABLE),
            PointType::RoadIntersection => Ok(Self::ROAD_INTERSECTION),
            PointType::RoadIntersectionWalkable => Ok(Self::ROAD_INTERSECTION_WALKABLE),
            _ => Err("Not found"),
        }
    }
}

impl TryFrom<PointType> for BuildingModel {
    type Error = &'static str;

    fn try_from(value: PointType) -> Result<Self, Self::Error> {
        match value {
            PointType::Blgd01_01 => Ok(Self::BLGD01_01),
            PointType::Blgd02_01 => Ok(Self::BLGD02_01),
            _ => Err("Not found"),
        }
    }
}
