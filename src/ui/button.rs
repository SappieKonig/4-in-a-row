use ggez::graphics;
use ggez::mint::Point2;

#[derive(Clone)]
pub struct Button {
    pub rect: graphics::Rect,
    pub text: String,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str) -> Self {
        Self {
            rect: graphics::Rect::new(x, y, width, height),
            text: text.to_string(),
        }
    }

    pub fn contains(&self, point: Point2<f32>) -> bool {
        self.rect.contains(point)
    }
}