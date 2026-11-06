use agb::display::object::Object;
use agb::fixnum::{num, rect, vec2};
use agb::input::Tri;
use agb::{display::object::SpriteVram, include_aseprite};

use crate::Vector;

pub struct Projectile {
    pos: Vector,
    speed: Vector,
    sprite: SpriteVram,
    frame: usize,
    hit: bool,
}

include_aseprite!(mod sprite, "gfx/projectile.aseprite");

impl Projectile {
    fn update_sprite(&mut self) {
        self.sprite = sprite::SPIN.animation_sprite(self.frame).into();
    }
}

impl super::object_traits::Entity for Projectile {
    fn new(start_pos: agb::fixnum::Vector2D<i32>) -> Self
    where
        Self: Sized,
    {
        Projectile {
            pos: start_pos.change_base(),
            speed: vec2(num!(0), num!(0)),
            sprite: sprite::SPIN.sprite(0).into(),
            frame: 0,
            hit: false,
        }
    }

    fn show(&self, frame: &mut agb::display::GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_pos(self.pos.floor())
            .show(frame);
    }

    fn get_type(&self) -> super::object_traits::EntityType {
        super::object_traits::EntityType::Projectile
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos.floor(), vec2(8, 8))
    }

    fn handle_entity_collision(&mut self, _other: &mut dyn super::object_traits::Entity) {
        self.hit = true;
    }

    fn update(
        &mut self,
        player: &mut crate::player::Player,
        _mixer: &mut agb::sound::mixer::Mixer,
        level: &crate::level::Level,
    ) -> bool {
        if self.speed == vec2(num!(0), num!(0)) {
            if !player.flipped {
                self.speed.x = num!(4);
            } else {
                self.speed.x = num!(-4);
            }
        }

        if level.collides((self.pos.floor() + vec2(4, 4)) / 8) != Tri::Zero {
            self.hit = true
        }

        self.pos += self.speed;

        !self.hit
    }
}
