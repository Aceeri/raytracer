
#[derive(Debug, Copy, Clone)]
pub enum Color {
    Bit8(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Self {
        Color::Bit8(0, 0, 0)
    }
}
