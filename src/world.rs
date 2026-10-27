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
use alloc::{boxed::Box, vec::Vec};

pub mod coin;
pub mod heart;
pub mod object_traits;

pub struct World {
    pub level: &'static super::level::Level,
    pub bg: InfiniteScrolledMap,
    pub player: super::player::Player,
    pub started: bool,
    pub entities: Vec<Box<dyn object_traits::Entity>>,
    pub collectibles: Vec<Box<dyn object_traits::Collectible>>,
    bg_pos: Vector2D<i32>,
}

impl World {
    pub fn new(level: &'static super::level::Level, lives: usize) -> Self {
        let bg = RegularBackground::new(
            Priority::P2,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        let bg = InfiniteScrolledMap::new(bg);

        World {
            level,
            bg,
            player: super::player::Player::new(level.player_start.into(), lives),
            bg_pos: vec2(0, 0),
            started: true,
            entities: Vec::with_capacity(32),
            collectibles: Vec::with_capacity(64),
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
        if self.started {
            self.player.show(frame);
            for entity in &self.entities {
                entity.show(frame);
            }
            for collectible in &self.collectibles {
                collectible.show(frame);
            }
        }
    }

    pub fn has_won(&self) -> bool {
        self.player.has_won(self.level)
    }

    pub fn spike_hit(&self) -> bool {
        self.player.hits_spike(self.level)
    }

    pub fn collides(&self) -> Tri {
        self.level.collides(self.player.pos.floor() / 8)
    }

    pub fn update(&mut self, buttons: &ButtonController, mixer: &mut Mixer) {
        self.set_pos(self.bg_pos);
        if self.started {
            self.player.update(buttons, self.level, mixer);

            for entity in &mut self.entities {
                if self.player.collision_rect.touches(entity.get_rect()) {
                    entity.handle_player_collision(&mut self.player, mixer);
                }
            }
            self.entities
                .retain_mut(|entity| entity.update(&mut self.player, mixer));

            self.collectibles.retain_mut(|collectible| {
                collectible.update();
                if self.player.collision_rect.touches(collectible.get_rect()) {
                    collectible.handle_collection(&mut self.player, mixer);
                    false
                } else {
                    true
                }
            });
        }
    }
}
