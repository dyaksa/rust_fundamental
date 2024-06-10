#[derive(Debug)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn red() -> Self {
        Color(255,0,0)
    }

    pub fn green() -> Self {
        Color(0,255,0)
    }

    pub fn blue() -> Self {
        Color(0,0,255)
    }
} 