use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, SpriteVram},
    },
    fixnum::{Vector2D, rect, vec2},
    include_aseprite, include_wav,
    sound::mixer::{SoundChannel, SoundData},
};

include_aseprite!(mod sprites, "gfx/coin.aseprite");
static COIN_SFX: SoundData = include_wav!("sfx/coin.wav");

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
    fn new(pos: Vector2D<i32>) -> Self {
        Coin {
            pos: pos,
            sprite: sprites::SPIN.sprite(0).into(),
            frame: 0,
        }
    }

    fn update(&mut self) {
        self.update_sprite();
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_pos(self.pos)
            .set_priority(Priority::P2)
            .show(frame);
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos, vec2(8, 8))
    }

    fn handle_collection(
        &mut self,
        _player: &mut crate::player::Player,
        mixer: &mut agb::sound::mixer::Mixer,
    ) {
        mixer.play_sound(SoundChannel::new(COIN_SFX));
    }
}
