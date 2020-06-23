use volatile::Volatile;
use core::fmt;

// ----- 颜色结构定义 -----

#[allow(dead_code)] // 解除未使用的警告
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // 使其具有打印比较等功能
#[repr(u8)] // 用无符号8位存储，应该为4位但rust不支持，后面有hack
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // 不会改变其内存结构
struct ColorCode(u8); // 4 前景 4 背景

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8)) // back + fore 拼接起来了
    }
}

// ----- 整个文字结构定义 -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // 使用兼容C的内存布局
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// 整个 VGA 的大小
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    // Volatile 让编译器不会优化未使用的东西
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// ----- 写入工具 -----

pub struct Writer {
    //  写入器总是写到最后一行，当一行满时(或在\n上)将行向上移动
    column_position: usize,
    //column_position 字段跟踪最后一行中的当前位置。
    color_code: ColorCode,
    buffer: &'static mut Buffer, // 静态生存期指定引用对于整个程序运行时是有效的
}

impl Writer {
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ', // 使用空格填充
            color_code: self.color_code,
        };
        // 遍历整个一行
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn new_line(&mut self) {
        // 将下面一行的东西搬到上面一行去
        // 最后一行不搬相当于将其覆盖
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // 向上一行
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 如果是 ascii 码
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 不是就写入一个特殊字符
                _ => self.write_byte(0xfe),
            }
        }
    }
}

// 测试代码
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        // 0xb8000 是 VGA 的固定规定的地址 https://en.wikipedia.org/wiki/Memory-mapped_I/O
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    // ö 不是 ASCII 码
    writer.write_string("Wörld!\n");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}