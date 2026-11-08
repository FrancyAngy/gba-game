#![allow(dead_code)]

pub const CARD_OFFSET: usize = 0x4e;
pub const CARD_END: usize = 2076;
pub const SCAN_SUCCESS: u32 = 6;

pub const HANDSHAKE_1: u16 = 0xfbfb;
pub const HANDSHAKE_2: u16 = 0x5841;
pub const HANDSHAKE_3: u16 = 0x4534;

pub const GAME_REQUEST: u16 = 0xecec;

pub const GAME_ANIMATING: u16 = 0xf3f3;
pub const EREADER_ANIMATING: u16 = 0xf2f2;
pub const EREADER_READY: u16 = 0xf1f1;

pub const GAME_READY: u16 = 0xefef;

pub const EREADER_CANCEL: u16 = 0xf7f7;
pub const EREADER_SEND_READY: u16 = 0xf9f9;
pub const GAME_RECEIVE_READY: u16 = 0xfefe;

pub const EREADER_SEND_START: u16 = 0xfdfd;
pub const EREADER_SEND_END: u16 = 0xfcfc;

pub const GAME_RECEIVE_OK: u16 = 0xf5f5;
pub const GAME_RECEIVE_ERROR: u16 = 0xf4f4;

pub const EREADER_SIO_END: u16 = 0xf3f3;
pub const GAME_SIO_END: u16 = 0xf1f1;
