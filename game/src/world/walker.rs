use agb::{
    display::{
        Priority,
        object::{Object, SpriteVram},
    },
    fixnum::{Num, num, rect, vec2},
    include_aseprite,
    input::Tri,
};

use crate::{Number, Vector, level::Level};

pub struct Walker {
    pos: Vector,
    speed: Vector,
    base_speed: Vector,
    sprite: SpriteVram,
    frame: usize,
    alive: bool,
    direction: Number,
}

include_aseprite!(mod sprite, "gfx/walker.aseprite");

static WALKER_SPEED: Number = num!(0.6);

impl Walker {
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

    fn touches_left_wall(&self, level: &Level) -> bool {
        let check = vec2(self.pos.x - Number::from_raw(1), self.pos.y - num!(4));

        level.collides(check.floor() / 8) == Tri::Positive
    }

    fn touches_right_wall(&self, level: &Level) -> bool {
        let check = vec2(self.pos.x + 8, self.pos.y - num!(4));

        level.collides(check.floor() / 8) == Tri::Positive
    }

    fn in_water(&self, level: &Level) -> bool {
        level.water_collision(self.pos.floor() / 8)
    }

    fn on_ground(&self, level: &Level) -> bool {
        let check = vec2(self.pos.x, self.pos.y + num!(8) + Number::from_raw(1));

        level.collides(check.floor() / 8) == Tri::Positive
    }
}

impl super::object_traits::Entity for Walker {
    fn new(start_pos: agb::fixnum::Vector2D<i32>) -> Self
    where
        Self: Sized,
    {
        Walker {
            pos: start_pos.change_base(),
            speed: vec2(WALKER_SPEED, num!(0)),
            base_speed: vec2(WALKER_SPEED, num!(0)),
            sprite: sprite::WALK.sprite(0).into(),
            frame: 0,
            alive: true,
            direction: num!(1),
        }
    }

    fn get_type(&self) -> super::object_traits::EntityType {
        super::object_traits::EntityType::Walker
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos.floor(), vec2(8, 8))
    }

    fn show(&self, frame: &mut agb::display::GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_priority(Priority::P1)
            .set_hflip(self.speed.x > num!(-0.1))
            .set_pos(self.pos.floor())
            .show(frame);
    }

    fn update(
        &mut self,
        _player: &mut crate::player::Player,
        _mixer: &mut agb::sound::mixer::Mixer,
        level: &Level,
    ) -> bool {
        if self.in_water(level) {
            self.alive = false;
        }

        if self.alive {
            self.speed.y += num!(0.18);
            self.handle_collision(level);
            self.update_sprite();
            if self.touches_right_wall(level) || self.touches_left_wall(level) {
                self.direction *= num!(-1);
                self.base_speed.x = self.direction * WALKER_SPEED;
            }
            if !self.on_ground(level) {
                self.speed.x = self.base_speed.x / 3;
            } else {
                self.speed.x = self.base_speed.x;
            }
        }

        self.alive
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
        } else {
            self.speed *= num!(-1);
        }
    }
}
