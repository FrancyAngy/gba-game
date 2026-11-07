#![allow(dead_code)]

use crate::protocol::HANDSHAKE_1;

const BASE_MEM: usize = 0x04000000;

#[derive(Clone, Copy)]
struct Reg16(*mut u16);

impl Reg16 {
    const fn at(addr: usize) -> Self {
        Reg16(addr as *mut u16)
    }
    #[inline(always)]
    fn read(self) -> u16 {
        unsafe { self.0.read_volatile() }
    }
    #[inline(always)]
    fn write(self, v: u16) {
        unsafe { self.0.write_volatile(v) }
    }
    #[inline(always)]
    fn modify(self, f: impl FnOnce(u16) -> u16) {
        self.write(f(self.read()))
    }
}

const RCNT: Reg16 = Reg16::at(BASE_MEM + 0x0134);
const SIOCNT: Reg16 = Reg16::at(BASE_MEM + 0x0128);
const SIOMLT_RECV: Reg16 = Reg16::at(BASE_MEM + 0x0120);
const SIOMLT_SEND: Reg16 = Reg16::at(BASE_MEM + 0x012A);

const START: u16 = 1 << 7;
const READY: u16 = 1 << 3;
const ERROR: u16 = 1 << 6;
const MULTIPLAYER: u16 = 1 << 13;
const GENERAL_PURPOSE_HIGH: u16 = 1 << 15;

const DISCONNECTED: u16 = 0xFFFF;

const PLAYER_ID_SHIFT: u16 = 4;
const PLAYER_ID_MASK: u16 = 0b11 << PLAYER_ID_SHIFT;

pub fn set_general_purpose_mode() {
    RCNT.write(GENERAL_PURPOSE_HIGH);
    SIOCNT.write(0);
}

#[inline(always)]
pub fn set_multiplay_mode(baudrate: u16) {
    RCNT.write(0);
    SIOCNT.write(MULTIPLAYER | baudrate);
    SIOMLT_SEND.write(0);
}

#[inline(always)]
pub fn set_data(data: u16) {
    SIOMLT_SEND.write(data);
}

#[inline(always)]
pub fn get_data() -> u16 {
    SIOMLT_RECV.read()
}

#[inline(always)]
pub fn start_transfer() {
    SIOCNT.modify(|v| v | START);
}

fn stop_transfer() {
    SIOCNT.modify(|v| v & !START);
}

#[inline(always)]
pub fn is_sending() -> bool {
    SIOCNT.read() & START != 0
}

#[inline(always)]
pub fn transfer_ok() -> bool {
    let ready = SIOCNT.read() & READY != 0;
    let error = SIOCNT.read() & ERROR != 0;

    ready && !error
}

#[inline(always)]
fn player_id() -> u16 {
    (SIOCNT.read() & PLAYER_ID_MASK) >> PLAYER_ID_SHIFT
}

pub fn send(data: u16, cancel: &mut impl FnMut() -> bool) -> bool {
    set_data(data);
    start_transfer();

    while is_sending() {
        if cancel() {
            stop_transfer();
            return false;
        }
    }

    if !transfer_ok() || player_id() != 1 {
        false
    } else {
        true
    }
}

pub fn send_expect(data: u16, expected: u16, cancel: &mut impl FnMut() -> bool) -> bool {
    let mut received: u16;
    loop {
        let mut sent: bool = false;
        for _ in 0..3 {
            if send(data, cancel) {
                sent = true;
                break;
            }
        }

        if !sent {
            return false;
        }

        received = get_data();

        if received == 0 && expected != HANDSHAKE_1 {
            return false;
        }

        if received == expected {
            return true;
        }
    }
}

pub fn send_receive(data: u16, not_expected: u16, cancel: &mut impl FnMut() -> bool) -> u16 {
    let mut received: u16;

    while {
        if !send(data, cancel) {
            return DISCONNECTED;
        }

        received = get_data();

        received == not_expected
    } {}

    received
}
