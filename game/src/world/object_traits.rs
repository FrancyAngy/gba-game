use agb::{
    display::GraphicsFrame,
    fixnum::{Rect, Vector2D},
    sound::mixer::{Mixer, SoundChannel},
};

use crate::{level::Level, player::Player};

#[derive(PartialEq, Eq)]
pub enum EntityType {
    Door,
    Projectile,
    Walker,
    Jumper,
}

pub trait Entity {
    fn new(start_pos: Vector2D<i32>) -> Self
    where
        Self: Sized;
    fn get_type(&self) -> EntityType;
    fn update(&mut self, player: &mut Player, mixer: &mut Mixer, level: &Level) -> bool {
        true
    }
    fn handle_player_collision(&mut self, player: &mut Player, mixer: &mut Mixer) {}
    fn handle_entity_collision(&mut self, other: &mut dyn Entity) {}
    fn get_rect(&self) -> Rect<i32>;
    fn is_solid(&self) -> bool {
        false
    }
    fn show(&self, frame: &mut GraphicsFrame);
}

pub trait Collectible {
    fn new(pos: Vector2D<i32>) -> Self
    where
        Self: Sized;
    fn update(&mut self);
    fn show(&self, frame: &mut GraphicsFrame);
    fn handle_collection(&mut self, player: &mut Player, mixer: &mut Mixer) {
        mixer.play_sound(SoundChannel::new(super::COLLECT_SFX));
    }
    fn get_rect(&self) -> Rect<i32>;
}
