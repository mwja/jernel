#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use jernel::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    jernel::init();

    #[cfg(not(test))]
    main();
    #[cfg(test)]
    test_main();

    jernel::hlt_loop();
}

fn main() {
    println!("Hello, World!");
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jernel::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    jernel::hlt_loop();
}
