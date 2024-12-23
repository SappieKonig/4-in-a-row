use ggez::graphics::{self, Canvas, Color, DrawParam, Rect, Text, TextFragment, Drawable};
use ggez::{Context, GameResult};
use ggez::mint::Point2;

#[derive(Clone)]
pub struct Dropdown<T: Clone> {
    pub rect: Rect,
    pub options: Vec<(String, T)>,
    pub selected_index: usize,
    pub is_open: bool,
}

impl<T: Clone> Dropdown<T> {
    pub fn new(x: f32, y: f32, width: f32, height: f32, options: Vec<(String, T)>) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            options,
            selected_index: 0,
            is_open: false,
        }
    }

    fn get_option_rect(&self, index: usize) -> Rect {
        Rect::new(
            self.rect.x,
            self.rect.y + self.rect.h * (index) as f32,
            self.rect.w,
            self.rect.h,
        )
    }

    pub fn contains(&self, point: Point2<f32>) -> bool {
        if !self.is_open {
            return self.rect.contains(point);
        }
        
        // When open, check all option rects
        for i in 0..self.options.len() {
            if self.get_option_rect(i).contains(point) {
                return true;
            }
        }
        false
    }

    pub fn get_option_at(&self, point: Point2<f32>) -> Option<usize> {
        if !self.is_open {
            return None;
        }

        // Check each option rectangle
        for i in 0..self.options.len() {
            if self.get_option_rect(i).contains(point) {
                return Some(i);
            }
        }
        None
    }

    pub fn selected_value(&self) -> T {
        self.options[self.selected_index].1.clone()
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Draw all options if open, otherwise just the selected one
        let num_rects = if self.is_open { self.options.len() } else { 1 };

        for i in 0..num_rects {
            let rect = self.get_option_rect(i);
            
            // Draw background
            let bg = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                if i == self.selected_index { Color::from_rgb(230, 230, 230) } else { Color::WHITE },
            )?;
            canvas.draw(&bg, DrawParam::default());

            // Draw border
            let border = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(2.0),
                rect,
                Color::BLACK,
            )?;
            canvas.draw(&border, DrawParam::default());

            // Draw text
            let option_text = &self.options[if self.is_open { i } else { self.selected_index }].0;
            let text = Text::new(TextFragment::new(option_text).color(Color::BLACK));
            let text_dims = text.dimensions(ctx).unwrap();
            canvas.draw(
                &text,
                DrawParam::default().dest([
                    rect.x + 10.0,
                    rect.y + (rect.h - text_dims.h as f32) / 2.0,
                ]),
            );
        }

        // Draw dropdown arrow on the main box
        let arrow = if self.is_open { "▼" } else { "▲" };
        let arrow_text = Text::new(TextFragment::new(arrow).color(Color::BLACK));
        let arrow_dims = arrow_text.dimensions(ctx).unwrap();
        canvas.draw(
            &arrow_text,
            DrawParam::default().dest([
                self.rect.x + self.rect.w - arrow_dims.w as f32 - 10.0,
                self.rect.y + (self.rect.h - arrow_dims.h as f32) / 2.0,
            ]),
        );

        Ok(())
    }
}