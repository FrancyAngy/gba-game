use std::env;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use tiled::{
    FilesystemResourceReader, FiniteTileLayer, Layer, Map, ObjectLayer, PropertyValue,
    ResourceReader, TileLayer,
};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

struct BuildResourceReader;

static LEVELS: &[&str] = &["level_01.tmx", "level_02.tmx"];

fn main() -> Result<(), Box<dyn Error>> {
    build_cpp_lib();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_file_name = format!("{out_dir}/levels.rs");
    let mut file = std::fs::File::create(out_file_name)?;

    for (number, level) in LEVELS.iter().enumerate() {
        let ident = quote::format_ident!("LEVEL_{}", number);
        let level = import_level(&format!("tiled/{level}"))?;
        let content = quote! {
            static #ident: Level = #level;
        };
        writeln!(file, "{content}")?;
    }

    let levels = (0..LEVELS.len()).map(|x| quote::format_ident!("LEVEL_{}", x));
    writeln!(
        file,
        "{}",
        quote! {
            pub static LEVELS: &[&Level] = &[#(&#levels),*];
        }
    )?;

    Ok(())
}

trait GetLayer {
    fn get_layer_by_name(&self, name: &str) -> Layer<'_>;
    fn get_tile_layer(&self, name: &str) -> FiniteTileLayer<'_>;
    fn get_object_layer(&self, name: &str) -> ObjectLayer<'_>;
}

impl GetLayer for Map {
    fn get_layer_by_name(&self, name: &str) -> Layer<'_> {
        self.layers().find(|x| x.name == name).unwrap()
    }
    fn get_tile_layer(&self, name: &str) -> FiniteTileLayer<'_> {
        match self.get_layer_by_name(name).as_tile_layer().unwrap() {
            TileLayer::Finite(finite_tile_layer) => finite_tile_layer,
            TileLayer::Infinite(_) => panic!("Infinite tile layer not supported"),
        }
    }

    fn get_object_layer(&self, name: &str) -> ObjectLayer<'_> {
        self.get_layer_by_name(name).as_object_layer().unwrap()
    }
}

impl ResourceReader for BuildResourceReader {
    type Resource = <FilesystemResourceReader as ResourceReader>::Resource;

    type Error = <FilesystemResourceReader as ResourceReader>::Error;

    fn read_from(
        &mut self,
        path: &std::path::Path,
    ) -> std::result::Result<Self::Resource, Self::Error> {
        println!("cargo::rerun-if-changed={}", path.to_string_lossy());
        FilesystemResourceReader.read_from(path)
    }
}

#[derive(Debug, Clone, Copy)]
struct TileInfo {
    id: Option<u32>,
    colliding: bool,
    win: bool,
    spike: bool,
}

#[derive(Debug, Clone)]
struct Level {
    size: (u32, u32),
    tiles: Vec<TileInfo>,
    player_start: (i32, i32),
}

impl ToTokens for Level {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let background_tiles = self.tiles.iter().map(|x| match x.id {
            // if the tile is defined, look up the TileSetting in the background tiles
            Some(x) => quote! { TILES[#x as usize] },
            // otherwise, use the blank tile
            None => quote! { TileSetting::BLANK },
        });

        // this creates the bit-array from the booleans on the tiles.
        let collision_map = self.tiles.chunks(8).map(|x| {
            x.iter()
                .map(|x| x.colliding as u8)
                .fold(0u8, |a, b| (a >> 1) | (b << 7))
        });

        let winning_map = self.tiles.chunks(8).map(|x| {
            x.iter()
                .map(|x| x.win as u8)
                .fold(0u8, |a, b| (a >> 1) | (b << 7))
        });

        let spike_map = self.tiles.chunks(8).map(|x| {
            x.iter()
                .map(|x| x.spike as u8)
                .fold(0u8, |a, b| (a >> 1) | (b << 7))
        });

        let (player_x, player_y) = self.player_start;
        let (width, height) = self.size;

        quote! {
            Level {
                width: #width,
                height: #height,
                background: &[#(#background_tiles),*],
                collision_map: &[#(#collision_map),*],
                winning_map: &[#(#winning_map),*],
                spike_map: &[#(#spike_map),*],
                player_start: (#player_x, #player_y),
            }
        }
        .to_tokens(tokens)
    }
}

fn import_level(level: &str) -> Result<Level, Box<dyn Error>> {
    let level = tiled::Loader::with_reader(BuildResourceReader).load_tmx_map(level)?;

    let map = level.get_tile_layer("Level");
    let objs = level.get_object_layer("Objects");

    let width = map.width();
    let height = map.height();

    let mut tiles = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let tile = match map.get_tile(x as i32, y as i32) {
                Some(tile) => {
                    let properties = &tile.get_tile().unwrap().properties;

                    let mut colliding = false;
                    if properties.contains_key("COLLISION") {
                        colliding = properties["COLLISION"] == PropertyValue::BoolValue(true);
                    }

                    let mut win = false;
                    if properties.contains_key("WIN") {
                        win = properties["WIN"] == PropertyValue::BoolValue(true);
                    }

                    let mut spike = false;
                    if properties.contains_key("SPIKE") {
                        spike = properties["SPIKE"] == PropertyValue::BoolValue(true);
                    }

                    TileInfo {
                        colliding,
                        win,
                        spike,
                        id: Some(tile.id()),
                    }
                }
                None => TileInfo {
                    colliding: false,
                    win: false,
                    spike: false,
                    id: None,
                },
            };

            tiles.push(tile);
        }
    }

    let player = objs
        .objects()
        .find(|x| x.name == "PLAYER")
        .expect("Should be able to find the player");

    let player_x = player.x as i32;
    let player_y = player.y as i32;

    Ok(Level {
        size: (width, height),
        tiles,
        player_start: (player_x, player_y),
    })
}

fn build_cpp_lib() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cpp_libs_dir = manifest_dir.join("cpp_libs");
    let link_lib = cpp_libs_dir.join("gba-link-connection").join("lib");
    let shim = cpp_libs_dir.join("shim.cpp");

    cc::Build::new()
        .cpp(true)
        .compiler("arm-none-eabi-g++")
        .archiver("arm-none-eabi-ar")
        .cpp_link_stdlib(None)
        .std("c++17")
        .opt_level(2)
        .flag("-mthumb")
        .flag("-mthumb-interwork")
        .flag("-mcpu=arm7tdmi")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-fno-threadsafe-statics")
        .flag("-fno-use-cxa-atexit")
        .include(&link_lib)
        .file(&shim)
        .compile("shim");

    println!("cargo:rerun-if-changed={}", shim.display());
    println!("cargo:rerun-if-changed={}", link_lib.display());
}
