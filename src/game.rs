use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, Text, TextFragment, Drawable, DrawParam};
use ggez::input::mouse::MouseButton;
use ggez::mint::Point2;

use crate::board::Board;
use crate::player::Player;
use crate::ui::{button::Button, screen::GameScreen, drawing, dropdown::Dropdown};
use crate::config::{SCREEN_WIDTH, SCREEN_HEIGHT, BUTTON_WIDTH, BUTTON_HEIGHT, CELL_SIZE};
use crate::bot::{Bot, RandomBot, MctsBot};

pub struct GameState {
    board: Board,
    current_player_number: u8,  // 1 or 2
    player_types: [Player; 2],  // Stores if each player is Human or a type of Bot
    game_over: bool,
    screen: GameScreen,
    start_button: Button,
    new_game_button: Button,
    winner: Option<u8>,  // 1, 2, or None for draw
    player1_dropdown: Dropdown<Player>,
    player2_dropdown: Dropdown<Player>,
    random_bot: RandomBot,
    mcts_bot: MctsBot,
}

impl GameState {
    pub fn new() -> Self {
        let start_button = Button::new(
            SCREEN_WIDTH / 2.0 - BUTTON_WIDTH / 2.0,
            SCREEN_HEIGHT / 2.0,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            "Start Game"
        );

        let new_game_button = Button::new(
            SCREEN_WIDTH / 2.0 - BUTTON_WIDTH / 2.0,
            SCREEN_HEIGHT / 2.0 + BUTTON_HEIGHT + 20.0,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            "New Game"
        );

        let player_options = vec![
            ("Human".to_string(), Player::Human),
            ("Random Bot".to_string(), Player::RandomBot),
            ("MCTS Bot".to_string(), Player::MctsBot),
        ];

        // Position dropdowns side by side
        let spacing = 40.0;
        let total_width = BUTTON_WIDTH * 2.0 + spacing;
        let start_x = (SCREEN_WIDTH - total_width) / 2.0;
        let y = SCREEN_HEIGHT / 3.0;

        let player1_dropdown = Dropdown::new(
            start_x,
            y,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            player_options.clone(),
        );

        let player2_dropdown = Dropdown::new(
            start_x + BUTTON_WIDTH + spacing,
            y,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            player_options,
        );

        Self {
            board: Board::new(),
            current_player_number: 1,
            player_types: [Player::Human, Player::Human],
            game_over: false,
            screen: GameScreen::Menu,
            start_button,
            new_game_button,
            winner: None,
            player1_dropdown,
            player2_dropdown,
            random_bot: RandomBot,
            mcts_bot: MctsBot::new(1000), // 1 second thinking time
        }
    }

    pub fn reset_game(&mut self) {
        self.board = Board::new();
        self.current_player_number = 1;
        self.game_over = false;
        self.winner = None;
        self.screen = GameScreen::Menu;
    }

    fn handle_player_move(&mut self, col: usize) {
        let current_player = self.player_types[(self.current_player_number - 1) as usize];
        if let Some((row, col)) = self.board.make_move(col, current_player, self.current_player_number) {
            if self.board.check_win(row, col) {
                self.game_over = true;
                self.winner = Some(self.current_player_number);
                self.screen = GameScreen::GameOver;
            } else if self.board.is_full() {
                self.game_over = true;
                self.winner = None;
                self.screen = GameScreen::GameOver;
            } else {
                // Switch between player 1 and 2
                self.current_player_number = if self.current_player_number == 1 { 2 } else { 1 };
            }
        }
    }
    
    fn start_game(&mut self) {
        // Set player types based on dropdown selections
        self.player_types[0] = self.player1_dropdown.selected_value();
        self.player_types[1] = self.player2_dropdown.selected_value();
        
        self.board = Board::new();
        self.current_player_number = 1;
        self.game_over = false;
        self.winner = None;
        self.screen = GameScreen::Game;
        
        // If first player is a bot, make their move
        if self.player_types[0].is_bot() {
            let col = match self.player_types[0] {
                Player::RandomBot => self.random_bot.make_move(&self.board, self.current_player_number),
                Player::MctsBot => self.mcts_bot.make_move(&self.board, self.current_player_number),
                _ => None,
            };
            if let Some(col) = col {
                self.handle_player_move(col);
            }
        }
    }

    fn get_current_player_type(&self) -> Player {
        self.player_types[(self.current_player_number - 1) as usize]
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.screen == GameScreen::Game && 
           !self.game_over && 
           self.get_current_player_type().is_bot() {
            // Get the current bot's move
            let col = match self.get_current_player_type() {
                Player::RandomBot => self.random_bot.make_move(&self.board, self.current_player_number),
                Player::MctsBot => self.mcts_bot.make_move(&self.board, self.current_player_number),
                _ => None,
            };
            
            if let Some(col) = col {
                self.handle_player_move(col);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        match self.screen {
            GameScreen::Menu => {
                // Draw player selection texts
                let p1_text = Text::new(
                    TextFragment::new("Player 1:").color(Color::BLACK)
                );
                let p2_text = Text::new(
                    TextFragment::new("Player 2:").color(Color::BLACK)
                );
                
                let text_y = SCREEN_HEIGHT / 3.0 - 30.0;
                canvas.draw(
                    &p1_text,
                    DrawParam::default().dest([
                        self.player1_dropdown.rect.x,
                        text_y,
                    ]),
                );
                canvas.draw(
                    &p2_text,
                    DrawParam::default().dest([
                        self.player2_dropdown.rect.x,
                        text_y,
                    ]),
                );

                self.player1_dropdown.draw(ctx, &mut canvas)?;
                self.player2_dropdown.draw(ctx, &mut canvas)?;
                drawing::draw_button(ctx, &mut canvas, &self.start_button, false)?;
            }
            GameScreen::Game => {
                drawing::draw_board(ctx, &mut canvas, &self.board)?;

                // Draw current player indicator
                let current_type = self.get_current_player_type();
                let player_text = format!(
                    "Current Turn: Player {} ({})",
                    self.current_player_number,
                    match current_type {
                        Player::Human => "Human",
                        Player::RandomBot => "Random Bot",
                        Player::MctsBot => "MCTS Bot",
                        Player::Empty => "",
                    }
                );
                let text = Text::new(TextFragment::new(player_text).color(Color::BLACK));
                let text_dims = text.dimensions(ctx).unwrap();
                canvas.draw(
                    &text,
                    DrawParam::default().dest([
                        (SCREEN_WIDTH - text_dims.w as f32) / 2.0,
                        10.0,
                    ]),
                );
            }
            GameScreen::GameOver => {
                drawing::draw_board(ctx, &mut canvas, &self.board)?;

                // Draw game over message
                let message = match self.winner {
                    Some(player_num) => format!("Player {} Wins!", player_num),
                    None => "Game Draw!".to_string(),
                };

                let text = Text::new(TextFragment::new(message).color(Color::BLACK));
                let text_dims = text.dimensions(ctx).unwrap();
                canvas.draw(
                    &text,
                    DrawParam::default().dest([
                        (SCREEN_WIDTH - text_dims.w as f32) / 2.0,
                        SCREEN_HEIGHT / 4.0,
                    ]),
                );

                drawing::draw_button(ctx, &mut canvas, &self.new_game_button, false)?;
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            let point = Point2 { x, y };

            match self.screen {
                GameScreen::Menu => {
                    if self.start_button.contains(point) {
                        self.start_game();
                    } else if self.player1_dropdown.contains(point) {
                        if self.player1_dropdown.is_open {
                            if let Some(index) = self.player1_dropdown.get_option_at(point) {
                                self.player1_dropdown.selected_index = index;
                            }
                            self.player1_dropdown.is_open = false;
                        } else {
                            self.player1_dropdown.is_open = true;
                            self.player2_dropdown.is_open = false;
                        }
                    } else if self.player2_dropdown.contains(point) {
                        if self.player2_dropdown.is_open {
                            if let Some(index) = self.player2_dropdown.get_option_at(point) {
                                self.player2_dropdown.selected_index = index;
                            }
                            self.player2_dropdown.is_open = false;
                        } else {
                            self.player2_dropdown.is_open = true;
                            self.player1_dropdown.is_open = false;
                        }
                    }
                }
                GameScreen::Game => {
                    if !self.game_over && !self.get_current_player_type().is_bot() {
                        let col = (x / CELL_SIZE) as usize;
                        self.handle_player_move(col);
                    }
                }
                GameScreen::GameOver => {
                    if self.new_game_button.contains(point) {
                        self.reset_game();
                    }
                }
            }
        }
        Ok(())
    }
}