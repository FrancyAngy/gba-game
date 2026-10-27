use crate::Vector;
use agb::display::GraphicsFrame;

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
    fn update(&mut self, player: &mut super::super::player::Player) -> bool;
    fn handle_player_collision(&mut self, player: &mut super::super::player::Player) {}
    fn show(&self, frame: &mut GraphicsFrame);
}

pub trait Collectible {
    fn new(pos: Vector) -> Self
    where
        Self: Sized;
    fn update(&mut self) -> bool;
    fn show(&self, frame: &mut GraphicsFrame);
    fn handle_collection(&mut self, player: &mut super::super::player::Player) {}
}
