use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, SpriteVram},
    },
    fixnum::{Vector2D, num, vec2},
    include_aseprite, include_wav,
    input::{Button, ButtonController, Tri},
    sound::mixer::{ChannelId, Mixer, SoundChannel, SoundData},
};

#[derive(Eq, PartialEq)]
pub enum ContinuousSfxId {
    None,
    Slide,
}

include_aseprite!(mod sprites, "gfx/player.aseprite");

static WATER_SFX: SoundData = include_wav!("sfx/water.wav");
static JUMP_SFX: SoundData = include_wav!("sfx/jump.wav");
static SWIM_SFX: SoundData = include_wav!("sfx/swim.wav");
static SLIDE_SFX: SoundData = include_wav!("sfx/slide.wav");

pub struct Player {
    pub pos: super::Vector,
    pub speed: super::Vector,
    pub frame: usize,
    pub sprite: SpriteVram,
    pub flipped: bool,
    pub was_in_water: bool,
    pub hide: bool,
    pub walljump_state: i32,
    pub continuous_sfx: Option<ChannelId>,
    pub continuous_sfx_id: ContinuousSfxId,
    can_jump: i32,
    walljump_timer: i32,
}

impl Player {
    pub fn new(start: Vector2D<i32>) -> Self {
        Player {
            pos: start.change_base(),
            speed: vec2(num!(0), num!(0)),
            frame: 0,
            sprite: sprites::STANDING.sprite(0).into(),
            flipped: false,
            was_in_water: false,
            hide: false,
            can_jump: 0,
            walljump_state: 0,
            walljump_timer: 0,
            continuous_sfx: None,
            continuous_sfx_id: ContinuousSfxId::None,
        }
    }

    pub fn die(&mut self, mixer: &mut Mixer) {
        self.hide = true;
        if let Some(id) = self.continuous_sfx.take() {
            if let Some(channel) = mixer.channel(&id) {
                channel.stop();
            }
        }
    }

    pub fn show(&self, frame: &mut GraphicsFrame) {
        if !self.hide {
            Object::new(self.sprite.clone())
                .set_hflip(self.flipped)
                .set_pos(self.pos.round() - vec2(8, 8))
                .set_priority(Priority::P1)
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
            mixer.play_sound(SoundChannel::new(SWIM_SFX));
        } else {
            self.speed.y = num!(-2.2);
            mixer.play_sound(SoundChannel::new(JUMP_SFX));
        }
    }

    pub fn handle_wall_jump(&mut self, touches_left: bool, mixer: &mut Mixer) {
        self.speed.x = num!(-1);
        if touches_left {
            self.speed.x *= -1;
        }
        self.speed.y = num!(-2.2);
        mixer.play_sound(SoundChannel::new(JUMP_SFX));
    }

    pub fn update_sprite(&mut self) {
        if self.speed.x > num!(0) {
            self.flipped = false;
        } else if self.speed.x < num!(0) {
            self.flipped = true;
        }

        self.sprite = if self.speed.y < num!(-0.1) {
            sprites::JUMPING.animation_frame(&mut self.frame, 2)
        } else if self.speed.y > num!(0.1) {
            sprites::FALLING.animation_frame(&mut self.frame, 2)
        } else if self.speed.x.abs() > num!(0.05) {
            sprites::WALKING.animation_frame(&mut self.frame, 2)
        } else {
            sprites::STANDING.animation_frame(&mut self.frame, 2)
        }
        .into()
    }

    pub fn is_on_ground(&self, level: &super::level::Level) -> bool {
        let check = vec2(self.pos.x, self.pos.y + 8 + super::Number::from_raw(1));

        level.collides(check.floor() / 8) == Tri::Positive
    }

    pub fn touches_left_wall(&self, level: &super::level::Level) -> bool {
        let check = vec2(self.pos.x - 5 - super::Number::from_raw(1), self.pos.y);

        level.collides(check.floor() / 8) == Tri::Positive
    }

    pub fn touches_right_wall(&self, level: &super::level::Level) -> bool {
        let check = vec2(self.pos.x + 4 + super::Number::from_raw(1), self.pos.y);

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
        let touch_left = self.touches_left_wall(level);
        let touch_right = self.touches_right_wall(level);
        let x_tri = buttons.x_tri();

        if on_ground && (self.can_jump != 10 || self.walljump_state == 2) {
            self.can_jump = 10;
            self.walljump_state = 0;
        } else if (in_water && self.can_jump != 0) || (self.speed.y <= num!(-1)) {
            self.can_jump = 0;
        } else if self.can_jump > 0 {
            self.can_jump -= 1;
        }

        if in_water && self.walljump_state != 0 {
            self.walljump_state = 0;
            self.walljump_timer = 0;
        }

        if self.walljump_state != 2 {
            self.handle_horizontal_input(x_tri as i32, on_ground);
        }

        if buttons.is_just_pressed(Button::A) && (self.can_jump > 0 || in_water) {
            self.handle_jump(in_water, in_top_water, mixer);
        }

        if x_tri == Tri::Zero {
            if on_ground {
                self.speed.x *= 12;
                self.speed.x /= 16;
            } else if in_water {
                self.speed.x *= 63;
                self.speed.x /= 64;
            }
        } else if ((x_tri == Tri::Positive && self.speed.x < num!(0))
            || (x_tri == Tri::Negative && self.speed.x > num!(0)))
            && self.walljump_state != 2
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
            mixer.play_sound(SoundChannel::new(WATER_SFX));
        }

        if ((touch_left && x_tri == Tri::Negative) || (touch_right && x_tri == Tri::Positive))
            && !on_ground
            && !in_water
            && self.walljump_timer == 0
            && buttons.is_released(Button::A)
        {
            self.walljump_state = 1;
        }

        if self.walljump_state == 1 && buttons.is_just_pressed(Button::A) {
            self.handle_wall_jump(touch_left, mixer);
            self.walljump_state = 2;
            self.walljump_timer = 3;
        } else if self.walljump_state == 1 {
            self.speed.y = num!(0.4);
        }

        if self.walljump_state == 1 && (!(touch_right || touch_left) || on_ground || in_water) {
            self.walljump_state = 0;
        }

        if self.walljump_timer > 0 {
            self.walljump_timer -= 1;
        }

        if self.walljump_state == 1 && self.continuous_sfx.is_none() {
            let mut sfx = SoundChannel::new(SLIDE_SFX);
            sfx.should_loop();

            self.continuous_sfx = mixer.play_sound(sfx);
            self.continuous_sfx_id = ContinuousSfxId::Slide;
        }

        if self.walljump_state != 1
            && self.continuous_sfx_id == ContinuousSfxId::Slide
            && !self.continuous_sfx.is_none()
        {
            if let Some(id) = self.continuous_sfx.take() {
                if let Some(channel) = mixer.channel(&id) {
                    channel.stop();
                }
            }
        }

        self.handle_collision(level);

        self.update_sprite();

        self.was_in_water = in_water;
    }
}
