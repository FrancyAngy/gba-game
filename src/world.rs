use agb::{
    display::{
        GraphicsFrame, Priority,
        tiled::{
            InfiniteScrolledMap, RegularBackground, RegularBackgroundSize, TileFormat, TileSetting,
        },
    },
    fixnum::{Vector2D, vec2},
    input::{ButtonController, Tri},
    sound::mixer::Mixer,
};

pub struct World {
    pub level: &'static super::level::Level,
    pub bg: InfiniteScrolledMap,
    pub player: super::player::Player,
}

impl World {
    pub fn new(level: &'static super::level::Level) -> Self {
        let bg = RegularBackground::new(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        let bg = InfiniteScrolledMap::new(bg);

        World {
            level,
            bg,
            player: super::player::Player::new(level.player_start.into()),
        }
    }

    pub fn set_pos(&mut self, pos: Vector2D<i32>) {
        self.bg.set_scroll_pos(pos, |pos| {
            let tile = if self.level.bounds().contains_point(pos) {
                // Convert 2D coordinates to a 1D index into our flat tile array.
                // This is called row-major indexing.
                let idx = pos.x + pos.y * self.level.width as i32;
                self.level.background[idx as usize]
            } else {
                // Use the transparent tile outside the level bounds.
                // agb may contain specific optimizations around the blank tile.
                TileSetting::BLANK
            };

            (&super::tiles::TILES.tiles, tile)
        });
    }

    pub fn show(&self, frame: &mut GraphicsFrame) {
        self.bg.show(frame);
        self.player.show(frame);
    }

    pub fn has_won(&self) -> bool {
        self.player.has_won(self.level)
    }

    pub fn spike_hit(&self) -> bool {
        self.player.hits_spike(self.level)
    }

    pub fn in_water(&self) -> bool {
        self.player.hits_water(self.level)
    }

    pub fn collides(&self) -> Tri {
        self.level.collides(self.player.pos.floor() / 8)
    }

    pub fn update(&mut self, buttons: &ButtonController, mixer: &mut Mixer) {
        self.set_pos(vec2(0, 0));
        self.player.update(buttons, self.level, mixer);
    }
}
