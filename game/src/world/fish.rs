use agb::{
    display::object::{Object, SpriteVram},
    fixnum::{num, rect, vec2},
    include_aseprite,
    input::Tri,
};

use crate::{
    Number, Vector,
    level::{self, Level},
};

pub struct Fish {
    pos: Vector,
    speed: Vector,
    sprite: SpriteVram,
    frame: usize,
    alive: bool,
    timer: u32,
    water_timer: u32,
    flip: bool,
}

include_aseprite!(mod sprite, "gfx/fish.aseprite");

impl Fish {
    fn update_sprite(&mut self) {
        self.sprite = sprite::SWIM.animation_sprite(self.frame).into();

        if self.speed.x >= num!(0.1) {
            self.flip = false;
        } else if self.speed.x <= num!(0.1) {
            self.flip = true;
        }
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

    fn is_on_ground(&self, level: &Level) -> bool {
        let check = vec2(self.pos.x, self.pos.y + 8 + Number::from_raw(1));

        level.collides(check.floor() / 8) == Tri::Positive
    }

    fn in_water(&self, level: &Level) -> bool {
        level.water_collision(self.pos.floor() / 8)
    }
}

impl super::object_traits::Entity for Fish {
    fn new(start_pos: agb::fixnum::Vector2D<i32>) -> Self
    where
        Self: Sized,
    {
        Fish {
            pos: start_pos.change_base(),
            speed: vec2(num!(0), num!(0)),
            sprite: sprite::SWIM.sprite(0).into(),
            frame: 0,
            alive: true,
            timer: 60,
            water_timer: 240,
            flip: false,
        }
    }

    fn get_type(&self) -> super::object_traits::EntityType {
        super::object_traits::EntityType::Fish
    }

    fn get_rect(&self) -> agb::fixnum::Rect<i32> {
        rect(self.pos.floor(), vec2(8, 8))
    }

    fn show(&self, frame: &mut agb::display::GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_pos(self.pos.floor())
            .set_hflip(self.flip)
            .show(frame);
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

    fn update(
        &mut self,
        player: &mut crate::player::Player,
        _mixer: &mut agb::sound::mixer::Mixer,
        level: &Level,
    ) -> bool {
        let in_water = self.in_water(level);
        let on_ground = self.is_on_ground(level);
        if self.alive {
            if self.timer > 0 {
                self.timer -= 1;
            } else if in_water || on_ground {
                if in_water {
                    let delta = player.pos - self.pos;

                    let dir = if delta.x == num!(0) && delta.y == num!(0) {
                        vec2(num!(0), num!(0))
                    } else {
                        delta.fast_normalise()
                    };

                    let swim_speed = num!(0.6);
                    self.speed = vec2(dir.x * swim_speed, dir.y * swim_speed);
                } else if on_ground {
                    self.speed.x = if self.pos.x >= player.pos.x {
                        num!(1)
                    } else {
                        num!(-1)
                    };
                    self.speed.y = num!(-0.6);
                }

                self.timer = 90;
            }

            if in_water {
                if self.water_timer != 240 {
                    self.water_timer = 240;
                }

                self.speed *= 31;
                self.speed /= 32;
            } else {
                if self.water_timer > 0 {
                    self.water_timer -= 1;
                } else {
                    self.alive = false;
                }

                self.speed.y += num!(0.23);
            }

            self.pos += self.speed;

            if self.is_on_ground(level) && !self.in_water(level) {
                self.speed = vec2(num!(0), num!(0));
            }

            self.handle_collision(level);
            self.update_sprite();
        }

        self.alive
    }
}
