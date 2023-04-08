use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Terrain;

#[derive(Resource)]
pub struct Grid {
    height: f32,
    width: f32,
    cell_size: f32,
    cells_turret: Vec<Vec<Option<Entity>>>,
}

impl Grid {
    pub fn new(cell_size: f32, height: f32, width: f32) -> Self {
        let height = height - (height % cell_size);
        let width = width - (width % cell_size);

        let nb_columns = (width / cell_size) as usize;
        let nb_rows = (height / cell_size) as usize;
        let cells_turret = vec![vec![None; nb_rows]; nb_columns];

        Self {
            cell_size,
            height,
            width,
            cells_turret,
        }
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }

    pub fn snap_to_cell_center(&self, pos: Vec2) -> Vec2 {
        Vec2::new(
            self.snap_coordinate_to_cell_center(pos.x),
            self.snap_coordinate_to_cell_center(pos.y),
        )
    }

    fn half_cell(&self) -> f32 {
        self.cell_size / 2.0
    }

    pub fn snap_coordinate_to_cell_center(&self, pos: f32) -> f32 {
        let rest_full = pos % self.cell_size;
        let rest_half = pos % self.half_cell();

        if rest_full == rest_half {
            pos - rest_half + self.half_cell()
        } else {
            pos - rest_half
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn bot_row_y(&self) -> f32 {
        self.snap_coordinate_to_cell_center(0.0)
    }

    pub fn top_row_y(&self) -> f32 {
        self.snap_coordinate_to_cell_center(self.height - self.cell_size)
    }

    pub fn nb_rows(&self) -> usize {
        self.cells_turret[0].len()
    }

    pub fn nb_columns(&self) -> usize {
        self.cells_turret.len()
    }

    pub fn put_turret(&mut self, turret_pos: Vec2, turret_entity: Entity) {
        let turret_pos = self.snap_to_cell_center(turret_pos);
        let row_index = (turret_pos.y / self.cell_size) as usize;
        let column_index = (turret_pos.x / self.cell_size) as usize;
        self.cells_turret[column_index][row_index] = Some(turret_entity);
    }

    pub fn get_turret(&self, turret_pos: Vec2) -> Option<Entity> {
        let turret_pos = self.snap_to_cell_center(turret_pos);
        let row_index = (turret_pos.y / self.cell_size) as usize;
        let column_index = (turret_pos.x / self.cell_size) as usize;
        self.cells_turret[column_index][row_index]
    }
}
