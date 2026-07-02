#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use jernel::println;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    jernel::init();

    #[cfg(not(test))]
    main(boot_info);
    #[cfg(test)]
    test_main();

    jernel::hlt_loop();
}

fn main(boot_info: &'static BootInfo) {
    use jernel::memory::frame::BootInfoFrameAllocator;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;
    println!("Hello, World!");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    jernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jernel::test_panic_handler(info)
}
