use crate::chip8::Chip8;

pub struct Scale {
    pub width: i32,
    pub height: i32,
}

pub trait Display {
    fn draw_to_screen(&self, screen_buf: &[bool; 2048], scale: &Scale);
}
