#![no_std] // 不使用标准库，因为标准库中有许多与系统相关的东西
#![no_main] // 禁止所有 rust 入口点

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World";

/// 这个方法会 panic 的时候会调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle] // no_mangle 不会对 _start 进行修改
pub extern "C" fn _start() -> ! {  // 此函数是入口点，因为链接器会查找函数
    vga_buffer::print_something();

    loop {}
}