extern crate sdl2;

use chip8_core::chip8::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};
use chip8_core::hello_core;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::io::Read;
use std::time::{Duration, UNIX_EPOCH};
use desktop::{map_key, get_current_time_in_microseconds};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: program [path_to_rom]");
        return;
    }
    const NUMBER_OF_CYCLES: u8 = 10;
    let mut chip8 = Chip8::new();
    desktop::load_file(&args[1], &mut chip8);
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
                Event::KeyDown {
                    keycode: Some(keycode) ,
                    ..
                } => {
                    let key = map_key(keycode);
                    chip8.key_down(key, true);
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    let key = map_key(keycode);
                    chip8.key_down(key, false);
                }
                _ => {}
            }
        }

        let start_time = get_current_time_in_microseconds();
        // The rest of the game loop goes here...
        for _ in 0..NUMBER_OF_CYCLES {
            chip8.tick();
        }
        chip8.tick_timers();
        desktop::draw_to_screen(&mut canvas, &mut chip8);
        canvas.present();
        let end_time = get_current_time_in_microseconds();
        // This sleep call ensures that the system will run at 60fps. Since modern hardware is so advanced this is
        // required in order to provide a decent execution speed.
        std::thread::sleep(Duration::from_micros((16666.67 as u64) - (end_time - start_time) as u64));
    }
}
