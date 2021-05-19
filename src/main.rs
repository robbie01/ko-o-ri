#![no_std]
#![no_main]

use core::panic::PanicInfo;
use stivale2::*;
use x86_64::instructions::port::{Port};

static STACK: [u8; 4096] = [0; 4096];

static TERMINAL_TAG: header::Terminal = header::Terminal::new();

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: Header = Header::new(Some(entry_porzint), &STACK[4095] as *const u8 as *const (), HeaderFlags::empty(), &TERMINAL_TAG);

extern "sysv64" fn entry_porzint(_structure: *const Structure) -> ! {
    let mut com1: Port<u8> = Port::new(0x3F8);
    for &char in b"Hello, world!".iter() {
        unsafe {
            com1.write(char);
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_infos: &PanicInfo) -> ! {
    loop {}
}
