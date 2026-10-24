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
        Priority,
        tiled::{RegularBackground, RegularBackgroundSize, TileFormat},
    },
    fixnum::{Num, Vector2D},
    include_aseprite, include_background_gfx, include_wav,
    input::{ButtonController, Tri},
    sound::mixer::{Frequency, SoundChannel, SoundData},
};

use crate::tiles::HEALTH;

// By default no_std crates don't get alloc, so you won't be able to use things like Vec
// until you declare the extern crate. `agb` provides an allocator so it will all work
extern crate alloc;

mod level;
mod player;
mod shim;
mod world;

static WATER_SFX: SoundData = include_wav!("sfx/water.wav");
static JUMP_SFX: SoundData = include_wav!("sfx/jump.wav");
static SWIM_SFX: SoundData = include_wav!("sfx/swim.wav");
static DEATH_SFX: SoundData = include_wav!("sfx/death.wav");

include_background_gfx!(mod tiles,
"03adfc",
TILES => "gfx/tileset.png",
HEALTH => "gfx/health.aseprite"
);

include_aseprite!(mod sprites, "gfx/sprites.aseprite");

type Number = Num<i32, 8>;
type Vector = Vector2D<Number>;

static START_LIVES: usize = 5;
static RESPAWN_TIME: usize = 30;

// The main function must take 1 arguments and never returns, and must be marked with
// the #[agb::entry] macro.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();
    let mut buttons = ButtonController::new();
    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);

    gfx.set_background_palettes(tiles::PALETTES);

    let mut level = 0;
    let mut lives = START_LIVES;
    let mut world = world::World::new(level::levels::LEVELS[level]);
    let mut respawn_timer = 0;

    let mut player_health_background = RegularBackground::new(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    loop {
        buttons.update();
        let mut frame = gfx.frame();

        for i in 0..4 {
            player_health_background.set_tile(
                (i + 1, 0),
                &tiles::HEALTH.tiles,
                tiles::HEALTH.tile_settings[i as usize],
            );
        }

        for i in 0..(START_LIVES as i32) {
            let tile = if i < lives as i32 { 5 } else { 4 };
            player_health_background.set_tile(
                (i + 5, 0),
                &tiles::HEALTH.tiles,
                tiles::HEALTH.tile_settings[tile as usize],
            );
        }

        if respawn_timer == 0 {
            world.update(&buttons, &mut mixer);

            if world.has_won() {
                level += 1;
                level %= level::levels::LEVELS.len();
                if level == 0 {
                    lives = START_LIVES;
                }
                world = world::World::new(level::levels::LEVELS[level]);
            }

            if world.spike_hit() || world.collides() == Tri::Negative {
                mixer.play_sound(SoundChannel::new(DEATH_SFX));
                lives -= 1;
                respawn_timer = RESPAWN_TIME;
                world.player.die();
            }
        } else {
            respawn_timer -= 1;
            if respawn_timer == 0 {
                if lives == 0 {
                    lives = START_LIVES;
                    level = 0;
                }
                world = world::World::new(level::levels::LEVELS[level]);
            }
        }

        player_health_background.show(&mut frame);
        world.show(&mut frame);
        frame.commit();

        mixer.frame();
    }
}
