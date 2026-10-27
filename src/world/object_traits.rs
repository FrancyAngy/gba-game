use agb::{
    display::GraphicsFrame,
    fixnum::{Rect, Vector2D},
    sound::mixer::Mixer,
};

use crate::player::Player;

pub enum EntityType {
    None,
    Walker,
    Jumper,
}

pub trait Entity {
    fn new(start_pos: super::super::Vector) -> Self
    where
        Self: Sized;
    fn get_type(&self) -> EntityType;
    fn update(&mut self, player: &mut Player, mixer: &mut Mixer) -> bool {
        false
    }
    fn handle_player_collision(&mut self, player: &mut Player, mixer: &mut Mixer) {}
    fn handle_entity_collision(&mut self, other: &mut dyn Entity);
    fn get_rect(&self) -> Rect<i32>;
    fn show(&self, frame: &mut GraphicsFrame);
}

pub trait Collectible {
    fn new(pos: Vector2D<i32>) -> Self
    where
        Self: Sized;
    fn update(&mut self);
    fn show(&self, frame: &mut GraphicsFrame);
    fn handle_collection(&mut self, player: &mut Player, mixer: &mut Mixer) {}
    fn get_rect(&self) -> Rect<i32>;
}
