mod board;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::nalgebra as na;
use ggez::{graphics, timer, Context, ContextBuilder, GameResult};
use rand::distributions::{Distribution, Uniform};
use std::string::String;

const INITIAL_MESSAGE: &str = "Welcome to Tic-Tac-Toe!";

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("tic_tac_toe", "Jacob Sansbury")
        .build()
        .expect("Could not create ggez context!");

    let mut game = TicTacToe::new(&mut ctx);

    event::run(&mut ctx, &mut event_loop, &mut game).expect("Error running game.");
}

struct TicTacToe {
    board: board::GameBoard,
    message: String,
}

impl TicTacToe {
    pub fn new(_ctx: &mut Context) -> TicTacToe {
        TicTacToe {
            board: board::GameBoard::new(_ctx),
            message: String::from(INITIAL_MESSAGE),
        }
    }

    pub fn play_for_computer(&mut self) {
        let mut has_placed = false;
        let mut rng = rand::thread_rng();
        while !has_placed {
            let computer_player = board::Player::Square;
            let die = Uniform::from(0..9);
            let place = die.sample(&mut rng);
            if let Ok(_computer_player_result) = self.board.play(computer_player, place) {
                has_placed = true;
            }
        }
    }

    pub fn three_set_has_winner(&self, a: usize, b: usize, c: usize) -> Option<board::Player> {
        if self.board.game_state[a] == self.board.game_state[b]
            && self.board.game_state[b] == self.board.game_state[c]
        {
            match self.board.game_state[a] {
                Some(board::Player::Circle) => Some(board::Player::Circle),
                Some(board::Player::Square) => Some(board::Player::Square),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn has_winner(&self) -> Option<board::Player> {
        return self
            .three_set_has_winner(0, 1, 2)
            .or(self.three_set_has_winner(3, 4, 5))
            .or(self.three_set_has_winner(6, 7, 8))
            // vertical
            .or(self.three_set_has_winner(0, 3, 6))
            .or(self.three_set_has_winner(1, 4, 7))
            .or(self.three_set_has_winner(2, 5, 8))
            // diagnoal
            .or(self.three_set_has_winner(0, 4, 8))
            .or(self.three_set_has_winner(2, 4, 6));
    }
}

impl EventHandler for TicTacToe {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        const FPS: u32 = 24;
        while timer::check_update_time(_ctx, FPS) {
            return Ok(());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code
        self.board.draw(ctx)?;
        let message = graphics::TextFragment::from(self.message.as_ref())
            .color(graphics::Color::new(0.0, 0.0, 1.0, 1.0))
            .scale(graphics::Scale::uniform(25.0));

        let text_render = graphics::Text::new(message);
        let text_width = text_render.width(ctx) as f32;
        graphics::draw(
            ctx,
            &text_render,
            (na::Point2::new((self.board.width - text_width) / 2.0, 50.0),),
        )?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::R {
            self.message = String::from(INITIAL_MESSAGE);
            self.board.reset_game_state()
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let player = board::Player::Circle;
        if let Some(player) = self.has_winner() {
            println!("Can't place becase {:?} won!", player);
            return;
        }
        if let Some(place) = self.board.get_sq_for_point(na::Point2::new(x, y)) {
            println!("Clicked on {}", place);
            let result = self.board.play(player, place);

            if let Err(err) = result {
                return println!("{}", err);
            }

            if let Ok(played_piece) = result {
                if played_piece && !self.board.is_full() {
                    if let Some(player) = self.has_winner() {
                        self.message = format!("{:?} won!", player);
                        return;
                    }
                    self.play_for_computer();
                }
            }
        }
    }
}
