use agb::{
    display::object::{Object, SpriteVram},
    fixnum::{Vector2D, rect, vec2},
    include_aseprite,
    sound::mixer::SoundChannel,
};

include_aseprite!(mod sprites, "gfx/key.aseprite");

pub struct Key {
    pos: Vector2D<i32>,
    sprite: SpriteVram,
    frame: usize,
}

impl Key {
    fn update_sprite(&mut self) {
        self.sprite = sprites::SPIN.animation_sprite(self.frame).into();
    }
}

impl super::Collectible for Key {
    fn new(pos: Vector2D<i32>) -> Self
    where
        Self: Sized,
    {
        Key {
            pos,
            sprite: sprites::SPIN.sprite(0).into(),
            frame: 0,
        }
    }

    fn update(&mut self) {
        self.update_sprite();
    }

    fn show(&self, frame: &mut agb::display::GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_pos(self.pos)
            .show(frame);
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos, vec2(8, 8))
    }

    fn handle_collection(
        &mut self,
        player: &mut crate::player::Player,
        mixer: &mut agb::sound::mixer::Mixer,
    ) {
        player.keys += 1;
        mixer.play_sound(SoundChannel::new(super::COLLECT_SFX));
    }
}
