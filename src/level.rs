use agb::{
    display::tiled::TileSetting,
    fixnum::{Rect, Vector2D, rect, vec2},
    input::Tri,
};

pub struct Level {
    pub width: u32,
    pub height: u32,
    pub background: &'static [TileSetting],
    pub collision_map: &'static [u8],
    pub winning_map: &'static [u8],
    pub spike_map: &'static [u8],
    pub water_map: &'static [u8],
    pub water_top_map: &'static [u8],
    pub player_start: (i32, i32),
}

impl Level {
    pub fn bounds(&self) -> Rect<i32> {
        rect(
            vec2(0, 0),
            vec2(self.width as i32 - 1, self.height as i32 - 1),
        )
    }

    pub fn collides(&self, tile: Vector2D<i32>) -> Tri {
        if !self.bounds().contains_point(tile) {
            return Tri::Negative;
        }

        let idx = (tile.x + tile.y * self.width as i32) as usize;

        if self.collision_map[idx / 8] & (1 << (idx % 8)) != 0 {
            return Tri::Positive;
        }

        Tri::Zero
    }

    pub fn win_collision(&self, tile: Vector2D<i32>) -> bool {
        if !self.bounds().contains_point(tile) {
            return false;
        }

        let idx = (tile.x + tile.y * self.width as i32) as usize;

        self.winning_map[idx / 8] & (1 << (idx % 8)) != 0
    }

    pub fn spike_collision(&self, tile: Vector2D<i32>) -> bool {
        if !self.bounds().contains_point(tile) {
            return false;
        }

        let idx = (tile.x + tile.y * self.width as i32) as usize;

        self.spike_map[idx / 8] & (1 << (idx % 8)) != 0
    }

    pub fn water_collision(&self, tile: Vector2D<i32>) -> bool {
        if !self.bounds().contains_point(tile) {
            return false;
        }

        let idx = (tile.x + tile.y * self.width as i32) as usize;

        self.water_map[idx / 8] & (1 << (idx % 8)) != 0
    }

    pub fn water_top_collision(&self, tile: Vector2D<i32>) -> bool {
        if !self.bounds().contains_point(tile) {
            return false;
        }

        let idx = (tile.x + tile.y * self.width as i32) as usize;

        self.water_top_map[idx / 8] & (1 << (idx % 8)) != 0
    }
}
