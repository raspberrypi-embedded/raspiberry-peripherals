use crate::{ addr, mmio_read, mmio_write };
use core::char;


/// Check if mini uart is writeable
pub fn uart_write_byte_ready() -> bool {
    return (mmio_read(addr::AUX_MU_LSR_REG) & 0x20) > 0
}

/// Check if mini uart is readable
pub fn uart_read_byte_ready() -> bool {
    return (mmio_read(addr::AUX_MU_LSR_REG) & 0x1) > 0
}

/// Write char to uart
pub fn uart_write_char(c: char) {
    loop {
        if uart_write_byte_ready() { break }
    }
    if c == '\n' { 
        mmio_write(addr::AUX_MU_IO_REG, '\r' as u32);
        loop{
            if uart_write_byte_ready(){ break; }
        }
        // uart_write_char('\r');
    }
    mmio_write(addr::AUX_MU_IO_REG, c as u32);
}

/// Read something from mini uart
pub fn uart_read_char() -> char {
    loop {
        if uart_read_byte_ready(){ break }
    }
    let c = char::from_u32(mmio_read(addr::AUX_MU_IO_REG)).unwrap();
    c
}

/// Write String to uart by call `uart_write_char()`
pub fn uart_write_text(buf: &str) {
    for c in buf.chars() {
        // if c == '\n' { uart_write_char('\r') }
        uart_write_char(c)
    }
}

