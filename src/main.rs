// main.rs

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");

    rustos::init();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    #[cfg(test)]
    test_main();

    println!("No crashing");
    loop{}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn pnaic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1,1);
}
