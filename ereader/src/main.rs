#![no_std]
#![no_main]

use core::ffi::CStr;

use crate::{
    erapi::{Region, buttons},
    protocol::{CARD_END, CARD_OFFSET, HANDSHAKE_1, HANDSHAKE_2, HANDSHAKE_3},
};

mod erapi;
mod link;
mod protocol;

static PALETTE: [u16; 2] = [rgb(0, 0, 0), rgb(255, 255, 255)];
const RAM_END: u32 = 0x0202_7800;

const CARD_BUFFER_SIZE: usize = 2100;

const MSG_CANCEL: &'static CStr = c"Press B to cancel";

core::arch::global_asm!(
    ".syntax unified",
    ".section .text._start,\"ax\"",
    ".global _start",
    ".arm",
    "_start:",
    "    ldr  r0, =(_start_thumb+1)",
    "    bx   r0",
    "    .pool",
    ".thumb",
    "_start_thumb:",
    "    push {{lr}}",
    "    ldr  r0, =__bss_start",
    "    ldr  r1, =__bss_end",
    "    movs r2, #0",
    "1:  cmp  r0, r1",
    "    beq  2f",
    "    strb r2, [r0]",
    "    adds r0, #1",
    "    b    1b",
    "2:  pop  {{r3}}",
    "    mov  lr, r3",
    "    ldr  r3, =main",
    "    bx   r3",
    "    .align 2",
    "    .pool",
);

unsafe extern "C" {
    static __end: u8;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    erapi::fade_out(1);
    erapi::set_background_palette(&PALETTE, 0);
    let region = Region::create(0, 0, 0, 0, 240, 160);

    region.set_color(1);
    print_clean(c"Panic", false, &region);

    loop {}
}

#[repr(align(4))]
struct Card([u8; CARD_BUFFER_SIZE]);
static mut CARD: Card = Card([0; CARD_BUFFER_SIZE]);

const fn rgb(r: u8, g: u8, b: u8) -> u16 {
    (r >> 3) as u16 | ((g >> 3) as u16) << 5 | ((b >> 3) as u16) << 10
}

pub struct U8Str([u8; 4]);

impl U8Str {
    pub fn new(mut n: u8) -> Self {
        let mut buf = [0u8; 4]; // zeroed, so the byte after the digits is the NUL
        let mut len = 0;
        let mut started = false; // skip leading zeros
        for unit in [100u8, 10, 1] {
            let mut digit = 0u8;
            while n >= unit {
                // repeated subtraction, no division
                n -= unit;
                digit += 1;
            }
            if digit != 0 || started || unit == 1 {
                // always emit the last digit, so 0 -> "0"
                buf[len] = b'0' + digit;
                len += 1;
                started = true;
            }
        }
        U8Str(buf)
    }

    pub fn as_cstr(&self) -> &CStr {
        CStr::from_bytes_until_nul(&self.0).unwrap_or(c"?")
    }
}

pub struct CBuf<const N: usize> {
    buf: [u8; N],
    len: usize, // bytes used; the byte at buf[len] is always 0
}

impl<const N: usize> CBuf<N> {
    pub const fn new() -> Self {
        CBuf {
            buf: [0; N],
            len: 0,
        }
    }

    /// Appends as much of `src` as fits, always leaving room for the NUL.
    pub fn push_bytes(&mut self, src: &[u8]) {
        let room = N.saturating_sub(1).saturating_sub(self.len);
        let n = src.len().min(room);
        if let Some(dst) = self.buf.get_mut(self.len..self.len + n) {
            dst.copy_from_slice(&src[..n]);
            self.len += n;
        }
    }

    pub fn push_cstr(&mut self, s: &CStr) {
        self.push_bytes(s.to_bytes()); // to_bytes() excludes the NUL
    }

    pub fn as_cstr(&self) -> &CStr {
        CStr::from_bytes_until_nul(&self.buf).unwrap_or(c"?")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> u32 {
    erapi::init_memory((RAM_END - (&raw const __end) as u32) >> 10);

    let card: &mut [u8; CARD_BUFFER_SIZE] = unsafe { &mut (*(&raw mut CARD)).0 };

    reset(card);

    erapi::set_background_palette(&PALETTE, 0);

    erapi::set_background_mode(0);
    erapi::load_system_background(3, 30);

    let region = erapi::Region::create(0, 0, 1, 1, 28, 10);

    erapi::fade_in(20);
    erapi::render_frame(30);

    region.set_text_color(1, 0);

    erapi::render_frame(1);

    let mut cancel = || buttons::get_pressed(erapi::buttons::B);

    print_clean(c"Test", true, &region);

    while handshake(&mut cancel) && !buttons::get_pressed(erapi::buttons::B) {}

    if link::send_receive(HANDSHAKE_3, HANDSHAKE_3, &mut cancel) != protocol::GAME_REQUEST {
        error(1, &region);
        return erapi::EXIT_TO_MENU;
    }

    while !link::send_expect(
        protocol::GAME_ANIMATING,
        protocol::EREADER_ANIMATING,
        &mut cancel,
    ) {}

    while !link::send(protocol::EREADER_ANIMATING, &mut cancel) {}

    if !link::send_expect(protocol::EREADER_READY, protocol::GAME_READY, &mut cancel) {
        error(2, &region);
        return erapi::EXIT_TO_MENU;
    }

    if erapi::scan_dot_code(card) != protocol::SCAN_SUCCESS {
        error(3, &region);
    }

    print_clean(c"Sending", true, &region);

    if !link::send_expect(
        protocol::EREADER_SEND_READY,
        protocol::GAME_RECEIVE_READY,
        &mut cancel,
    ) {
        error(4, &region);
        return erapi::EXIT_TO_MENU;
    }

    if !link::send(protocol::EREADER_SEND_START, &mut cancel) {
        error(5, &region);
        return erapi::EXIT_TO_MENU;
    }

    let mut checksum: u32 = 0;

    let payload = &card[CARD_OFFSET..CARD_END];
    for pair in payload.chunks_exact(2) {
        let word = u16::from_le_bytes([pair[0], pair[1]]);
        if !link::send(word, &mut cancel) {
            error(6, &region);
            return erapi::EXIT_TO_MENU;
        }
        checksum += word as u32;
    }

    if !link::send((checksum & 0xffff) as u16, &mut cancel) {
        error(7, &region);
        return erapi::EXIT_TO_MENU;
    }

    if !link::send((checksum >> 16) as u16, &mut cancel) {
        error(8, &region);
        return erapi::EXIT_TO_MENU;
    }

    if !link::send(protocol::EREADER_SEND_END, &mut cancel) {
        error(9, &region);
        return erapi::EXIT_TO_MENU;
    }

    erapi::EXIT_TO_MENU
}

fn handshake(cancel: &mut impl FnMut() -> bool) -> bool {
    if !link::send_expect(HANDSHAKE_1, HANDSHAKE_1, cancel) {
        return true;
    }

    if !link::send_expect(HANDSHAKE_2, HANDSHAKE_2, cancel) {
        return true;
    }

    if !link::send_expect(HANDSHAKE_3, HANDSHAKE_3, cancel) {
        return true;
    }

    false
}

fn reset(card: &mut [u8; CARD_BUFFER_SIZE]) {
    link::set_general_purpose_mode();
    link::set_multiplay_mode(3); // 115200 bps

    card.fill(0);
}

fn error(code: u8, region: &Region) {
    let mut msg = CBuf::<32>::new();

    msg.push_cstr(c"Error. Code: ");
    msg.push_cstr(U8Str::new(code).as_cstr());
    print_clean(msg.as_cstr(), false, &region);
    erapi::render_frame(60);
}

fn print_clean(text: &CStr, can_cancel: bool, region: &Region) {
    region.clear();
    region.draw_text(0, 0, text);

    if can_cancel {
        region.draw_text(0, 8, MSG_CANCEL);
    }

    erapi::render_frame(1);
}
