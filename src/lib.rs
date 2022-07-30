#![no_std]

// extern crate print;
// extern crate bit_field;
#[macro_use]
pub mod mini_uart;
pub mod addr;
pub mod gpio;
pub mod mailboxes;
mod console;

#[cfg(feature = "raspi3")]
#[path = "boards/raspi3.rs"]
mod board;

#[cfg(feature = "raspi4")]
#[path = "boards/raspi4.rs"]
mod board;

use core::ptr::{ read, write };
pub use board::*;


/// Read something from mmio address
pub fn mmio_read(addr: usize) -> u32 {
    assert!(addr >= PERIPHERAL_BASE);
    assert!(addr <= PERIPHERAL_END);
    unsafe{
        let val = read(addr as *const u32);
        return val
    }
}

/// Write something to mmio address
pub fn mmio_write(addr: usize, val: u32) {
    assert!(addr >= PERIPHERAL_BASE);
    assert!(addr <= PERIPHERAL_END);
    unsafe{
        write(addr as *mut u32, val);
    }
}