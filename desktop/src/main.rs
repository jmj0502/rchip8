extern crate sdl2;

use chip8_core::chip8::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};
use chip8_core::hello_core;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::io::Read;
use std::time::Duration;

const SCALE: u32 = 15;

fn draw_to_screen(canvas: &mut WindowCanvas, emu: &Chip8) {
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

fn load_file(path: &str, emu: &mut Chip8) {
    let mut file = std::fs::File::open(path).expect("Couldn't find the specified file.");
    let mut file_buffer = Vec::new();

    file.read_to_end(&mut file_buffer)
        .expect("Couldn't read file to memory!");
    emu.load_file(&file_buffer);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: program [path_to_rom]");
        return;
    }
    let mut chip8 = Chip8::new();
    load_file(&args[1], &mut chip8);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 950, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        chip8.tick();
        draw_to_screen(&mut canvas, &mut chip8);
        canvas.present();
    }
}
