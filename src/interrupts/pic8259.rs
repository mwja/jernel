use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::InterruptStackFrame;

use crate::print;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init_pics() {
    unsafe {
        PICS.lock().initialize();
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn load_to_idt(idt: &mut x86_64::structures::idt::InterruptDescriptorTable) {
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    send_eoi(InterruptIndex::Timer);
}

/// Send end of interrupt signal
pub fn send_eoi(index: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(index.as_u8());
    }
}
