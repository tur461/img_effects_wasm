#![no_std]
#![no_main]
#![feature(lang_items)]

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}

#[allow(unused_attributes)]
#[no_mangle]
pub extern "C" fn _start() -> ! { 
    loop{} 
}
