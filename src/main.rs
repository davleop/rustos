// main.rs

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]

#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};

use core::panic::PanicInfo;
use rustos::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustos::memory;
    use x86_64::{
        structures::paging::{Translate, Page},
        VirtAddr
    };

    println!("Hello World{}", "!");
    rustos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

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

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rustos::hlt_loop();
}

/*#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    println!("Hello, World{}", "!");

    rustos::init();

    //

    #[cfg(test)]
    test_main();

    println!("No crashing");
    rustos::hlt_loop();
}*/

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
