pub mod audio;
pub mod display;

use crate::audio::AudioDeviceWrapper;
use chip8_core::chip8::Chip8;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::io::Read;
use std::time::UNIX_EPOCH;
use std::time::{Duration, Instant};

pub fn run(path_to_rom: &str) {
    const NUMBER_OF_CYCLES: u8 = 8;
    let mut chip8 = Chip8::new();
    let mut scale = display::Scale {
        width: 15,
        height: 18,
    };
    load_file(path_to_rom, &mut chip8);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't initialize the video component.");
    let audio_subsystem = sdl_context
        .audio()
        .expect("Couldn't initialize the audio component.");
    let audio_device = AudioDeviceWrapper::new(&audio_subsystem);

    let window = video_subsystem
        .window("rust-sdl2 demo", 950, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame_time = Instant::now();
    let frame_interval = Duration::new(0, 1_000_000_000u32 / chip8.get_fps());
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
                    if keycode != Keycode::F1 {
                        let key = map_key(keycode);
                        chip8.key_down(key, true);
                    } else {
                        chip8.save_state();
                    }
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

        // The rest of the game loop goes here...
        for _ in 0..NUMBER_OF_CYCLES {
            chip8.tick();
        }

        // This sleep call ensures that the system will run at 60fps. Modern hardware is so advanced that the emulator
        // may run at more than 400fps. So, this step is required in order to achieve a decent execution speed.
        std::thread::sleep(frame_interval.saturating_sub(frame_time.elapsed()));
        display::draw_to_screen(&mut canvas, &mut chip8, &scale);
        canvas.present();
        let should_beep = chip8.tick_timers();
        println!("Should beep: {}", should_beep);
        audio_device.beep(should_beep);

        frame_time = Instant::now();
    }
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
