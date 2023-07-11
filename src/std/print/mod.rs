
mod string;
#[macro_use]
pub mod macros;

use string::Color;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}


use self::string::{VgaChar, ColorCode};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


#[repr(transparent)]
pub struct Buffer {
    chars: [[VgaChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
pub struct Writer {
    col: usize,
    line: usize,
    buffer: &'static mut Buffer,
    color: ColorCode
}

impl Writer {
    pub fn new() -> Self {
        let mut s = Self {
            col: 0,
            line: 0,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            color: ColorCode::new(Color::White, Color::Black)
        };
        s.clear_screen();
        s
    }

    fn write_byte(&mut self, chr: u8) {
        match chr {
            b'\n' => self.new_line(),
            _ => {
                if self.col >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.chars[self.line][self.col] = VgaChar {
                    chr,
                    color: self.color,
                };
                self.col += 1;   
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.as_bytes() {
            match c {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(*c),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    
    #[allow(dead_code)]
    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = ColorCode::new(fg, bg);
    }
    
    pub fn clear_screen(&mut self) {
        let color = ColorCode::new(Color::White, Color::Black);
        for line in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[line][col] = VgaChar {
                    chr: 0,
                    color,
                };
            }
        }
    }

    pub fn clear_line(&mut self, line: usize) {
        let color = ColorCode::new(Color::White, Color::Black);
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[line][col] = VgaChar {
                chr: 0,
                color,
            };
        }
    }

    pub fn new_line(&mut self) {
        if self.line >= BUFFER_HEIGHT - 1{ 
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let chr = self.buffer.chars[row][col];
                    self.buffer.chars[row - 1][col] = chr;
                }
            }
            self.clear_line(BUFFER_HEIGHT - 1);
        } else {
            self.line += 1;
        }
        self.col = 0;
    }
}
