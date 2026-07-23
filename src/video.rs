pub const VIDEO_WIDTH: usize = 40;
pub const VIDEO_HEIGHT: usize = 12;

pub struct Video {
    mem: [u8; VIDEO_HEIGHT * VIDEO_WIDTH],
    cursor: usize,
}

impl Video {
    pub fn new() -> Video {
        Video {
            mem: [0; VIDEO_HEIGHT * VIDEO_WIDTH],
            cursor: 0,
        }
    }

    pub fn get_char(&self, idx: usize) -> u8 {
        self.mem[idx]
    }

    pub fn set_char(&mut self, ch: u8) {
        if ch == b'\n' {
            // Move to start of next line
            self.cursor = (self.cursor / VIDEO_WIDTH + 1) * VIDEO_WIDTH;
            if self.cursor >= VIDEO_HEIGHT * VIDEO_WIDTH {
                self.cursor = 0;
            }
        } else {
            if self.cursor < VIDEO_HEIGHT * VIDEO_WIDTH {
                self.mem[self.cursor] = ch;
            }
            self.cursor += 1;
            if self.cursor >= VIDEO_HEIGHT * VIDEO_WIDTH {
                self.cursor = 0;
            }
        }
    }
}
