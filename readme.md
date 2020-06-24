(WIP) a toy&study operating system written by rust run on qeum (x86_64)

## 目录结构

```
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   readme.md
│   rust-toolchain  // 指定 rust 使用每夜版
│   x86_64-blog_os.json // rust 编译 target
│
├───.cargo
│       config // cargo 配置
├───src
│       gdt.rs // gdt 表 段内存
│       interrupts.rs // 中断
│       lib.rs // share lib
│       main.rs 
│       serial.rs // 串口与宿主机通信
│       vga_buffer.rs // vga 显示
```

## TODO

- [x] 显示
- [x] 串口与宿主机通信
- [x] 中断
- [ ] 内存分配
- [ ] 进程、线程
- [ ] ...

## 参考

- https://os.phil-opp.com/
- https://rcore-os.github.io/rCore_tutorial_doc/os2atc2019/os2atc.html
- https://rcore.gitbook.io/rust-os-docs/
- https://rcore-os.github.io/rCore_tutorial_doc/
 