#![no_std] // 不使用标准库，因为标准库中有许多与系统相关的东西
#![no_main] // 禁止所有 rust 入口点
#![feature(custom_test_frameworks)] // 使用自定义的 test 套件
#![test_runner(blog_os::test_runner)] // 指定测试的工具
#![reexport_test_harness_main = "test_main"] // 指定测试的入口

use blog_os::println; // 抽象到 lib 中去 share
use core::panic::PanicInfo;

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
    blog_os::test_panic_handler(info)
}


#[no_mangle] // no_mangle 不会对 _start 进行修改
pub extern "C" fn _start() -> ! {  // 此函数是入口点，因为链接器会查找函数
    #[cfg(test)]
        test_main();
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}