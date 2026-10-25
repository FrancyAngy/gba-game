use agb::{
    display::{
        GraphicsFrame,
        object::{Object, SpriteVram},
    },
    fixnum::{Vector2D, num, vec2},
    input::{Button, ButtonController, Tri},
    sound::mixer::{Mixer, SoundChannel},
};

pub struct Player {
    pub pos: super::Vector,
    pub speed: super::Vector,
    pub frame: usize,
    pub sprite: SpriteVram,
    pub flipped: bool,
    pub was_in_water: bool,
    pub hide: bool,
    can_jump: i32,
}

impl Player {
    pub fn new(start: Vector2D<i32>) -> Self {
        Player {
            pos: start.change_base(),
            speed: vec2(num!(0), num!(0)),
            frame: 0,
            sprite: super::sprites::STANDING.sprite(0).into(),
            flipped: false,
            was_in_water: false,
            hide: false,
            can_jump: 0,
        }
    }

    pub fn die(&mut self) {
        self.hide = true;
    }

    pub fn show(&self, frame: &mut GraphicsFrame) {
        if !self.hide {
            Object::new(self.sprite.clone())
                .set_hflip(self.flipped)
                .set_pos(self.pos.round() - vec2(8, 8))
                .show(frame);
        }
    }

    pub fn handle_horizontal_input(&mut self, x_tri: i32, on_ground: bool) {
        let mut x = super::Number::new(x_tri) / 4;

        if x_tri.signum() != self.speed.x.to_raw().signum() {
            x *= 2;
        }

        if on_ground {
            x *= 2;
        }

        self.speed.x += x / 16;
        self.speed.x = self.speed.x.clamp(num!(-2), num!(2))
    }

    pub fn handle_jump(&mut self, in_water: bool, in_top_water: bool, mixer: &mut Mixer) {
        if in_water && !in_top_water {
            self.speed.y = num!(-1);
            mixer.play_sound(SoundChannel::new(super::SWIM_SFX));
        } else {
            self.speed.y = num!(-2.2);
            mixer.play_sound(SoundChannel::new(super::JUMP_SFX));
        }
    }

    pub fn update_sprite(&mut self) {
        if self.speed.x > num!(0) {
            self.flipped = false;
        } else if self.speed.x < num!(0) {
            self.flipped = true;
        }

        self.sprite = if self.speed.y < num!(-0.1) {
            super::sprites::JUMPING.animation_frame(&mut self.frame, 2)
        } else if self.speed.y > num!(0.1) {
            super::sprites::FALLING.animation_frame(&mut self.frame, 2)
        } else if self.speed.x.abs() > num!(0.05) {
            super::sprites::WALKING.animation_frame(&mut self.frame, 2)
        } else {
            super::sprites::STANDING.animation_frame(&mut self.frame, 2)
        }
        .into()
    }

    pub fn is_on_ground(&self, level: &super::level::Level) -> bool {
        let check = vec2(self.pos.x, self.pos.y + 8 + super::Number::from_raw(1));

        level.collides(check.floor() / 8) == Tri::Positive
    }

    pub fn handle_collision_component(
        speed: &mut super::Number,
        pos: &mut super::Number,
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

    pub fn handle_collision(&mut self, level: &super::level::Level) {
        Self::handle_collision_component(&mut self.speed.x, &mut self.pos.x, 4, &|x| {
            level.collides(vec2(x, self.pos.y.floor() / 8)) == Tri::Positive
        });
        Self::handle_collision_component(&mut self.speed.y, &mut self.pos.y, 8, &|y| {
            level.collides(vec2(self.pos.x.floor() / 8, y)) == Tri::Positive
        });
    }

    pub fn has_won(&self, level: &super::level::Level) -> bool {
        level.win_collision(self.pos.floor() / 8)
    }

    pub fn hits_spike(&self, level: &super::level::Level) -> bool {
        level.spike_collision(self.pos.floor() / 8)
    }

    pub fn hits_water(&self, level: &super::level::Level) -> bool {
        level.water_collision(self.pos.floor() / 8)
    }

    pub fn hits_top_water(&self, level: &super::level::Level) -> bool {
        level.water_top_collision(self.pos.floor() / 8)
    }

    pub fn update(
        &mut self,
        buttons: &ButtonController,
        level: &super::level::Level,
        mixer: &mut Mixer,
    ) {
        let on_ground = self.is_on_ground(level);
        let in_water = self.hits_water(level);
        let in_top_water = self.hits_top_water(level);

        if on_ground && self.can_jump != 10 {
            self.can_jump = 10;
        } else if (in_water && self.can_jump != 0) || (self.speed.y <= num!(-1)) {
            self.can_jump = 0;
        } else if self.can_jump > 0 {
            self.can_jump -= 1;
        }

        self.handle_horizontal_input(buttons.x_tri() as i32, on_ground);

        if buttons.is_just_pressed(Button::A) && (self.can_jump > 0 || in_water) {
            self.handle_jump(in_water, in_top_water, mixer);
        }

        if buttons.x_tri() == Tri::Zero {
            if on_ground {
                self.speed.x *= 12;
                self.speed.x /= 16;
            } else if in_water {
                self.speed.x *= 63;
                self.speed.x /= 64;
            }
        } else if (buttons.x_tri() == Tri::Positive && self.speed.x < num!(0))
            || (buttons.x_tri() == Tri::Negative && self.speed.x > num!(0))
        {
            self.speed.x *= 11;
            self.speed.x /= 16;
        }

        if !in_water {
            if buttons.is_pressed(Button::A) {
                self.speed.y += num!(0.08);
            } else {
                self.speed.y += num!(0.18);
            }
        } else {
            self.speed.y = (self.speed.y + num!(0.04)).clamp(num!(-2), num!(1.4));
        }

        if !self.was_in_water && in_water {
            self.speed.x = num!(0);
            self.speed.y = num!(-0.03);
            mixer.play_sound(SoundChannel::new(super::WATER_SFX));
        }

        self.handle_collision(level);

        self.update_sprite();

        self.was_in_water = in_water;
    }
}
