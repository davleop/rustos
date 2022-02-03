// Runtime Attributes
#![no_std]
#![no_main]

// Testing Attributes
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

// build-std
// Allocations and Collections
extern crate alloc;
use alloc::{
    boxed::Box,
    vec::Vec,
    rc::Rc
};
// panic type...
use core::panic::PanicInfo;

// Using this bootloader library as of now
// will add custom bootloader later...
use bootloader::{
    BootInfo,
    entry_point
};

// local libs
use rustos::task::{
    println,
    Task,
    executor::Executor,
    keyboard
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustos::{
        memory::{
            self,
            BootInfoFrameAllocator
        },
        allocator
    };
    use x86_64::{
        structures::paging::{
            Translate,
            Page
        },
        VirtAddr
    };

    // our first print with self-defined macro
    println!("Hello World{}", "!");

    // initialize system
    rustos::init();

    // Here, we want to get the physical memory offset so that we can
    // initialize memory mapper and the frame allocator
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    
    // setup testing main function
    #[cfg(test)]
    test_main();

    // run executor (multitasking)
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

/// Called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
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

