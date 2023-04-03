use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Terrain;

#[derive(Resource, Clone, Copy)]
pub struct Grid {
    cell_size: f32,
}

impl Grid {
    pub fn new(cell_size: f32) -> Self {
        Self { cell_size }
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }

    pub fn snap(&self, pos: Vec2) -> Vec2 {
        Vec2::new(self.snap_coordinate(pos.x), self.snap_coordinate(pos.y))
    }

    fn half_cell(&self) -> f32 {
        self.cell_size / 2.0
    }

    fn snap_coordinate(&self, pos: f32) -> f32 {
        let rest_full = pos % self.cell_size;
        let rest_half = pos % self.half_cell();

        if rest_full == rest_half {
            pos - rest_half + self.half_cell()
        } else {
            pos - rest_half
        }
    }
}
