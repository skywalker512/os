#![no_std] // 不使用标准库，因为标准库中有许多与系统相关的东西
#![no_main] // 禁止所有 rust 入口点
#![feature(custom_test_frameworks)] // 使用自定义的 test 套件
#![test_runner(crate::test_runner)] // 指定测试的工具
#![reexport_test_harness_main = "test_main"] // 指定测试的入口

use core::panic::PanicInfo;
mod serial;

mod vga_buffer;

/// 这个方法会 panic 的时候会调用
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// 测试时的 panic
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


#[no_mangle] // no_mangle 不会对 _start 进行修改
pub extern "C" fn _start() -> ! {  // 此函数是入口点，因为链接器会查找函数
    println!("Hello World{}", "!");

    #[cfg(test)]
        test_main();
    loop {}
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 2);
    serial_println!("[ok]");
}

#[cfg(test)] // 条件编译只有测试的时候才会编译
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // 0xf4 是 package.metadata.bootimage 中指定的
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
