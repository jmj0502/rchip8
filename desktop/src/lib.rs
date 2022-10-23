use chip8_core::chip8::{Chip8, SCREEN_WIDTH};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::io::Read;
use std::time::UNIX_EPOCH;

pub struct Scale {
    pub width: i32,
    pub height: i32,
}

pub fn draw_to_screen(canvas: &mut WindowCanvas, emu: &Chip8, scale: &Scale) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = emu.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;
            let rect = Rect::new(
                (x * (scale.width as u32)) as i32,
                (y * (scale.height as u32)) as i32,
                scale.width as u32,
                scale.height as u32,
            );
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

pub fn load_file(path: &str, emu: &mut Chip8) {
    let mut file = std::fs::File::open(path).expect("Couldn't find the specified file.");
    let mut file_buffer = Vec::new();

    file.read_to_end(&mut file_buffer)
        .expect("Couldn't read file to memory!");
    emu.load_file(&file_buffer);
}

pub fn map_key(key: Keycode) -> Option<u8> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::B => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

pub fn get_current_time_in_microseconds() -> u128 {
    let current_system_time = std::time::SystemTime::now();
    let current_time_in_micros = current_system_time
        .duration_since(UNIX_EPOCH)
        .expect("Couldn't get the duration since UNIX EPOCH from current system time.")
        .as_micros();
    current_time_in_micros
}