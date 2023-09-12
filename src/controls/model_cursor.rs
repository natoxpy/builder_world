use bevy::prelude::Resource;

use crate::models::{BuildingModel, FloorModel, Meta};

///
/// This cursor keeps track of what item is selected to be place down
#[derive(Debug, PartialEq, Resource, Clone, Eq)]
pub enum ModelCursor {
    Floor(usize),
    Buildings(usize),
}

impl Default for ModelCursor {
    fn default() -> Self {
        Self::Floor(0)
    }
}

impl ModelCursor {
    pub fn is(&self, other: Self) -> bool {
        if let Self::Floor(_) = self {
            if let Self::Floor(_) = other {
                return true;
            }
        }

        if let Self::Buildings(_) = self {
            if let Self::Buildings(_) = other {
                return true;
            }
        }

        false
    }

    pub fn max(&self) -> usize {
        match self {
            Self::Floor(_) => FloorModel::len(),
            Self::Buildings(_) => BuildingModel::len(),
        }
    }

    pub fn meta(&self) -> Meta {
        match self {
            Self::Floor(index) => FloorModel::index(index.clone()).get_meta().clone(),
            Self::Buildings(index) => BuildingModel::index(index.clone()).get_meta().clone(),
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::Floor(i) => i.clone(),
            Self::Buildings(i) => i.clone(),
        }
    }

    pub fn set(&mut self, c: usize) {
        match self {
            Self::Floor(i) => *i = c,
            Self::Buildings(i) => *i = c,
        }
    }
}

impl std::ops::Add<usize> for ModelCursor {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Self::Buildings(c) => Self::Buildings(c + rhs),
            Self::Floor(c) => Self::Floor(c + rhs),
        }
    }
}

impl std::ops::Sub<usize> for ModelCursor {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        match self {
            Self::Buildings(c) => Self::Buildings(c - rhs),
            Self::Floor(c) => Self::Floor(c - rhs),
        }
    }
}

impl std::ops::AddAssign<usize> for ModelCursor {
    fn add_assign(&mut self, rhs: usize) {
        let max = self.max() - 1;

        let apply_or_reset = |c: &mut usize| {
            if c.clone() >= max {
                *c = 0;
                return;
            }

            *c += rhs;
        };

        match self {
            Self::Buildings(c) => apply_or_reset(c),
            Self::Floor(c) => apply_or_reset(c),
        };
    }
}

impl std::ops::SubAssign<usize> for ModelCursor {
    fn sub_assign(&mut self, rhs: usize) {
        let max = self.max() - 1;

        let apply_or_reset = |val: &mut usize| {
            if val.clone() > 0 {
                *val -= rhs;
                return;
            }

            *val = max;
        };

        match self {
            Self::Buildings(val) => apply_or_reset(val),
            Self::Floor(val) => apply_or_reset(val),
        };
    }
}
