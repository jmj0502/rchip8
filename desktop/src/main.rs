extern crate sdl2;

use chip8_core::chip8::Chip8;
use desktop::{get_current_time_in_microseconds, map_key, Scale};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: program [path_to_rom]");
        return;
    }
    const NUMBER_OF_CYCLES: u8 = 10;
    let mut chip8 = Chip8::new();
    let mut scale = Scale {
        width: 15,
        height: 15,
    };
    desktop::load_file(&args[1], &mut chip8);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 950, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let key = map_key(keycode);
                    chip8.key_down(key, true);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    let key = map_key(keycode);
                    chip8.key_down(key, false);
                }
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(w, h) = win_event {
                        scale.width = w / 64;
                        scale.height = h / 32;
                    }
                }
                _ => {}
            }
        }

        let _start_time = get_current_time_in_microseconds();
        // The rest of the game loop goes here...
        for _ in 0..NUMBER_OF_CYCLES {
            chip8.tick();
        }
        chip8.tick_timers();
        desktop::draw_to_screen(&mut canvas, &mut chip8, &scale);
        canvas.present();
        let _end_time = get_current_time_in_microseconds();

        // This sleep call ensures that the system will run at 60fps. Modern hardware is so advanced that the emulator
        // may run at more than 400fps. So, this step is required in order to achieve a decent execution speed.
        std::thread::sleep(Duration::from_millis(21));
    }
}
