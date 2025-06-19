#![no_std] //dont link Rust standard library
#![no_main] //disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

//static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)] //dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ", some numbers: {} {}",
    //     42,
    //     1.337
    // )
    // .unwrap();

    println!("Hello World{}", "!");
    //panic!("Some panic message");
    loop {}
}

///This function is called on Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
