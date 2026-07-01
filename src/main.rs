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
    use x86_64::VirtAddr;
    println!("Hello, World!");

    use x86_64::structures::paging::Translate;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { jernel::memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        // new: use the `mapper.translate_addr` method
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
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
