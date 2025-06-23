#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ssebide_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ssebide_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    ssebide_os::init(); // new

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    //stack_overflow();

    //trigger  a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }

    // invoke a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash ");
    ssebide_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ssebide_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ssebide_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
