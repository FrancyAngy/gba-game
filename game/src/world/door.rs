use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, SpriteVram},
    },
    fixnum::{Vector2D, rect, vec2},
    include_aseprite, println,
};

use crate::level::Level;

pub struct Door {
    pos: Vector2D<i32>,
    sprite: SpriteVram,
    frame: usize,
    closed: bool,
}

include_aseprite!(mod sprite, "gfx/door.aseprite");

impl Door {
    fn update_sprite(&mut self) {
        self.sprite = sprite::DOOR.animation_sprite(0).into();
    }
}

impl super::object_traits::Entity for Door {
    fn update(
        &mut self,
        _player: &mut crate::player::Player,
        _mixer: &mut agb::sound::mixer::Mixer,
        _level: &Level,
    ) -> bool {
        self.update_sprite();
        self.closed
    }

    fn new(start_pos: Vector2D<i32>) -> Self
    where
        Self: Sized,
    {
        Door {
            pos: start_pos,
            sprite: sprite::DOOR.sprite(0).into(),
            frame: 0,
            closed: true,
        }
    }

    fn get_type(&self) -> super::object_traits::EntityType {
        super::object_traits::EntityType::Door
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos, vec2(8, 16))
    }

    fn handle_player_collision(
        &mut self,
        player: &mut crate::player::Player,
        mixer: &mut agb::sound::mixer::Mixer,
    ) {
        if player.keys > 0 {
            player.keys -= 1;
            self.closed = false;
        }
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_priority(Priority::P1)
            .set_pos(self.pos)
            .show(frame);
    }

    fn is_solid(&self) -> bool {
        self.closed
    }
}
