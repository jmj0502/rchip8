pub trait Display {
    fn clear(self);
    fn draw_at_xy(&mut self, x: u8, y: u8);
}
