// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't have an operating
// system, so most of the content of the standard library doesn't apply.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, SpriteVram},
        tiled::{
            InfiniteScrolledMap, RegularBackground, RegularBackgroundSize, TileFormat, TileSetting,
        },
    },
    fixnum::{Num, Rect, Vector2D, num, rect, vec2},
    include_aseprite, include_background_gfx,
    input::{Button, ButtonController, Tri},
    println,
};
// By default no_std crates don't get alloc, so you won't be able to use things like Vec
// until you declare the extern crate. `agb` provides an allocator so it will all work
extern crate alloc;

mod shim;

mod levels {
    use super::Level;
    use agb::display::tiled::TileSetting;
    static TILES: &[TileSetting] = super::tiles::TILES.tile_settings;

    include!(concat!(env!("OUT_DIR"), "/levels.rs"));
}

include_background_gfx!(mod tiles,
    "03adfc",
    TILES => "gfx/tileset.png");

include_aseprite!(mod sprites, "gfx/sprites.aseprite");

type Number = Num<i32, 8>;
type Vector = Vector2D<Number>;

struct Level {
    width: u32,
    height: u32,
    background: &'static [TileSetting],
    collision_map: &'static [u8],
    winning_map: &'static [u8],
    spike_map: &'static [u8],
    player_start: (i32, i32),
}

impl Level {
    fn bounds(&self) -> Rect<i32> {
        rect(
            vec2(0, 0),
            vec2(self.width as i32 - 1, self.height as i32 - 1),
        )
    }

    fn collides(&self, tile: Vector2D<i32>) -> bool {
        if !self.bounds().contains_point(tile) {
            return false;
        }

        let idx = (tile.x + tile.y * self.width as i32) as usize;

        self.collision_map[idx / 8] & (1 << (idx % 8)) != 0
    }

    fn win_collision(&self, tile: Vector2D<i32>) -> bool {
        if !self.bounds().contains_point(tile) {
            return false;
        }

        let idx = (tile.x + tile.y * self.width as i32) as usize;

        self.winning_map[idx / 8] & (1 << (idx % 8)) != 0
    }

    fn spike_collision(&self, tile: Vector2D<i32>) -> bool {
        if !self.bounds().contains_point(tile) {
            return false;
        }

        let idx = (tile.x + tile.y * self.width as i32) as usize;

        self.spike_map[idx / 8] & (1 << (idx % 8)) != 0
    }
}

struct World {
    level: &'static Level,
    bg: InfiniteScrolledMap,
    player: Player,
}

impl World {
    fn new(level: &'static Level) -> Self {
        let bg = RegularBackground::new(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        let bg = InfiniteScrolledMap::new(bg);

        World {
            level,
            bg,
            player: Player::new(level.player_start.into()),
        }
    }

    fn set_pos(&mut self, pos: Vector2D<i32>) {
        self.bg.set_scroll_pos(pos, |pos| {
            let tile = if self.level.bounds().contains_point(pos) {
                // Convert 2D coordinates to a 1D index into our flat tile array.
                // This is called row-major indexing.
                let idx = pos.x + pos.y * self.level.width as i32;
                self.level.background[idx as usize]
            } else {
                // Use the transparent tile outside the level bounds.
                // agb may contain specific optimizations around the blank tile.
                TileSetting::BLANK
            };

            (&tiles::TILES.tiles, tile)
        });
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        self.bg.show(frame);
        self.player.show(frame);
    }

    fn has_won(&self) -> bool {
        self.player.has_won(self.level)
    }

    fn spike_hit(&self) -> bool {
        self.player.hits_spike(self.level)
    }

    fn update(&mut self, buttons: &ButtonController) {
        self.set_pos(vec2(0, 0));
        self.player.update(buttons, self.level);
    }
}

struct Player {
    pos: Vector,
    speed: Vector,
    frame: usize,
    sprite: SpriteVram,
    flipped: bool,
}

impl Player {
    fn new(start: Vector2D<i32>) -> Self {
        Player {
            pos: start.change_base(),
            speed: vec2(num!(0), num!(0)),
            frame: 0,
            sprite: sprites::STANDING.sprite(0).into(),
            flipped: false,
        }
    }

    fn show(&self, frame: &mut GraphicsFrame) {
        Object::new(self.sprite.clone())
            .set_hflip(self.flipped)
            .set_pos(self.pos.round() - vec2(8, 8))
            .show(frame);
    }

    fn handle_horizontal_input(&mut self, x_tri: i32, on_ground: bool) {
        let mut x = Number::new(x_tri) / 4;

        if x_tri.signum() != self.speed.x.to_raw().signum() {
            x *= 2;
        }

        if on_ground {
            x *= 2;
        }

        self.speed.x += x / 16;
        self.speed.x = self.speed.x.clamp(num!(-2), num!(2))
    }

    fn handle_jump(&mut self) {
        self.speed.y -= num!(1.5);
    }

    fn update_sprite(&mut self) {
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

    fn is_on_ground(&self, level: &Level) -> bool {
        let check = vec2(self.pos.x, self.pos.y + 8 + Number::from_raw(1));

        level.collides(check.floor() / 8)
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
        Self::handle_collision_component(&mut self.speed.x, &mut self.pos.x, 4, &|x| {
            level.collides(vec2(x, self.pos.y.floor() / 8))
        });
        Self::handle_collision_component(&mut self.speed.y, &mut self.pos.y, 8, &|y| {
            level.collides(vec2(self.pos.x.floor() / 8, y))
        });
    }

    fn has_won(&self, level: &Level) -> bool {
        level.win_collision(self.pos.floor() / 8)
    }

    fn hits_spike(&self, level: &Level) -> bool {
        level.spike_collision(self.pos.floor() / 8)
    }

    fn update(&mut self, buttons: &ButtonController, level: &Level) {
        let on_ground = self.is_on_ground(level);

        self.handle_horizontal_input(buttons.x_tri() as i32, on_ground);

        if buttons.is_just_pressed(Button::A) && on_ground {
            self.handle_jump();
        }

        if (buttons.x_tri() == Tri::Zero
            || (buttons.x_tri() == Tri::Positive && self.speed.x < num!(0))
            || (buttons.x_tri() == Tri::Negative && self.speed.x > num!(0)))
            && on_ground
        {
            self.speed.x *= 14;
            self.speed.x /= 16;
        }

        if buttons.is_pressed(Button::A) {
            self.speed.y += num!(0.04);
        } else {
            self.speed.y += num!(0.1);
        }

        self.handle_collision(level);

        self.update_sprite();
    }
}

// The main function must take 1 arguments and never returns, and must be marked with
// the #[agb::entry] macro.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();
    let mut buttons = ButtonController::new();

    gfx.set_background_palettes(tiles::PALETTES);

    let mut level = 0;
    let mut world = World::new(levels::LEVELS[level]);

    println!("test");
    loop {
        buttons.update();

        world.update(&buttons);

        let mut frame = gfx.frame();

        world.show(&mut frame);

        frame.commit();

        if world.has_won() {
            level += 1;
            level %= levels::LEVELS.len();
            world = World::new(levels::LEVELS[level]);
        }

        if world.spike_hit() {
            world = World::new(levels::LEVELS[level])
        }
    }
}
