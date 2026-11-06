#![no_std]
#![no_main]

mod erapi;

static PALETTE: [u16; 2] = [0xFFFF, 0x0000];

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
    let region = erapi::Region::create(0, 0, 1, 1, 28, 10);

    erapi::fade_in(20);

    erapi::set_background_palette(&PALETTE, 0);
    region.set_text_color(1, 0);

    region.draw_text(0, 0, c"Test");

    erapi::render_frame(240);

    erapi::fade_out(20);

    erapi::EXIT_TO_MENU
}
