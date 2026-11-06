#[allow(dead_code)]
unsafe extern "C" {
    fn shim_card_init();
    fn shim_card_get_device(cancel: extern "C" fn() -> bool) -> i32;
    fn shim_card_send_loader(loader: *const u8, size: u32, cancel: extern "C" fn() -> bool) -> i32;
    fn shim_card_receive(buf: *mut u8, cancel: extern "C" fn() -> bool) -> i32;
}
