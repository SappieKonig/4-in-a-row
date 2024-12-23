use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, DrawParam, Text, TextFragment, Drawable, Color};
use crate::board::Board;
use crate::config::{CELL_SIZE, GRID_COLS, GRID_ROWS};
use crate::player::Player;
use crate::ui::button::Button;
use ggez::mint::Point2;

pub fn draw_button(ctx: &mut Context, canvas: &mut Canvas, button: &Button, selected: bool) -> GameResult {
    let color = if selected { Color::GREEN } else { Color::BLUE };
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        button.rect,
        color,
    )?;
    canvas.draw(&rect, DrawParam::default());

    let text = Text::new(TextFragment::new(&button.text).color(Color::WHITE));
    let text_dims = text.dimensions(ctx).unwrap();
    canvas.draw(
        &text,
        DrawParam::default().dest([
            button.rect.x + (button.rect.w - text_dims.w as f32) / 2.0,
            button.rect.y + (button.rect.h - text_dims.h as f32) / 2.0,
        ]),
    );

    Ok(())
}

pub fn draw_board(ctx: &mut Context, canvas: &mut Canvas, board: &Board) -> GameResult {
    // Draw the grid
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(2.0),
                graphics::Rect::new(
                    col as f32 * CELL_SIZE,
                    row as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                ),
                Color::BLACK,
            )?;
            canvas.draw(&rect, DrawParam::default());

            // Get both the piece and the player number (1 or 2)
            if let player_number = board.get_player_number(row, col) {
                if player_number != 0 {
                    let color = if player_number == 1 { Color::RED } else { Color::YELLOW };
                    let circle = graphics::Mesh::new_circle(
                        ctx,
                        graphics::DrawMode::fill(),
                        Point2 {
                            x: col as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                            y: row as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                        },
                        CELL_SIZE / 2.5,
                        0.1,
                        color,
                    )?;
                    canvas.draw(&circle, DrawParam::default());
                }
            }
        }
    }
    Ok(())
}