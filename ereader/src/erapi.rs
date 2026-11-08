#![allow(dead_code)]
use core::ffi::CStr;

const ERAPI: *const usize = 0x0300_75FC as *const usize;

pub const EXIT_TO_MENU: u32 = 2;
pub const EXIT_RESTART: u32 = 0;

pub mod buttons {
    #![allow(dead_code)]
    pub const A: u32 = 0x0001;
    pub const B: u32 = 0x0002;
    pub const SELECT: u32 = 0x0004;
    pub const START: u32 = 0x0008;
    pub const RIGHT: u32 = 0x0010;
    pub const LEFT: u32 = 0x0020;
    pub const UP: u32 = 0x0040;
    pub const DOWN: u32 = 0x0080;
    pub const L: u32 = 0x0100;
    pub const R: u32 = 0x0200;

    pub fn get_pressed(button: u32) -> bool {
        if (super::get_buttons_raw() & button) == button {
            true
        } else {
            false
        }
    }
}

#[inline(always)]
unsafe fn call1(id: u32) -> u32 {
    let f: extern "C" fn(u32) -> u32 = unsafe { core::mem::transmute(ERAPI.read_volatile()) };
    f(id)
}

#[inline(always)]
unsafe fn call2(id: u32, a: u32) -> u32 {
    let f: extern "C" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(ERAPI.read_volatile()) };
    f(id, a)
}

#[inline(always)]
unsafe fn call3(id: u32, a: u32, b: u32) -> u32 {
    let f: extern "C" fn(u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(ERAPI.read_volatile()) };
    f(id, a, b)
}

#[inline(always)]
unsafe fn call4(id: u32, a: u32, b: u32, c: u32) -> u32 {
    let f: extern "C" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(ERAPI.read_volatile()) };
    f(id, a, b, c)
}

#[inline(always)]
unsafe fn call5(id: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    let f: extern "C" fn(u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(ERAPI.read_volatile()) };
    f(id, a, b, c, d)
}

#[inline(always)]
unsafe fn call6(id: u32, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let f: extern "C" fn(u32, u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(ERAPI.read_volatile()) };
    f(id, a, b, c, d, e)
}

#[inline(always)]
unsafe fn call7(id: u32, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> u32 {
    let func: extern "C" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(ERAPI.read_volatile()) };
    func(id, a, b, c, d, e, f)
}

const fn pack(hi: u8, lo: u8) -> u32 {
    ((hi as u32) << 8) | lo as u32
}

#[derive(Clone, Copy)]
pub struct Region(u8);

impl Region {
    pub fn create(mode: u8, flags: u8, x: u8, y: u8, w: u8, h: u8) -> Region {
        let handle = unsafe { call4(0x290, pack(mode, flags), pack(x, y), pack(w, h)) };
        Region(handle as u8)
    }

    pub fn clear(self) {
        unsafe {
            call2(0x292, self.0 as u32);
        }
    }

    pub fn set_color(self, palette_index: u8) {
        unsafe {
            call3(0x291, self.0 as u32, palette_index as u32);
        }
    }

    pub fn set_text_color(self, text: u8, background: u8) {
        unsafe {
            call3(0x298, self.0 as u32, pack(text, background));
        }
    }

    /// x, y in pixels inside the region.
    pub fn draw_text(self, x: u8, y: u8, text: &CStr) {
        unsafe {
            call4(0x299, self.0 as u32, pack(x, y), text.as_ptr() as u32);
        }
    }
}

pub fn render_frame(count: u32) {
    unsafe {
        call2(0x300, count);
    }
}

pub fn set_background_mode(mode: u32) {
    unsafe {
        call2(0x219, mode);
    }
}

pub fn set_background_palette(colors: &'static [u16], row: u8) {
    unsafe {
        call4(
            0x27E,
            colors.as_ptr() as u32,
            row as u32,
            colors.len() as u32,
        );
    }
}

pub fn load_system_background(layer: u32, index: u32) {
    unsafe {
        call3(0x210, index, layer);
    }
}

pub fn fade_in(frames: u32) {
    unsafe {
        call2(0x200, frames);
    }
}

pub fn fade_out(frames: u32) {
    unsafe {
        call2(0x201, frames);
    }
}

pub fn init_memory(a: u32) {
    unsafe {
        call2(0x2EA, a);
    }
}

pub fn get_buttons_raw() -> u32 {
    unsafe { call1(0x302) }
}

pub fn scan_dot_code(buffer: &mut [u8]) -> u32 {
    unsafe { call2(0x2C2, buffer.as_mut_ptr() as u32) }
}
