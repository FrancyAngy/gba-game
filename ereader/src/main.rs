#![no_std]
#![no_main]

use core::ffi::CStr;

mod erapi;

static PALETTE: [u16; 2] = [rgb(0, 0, 0), rgb(255, 255, 255)];
static RAM_END: u32 = 0x0202_7800;

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
    loop {}
}

#[repr(align(4))]
struct Card([u8; 2100]);
static mut CARD: Card = Card([0; 2100]);

const fn rgb(r: u8, g: u8, b: u8) -> u16 {
    (r >> 3) as u16 | ((g >> 3) as u16) << 5 | ((b >> 3) as u16) << 10
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> u32 {
    erapi::init_memory((RAM_END - (&raw const __end) as u32) >> 10);

    erapi::set_background_palette(&PALETTE, 0);

    erapi::set_background_mode(0);
    erapi::load_system_background(3, 30);

    let region = erapi::Region::create(0, 0, 1, 1, 28, 10);

    erapi::fade_in(20);

    region.set_text_color(1, 0);

    region.draw_text(0, 0, c"Test");

    erapi::render_frame(120);

    region.draw_text(0, 8, c"Test 2");

    erapi::render_frame(120);

    region.draw_text(0, 16, c"Press B to return to menu");
    region.draw_text(0, 24, c"Press A to restart");

    loop {
        if erapi::buttons::get_pressed(erapi::buttons::B) {
            return erapi::EXIT_TO_MENU;
        } else if erapi::buttons::get_pressed(erapi::buttons::A) {
            return erapi::EXIT_RESTART;
        }

        erapi::render_frame(1);
    }
}
