/// Raspberry Pi4 in Low Peripherals Level

use crate::board::*;

// Mini Uart MMIO Address
/// Auxiliary Interrupt status
pub const AUX_IRQ: usize = PERIPHERAL_BASE + 0x215000;
/// Auxiliary enables
pub const AUX_ENABLES: usize = AUX_IRQ + 0x4;
/// Mini UART I/O Data
pub const AUX_MU_IO_REG: usize = AUX_IRQ + 0x40;
/// Mini UART Interrupt Enable
pub const AUX_MU_IER_REG: usize = AUX_IRQ + 0x44;
/// Mini UART Interrupt Identify
pub const AUX_MU_IIR_REG: usize = AUX_IRQ + 0x48;
/// Mini UART Line Control
pub const AUX_MU_LCR_REG: usize = AUX_IRQ + 0x4c;
/// Mini UART Modem Control
pub const AUX_MU_MCR_REG: usize = AUX_IRQ + 0x50;
/// Mini UART Line Status
pub const AUX_MU_LSR_REG: usize = AUX_IRQ + 0x54;
/// Mini UART Modem Status
pub const AUX_MU_MSR_REG: usize = AUX_IRQ + 0x58;
/// Mini UART Scratch
pub const AUX_MU_SCRATCH: usize = AUX_IRQ + 0x5c;
/// Mini UART Extra Control
pub const AUX_MU_CNTL_REG: usize = AUX_IRQ + 0x60;
/// Mini UART Extra Status
pub const AUX_MU_STAT_REG: usize = AUX_IRQ + 0x64;
/// Mini UART Baudrate
pub const AUX_MU_BAUD_REG: usize = AUX_IRQ + 0x68;

// GPIO
pub const GPFSEL0: usize = PERIPHERAL_BASE + 0x200000;
pub const GPSET0: usize  = PERIPHERAL_BASE + 0x20001C;
pub const GPCLR0: usize  = PERIPHERAL_BASE + 0x200028;
pub const GPPUPPDN0: usize =  PERIPHERAL_BASE + 0x2000E4;



/// Convert physical address to bus address (ref: peripherals page 6)
#[inline]
pub const fn phys_to_bus(paddr: u32) -> u32 {
    paddr | 0xC0000000
}

/// Convert physical address to bus address (ref: peripherals page 6)
#[inline]
pub const fn bus_to_phys(baddr: u32) -> u32 {
    baddr & !0xC0000000
}

/// Convert I/O peripherals address to bus address (ref: peripherals page 6)
#[inline]
pub const fn io_to_bus(paddr: u32) -> u32 {
    (paddr & 0xFFFFFF) | 0x7E000000
}

/// Convert bus address to I/O peripherals address (ref: peripherals page 6)
#[inline]
pub const fn bus_to_io(baddr: u32) -> u32 {
    (baddr & 0xFFFFFF) | 0x3F000000
}