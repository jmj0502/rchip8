use chip8_core::chip8::{Chip8, SCREEN_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

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
