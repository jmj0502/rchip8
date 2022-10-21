use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use chip8_core::chip8::{Chip8, SCREEN_WIDTH, SCREEN_HEIGHT};
use std::fs::File;
use std::io::Read;
use sdl2::keyboard::Keycode;


const SCALE: u32 = 15;

pub fn draw_to_screen(canvas: &mut WindowCanvas, emu: &Chip8) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = emu.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
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
        _ => None
    }
}

struct SDLCanvas {
}