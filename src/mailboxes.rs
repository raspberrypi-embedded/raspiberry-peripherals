
use crate::board::*;

/// The base address for the `MU` registers.
const MAILBOX_BASE: usize = PERIPHERAL_BASE + 0xB000 + 0x880;

pub const MAILBOX_RESPONSE: u32 = 1 << 31;
pub const MAILBOX_REQUEST: u32 = 0;

/// Available mailbox channels
///
/// (ref: https://github.com/raspberrypi/firmware/wiki/Mailboxes)
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum MailboxChannel {
    Framebuffer = 1,
    Property = 8,
}

#[repr(u32)]
pub enum MailboxTag {
    SetPower = 0x28001,
    SetClkRate = 0x38002,
    SetPhyWH = 0x48003,
    SetVirtWH = 0x48004,
    SetVirtOff = 0x48009,
    SetDepth = 0x48005,
    SetPixelOrder = 0x48006,
    GetFB = 0x40001,
    GetPitch = 0x40008,

    TagLast = 0
}

/// Read from mailbox status register (MAILx_STA).
#[repr(u32)]
pub enum MailboxStatus {
    MailboxEmpty = 1 << 30,
    MailboxFull = 1 << 31,
}

/// Mailbox registers. We basically only support mailbox 0 & 1. We
/// deliver to the VC in mailbox 1, it delivers to us in mailbox 0. See
/// BCM2835-ARM-Peripherals.pdf section 1.3 for an explanation about
/// the placement of memory barriers.
///
/// (ref: https://github.com/raspberrypi/firmware/wiki/Mailboxes)
#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    MBOX_READ: u32,
    __reserved0: [u32; 3],
    MBOX_POLL: u32,
    MBOX_SENDER: u32,
    MBOX_STATUS: u32,
    MBOX_CONFIG: u32,
    MBOX_WRITE: u32
}

#[repr(align(16))]
pub struct MailBoxBuffer {
    buf: [u32; 36]
}

impl MailBoxBuffer {
    pub fn write(&mut self, index: usize, val: u32) {
        self.buf[index] = val;
    }

    pub fn read(&self, index: usize) -> u32 {
        self.buf[index]
    }
}

/// The Raspberry Pi's mailbox.
///
/// (ref: https://github.com/raspberrypi/firmware/wiki/Accessing-mailboxes)
pub struct MailBox {
    registers: &'static mut Registers,
    buf: MailBoxBuffer
}

impl MailBox {
    /// Returns a new instance of `Mailbox`.
    #[inline]
    #[no_mangle]
    pub fn new() -> MailBox {
        MailBox {
            registers: unsafe { &mut *(MAILBOX_BASE as *mut Registers) },
            buf: MailBoxBuffer{ buf: [0u32; 36] }
        }
    }

    /// Check if mailbox is readable
    fn readable(&self) -> bool {
        // return mmio_read(self.registers.MBOX_STATUS as usize) != (MailboxStatus::MailboxEmpty as u32)
        return self.registers.MBOX_STATUS != (MailboxStatus::MailboxEmpty as u32)
    }

    /// Check if mailbox is writeable
    fn writeable(&self) -> bool {
        // return mmio_read(self.registers.MBOX_WRITE as usize) != (MailboxStatus::MailboxFull as u32)
        return self.registers.MBOX_STATUS != (MailboxStatus::MailboxFull as u32)
    }

    pub fn write_buf(&mut self, index: usize, val: u32) {
        self.buf.write(index, val)
    }

    pub fn read_buf(&self, index: usize) -> u32 {
        self.buf.read(index)
    }

    pub fn addr(&self) -> usize {
        &self.buf.buf[0] as *const _ as usize
    }

    /// Implement from https://github.com/isometimes/rpi4-osdev/blob/master/part5-framebuffer/mb.c
    pub fn call(&mut self, mbox_channel: MailboxChannel) -> Result<(), ()> {
        // 28-bit address (MSB) and 4-bit value (LSB)
        let buf_addr = &self.buf as *const _  as u32;
        let chan = mbox_channel as u32;
        let data = buf_addr + chan; 
        loop{
            // Wait until mailbox is writeable
            if self.writeable(){ break }
        }

        // Write the address of our buffer to the mailbox with the channel appended
        // mmio_write(self.registers.MBOX_WRITE as usize, data);
        self.registers.MBOX_WRITE = data;
        loop {
            loop {
                if self.readable(){ break; }
            }
            if self.registers.MBOX_READ == data {
                if self.buf.buf[1] == MAILBOX_RESPONSE { 
                    return Ok(())
                }
                else{ return Err(()) }
            }
        }
    }
}
