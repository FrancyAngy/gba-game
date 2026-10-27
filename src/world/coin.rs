use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, SpriteVram},
    },
    fixnum::Vector2D,
    include_aseprite,
};

use crate::Vector;

include_aseprite!(mod sprites, "gfx/coin.aseprite");

pub struct Coin {
    pub pos: Vector2D<i32>,
    sprite: SpriteVram,
    frame: usize,
}

impl Coin {
    fn update_sprite(&mut self) {
        self.sprite = sprites::SPIN.animation_sprite(self.frame).into();
    }
}

impl super::object_traits::Collectible for Coin {
    fn new(pos: Vector) -> Self {
        Coin {
            pos: pos.floor(),
            sprite: sprites::SPIN.sprite(0).into(),
            frame: 0,
        }
    }

    fn update(&mut self) -> bool {
        self.update_sprite();
        true
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_pos(self.pos)
            .set_priority(Priority::P2)
            .show(frame);
    }
}
