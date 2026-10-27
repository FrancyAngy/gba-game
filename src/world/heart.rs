use agb::{
    display::object::{Object, SpriteVram},
    fixnum::{Vector2D, rect, vec2},
    include_aseprite,
};

include_aseprite!(mod sprite, "gfx/heart.aseprite");

pub struct Heart {
    pos: Vector2D<i32>,
    sprite: SpriteVram,
    frame: usize,
}

impl Heart {
    fn update_sprite(&mut self) {
        self.sprite = sprite::HEART.animation_sprite(self.frame).into();
    }
}

impl super::object_traits::Collectible for Heart {
    fn new(pos: Vector2D<i32>) -> Self
    where
        Self: Sized,
    {
        Heart {
            pos: pos,
            sprite: sprite::HEART.sprite(0).into(),
            frame: 0,
        }
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos, vec2(8, 8))
    }

    fn show(&self, frame: &mut agb::display::GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_pos(self.pos)
            .show(frame);
    }

    fn handle_collection(
        &mut self,
        player: &mut crate::player::Player,
        mixer: &mut agb::sound::mixer::Mixer,
    ) {
        player.lives = (player.lives + 1).clamp(0, crate::START_LIVES);
    }

    fn update(&mut self) {
        self.update_sprite();
    }
}
