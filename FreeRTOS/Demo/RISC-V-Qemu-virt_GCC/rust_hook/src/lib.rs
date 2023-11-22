#![allow(non_snake_case)]
#![no_std]
use panic_halt as _;

include!("hook.rs");
#[no_mangle]
fn hook_vTaskStartScheduler() {
    unsafe{
        vSendString("hook_vTaskStartScheduler\n\0".as_ptr() as *const u8);
    }
}
