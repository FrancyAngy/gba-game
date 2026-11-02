use agb::{
    display::object::{Object, SpriteVram},
    fixnum::{num, rect, vec2},
    include_aseprite,
    input::Tri,
};

use crate::{Number, Vector, level::Level};

pub struct Jumper {
    pos: Vector,
    speed: Vector,
    sprite: SpriteVram,
    frame: usize,
    alive: bool,
    timer: i32,
}

include_aseprite!(mod sprite, "gfx/walker.aseprite");

impl Jumper {
    fn update_sprite(&mut self) {
        self.sprite = sprite::WALK.animation_sprite(self.frame).into();
    }

    fn handle_collision_component(
        speed: &mut Number,
        pos: &mut Number,
        half_width: i32,
        colliding: &dyn Fn(i32) -> bool,
    ) {
        let potential = *speed + *pos;
        let potential_ext = potential + speed.to_raw().signum() * half_width;

        let target = potential_ext.floor() / 8;

        if !colliding(target) {
            *pos = potential;
        } else {
            let center = target * 8 + 4;
            let player_pos = center - speed.to_raw().signum() * (4 + half_width);
            *pos = player_pos.into();
            *speed = 0.into();
        }
    }

    fn handle_collision(&mut self, level: &Level) {
        let mut x_pos = self.pos.x + num!(4);
        let mut y_pos = self.pos.y + num!(4);
        Self::handle_collision_component(&mut self.speed.x, &mut x_pos, 4, &|x| {
            level.collides(vec2(x, self.pos.y.floor() / 8)) == Tri::Positive
        });
        Self::handle_collision_component(&mut self.speed.y, &mut y_pos, 4, &|y| {
            level.collides(vec2(self.pos.x.floor() / 8, y)) == Tri::Positive
        });
        self.pos.x = x_pos - num!(4);
        self.pos.y = y_pos - num!(4);
    }

    pub fn is_on_ground(&self, level: &Level) -> bool {
        let check = vec2(self.pos.x, self.pos.y + 8 + Number::from_raw(1));

        level.collides(check.floor() / 8) == Tri::Positive
    }
}

impl super::object_traits::Entity for Jumper {
    fn new(start_pos: agb::fixnum::Vector2D<i32>) -> Self
    where
        Self: Sized,
    {
        Jumper {
            pos: start_pos.change_base(),
            speed: vec2(num!(0), num!(0)),
            sprite: sprite::WALK.sprite(0).into(),
            frame: 0,
            alive: true,
            timer: 60,
        }
    }

    fn update(
        &mut self,
        player: &mut crate::player::Player,
        _mixer: &mut agb::sound::mixer::Mixer,
        level: &crate::level::Level,
    ) -> bool {
        let on_ground = self.is_on_ground(level);
        self.speed.y += num!(0.18);

        self.handle_collision(level);

        if on_ground {
            self.speed.x = num!(0);
        }

        if self.timer == 0 && on_ground {
            self.timer = 180;
            self.speed = vec2(num!(0.8), num!(-2));
            if self.pos.x > player.pos.x {
                self.speed.x *= num!(-1)
            }
        } else if self.timer > 0 {
            self.timer -= 1;
        }

        self.pos += self.speed;

        self.update_sprite();
        self.alive
    }

    fn get_type(&self) -> super::object_traits::EntityType {
        super::object_traits::EntityType::Jumper
    }

    fn show(&self, frame: &mut agb::display::GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_hflip(self.speed.x <= num!(-0.1))
            .set_pos(self.pos.floor())
            .show(frame);
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos.floor(), vec2(8, 8))
    }

    fn handle_player_collision(
        &mut self,
        player: &mut crate::player::Player,
        mixer: &mut agb::sound::mixer::Mixer,
    ) {
        player.die(mixer);
    }

    fn handle_entity_collision(&mut self, other: &mut dyn super::object_traits::Entity) {
        if other.get_type() == super::object_traits::EntityType::Projectile {
            self.alive = false;
        }
    }
}
