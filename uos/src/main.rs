#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(uos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use uos::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use uos::allocator;
    use uos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello Unimplemented OS");

    // 初始化中断
    uos::init();

    // 初始化分页
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // 初始化allocator
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");


    #[cfg(test)]
    test_main();
    println!("It did not crash!");

    uos::print!(" ~ ");

    uos::hlt_loop();
}

/* 测试部分 */

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    uos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uos::test_panic_handler(info)
}
