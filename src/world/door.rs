use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, SpriteVram},
    },
    fixnum::{Vector2D, rect, vec2},
    include_aseprite, println,
};

pub struct Door {
    pos: Vector2D<i32>,
    sprite: SpriteVram,
    frame: usize,
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
        player: &mut crate::player::Player,
        mixer: &mut agb::sound::mixer::Mixer,
    ) -> bool {
        self.update_sprite();
        !player.has_key
    }

    fn new(start_pos: Vector2D<i32>) -> Self
    where
        Self: Sized,
    {
        Door {
            pos: start_pos,
            sprite: sprite::DOOR.sprite(0).into(),
            frame: 0,
        }
    }

    fn get_type(&self) -> super::object_traits::EntityType {
        super::object_traits::EntityType::Door
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos, vec2(8, 16))
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_priority(Priority::P1)
            .set_pos(self.pos)
            .show(frame);
    }
}
