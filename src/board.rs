use ggez::graphics::{self, DrawMode, Mesh, Rect};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use glam::*;
use std::string::String;

const GUTTER: f32 = 5.0;
const MIN_BORDER: f32 = 100.0;

#[derive(Debug, PartialEq)]
pub enum Player {
    Circle,
    Square,
}

pub struct GameBoard {
    pub game_state: Vec<Option<Player>>,
    pub width: f32,
    square_width: f32,
    grid_unit: f32,
    icon_size: f32,
    height: f32,
}

impl GameBoard {
    pub fn new(ctx: &mut Context) -> GameBoard {
        let game_state = vec![None, None, None, None, None, None, None, None, None];
        assert_eq!(9, game_state.len());
        let (width, height) = graphics::size(ctx);
        let square_width: f32 = (height - (GUTTER * 3.0) - (MIN_BORDER * 2.0)) / 3.0;
        let grid_unit: f32 = square_width + GUTTER;
        let icon_size: f32 = (square_width / 2.0) * 0.75;
        GameBoard {
            game_state: game_state,
            square_width: square_width,
            grid_unit: grid_unit,
            icon_size: icon_size,
            width: width,
            height: height,
        }
    }
    pub fn reset_game_state(&mut self) {
        self.game_state = vec![None, None, None, None, None, None, None, None, None];
    }
    pub fn is_full(&self) -> bool {
        for i in 0..self.game_state.len() {
            match &self.game_state[i] {
                None => return false,
                _ => continue,
            }
        }
        true
    }
    pub fn play(&mut self, player: Player, pos: usize) -> Result<bool, String> {
        if let Some(player) = &self.game_state[pos] {
            let error = format!("{:?} is already there.", player);
            return Err(error);
        }
        self.game_state[pos] = Some(player);
        Ok(true)
    }
    pub fn player_icon_at_index(&self, ctx: &mut Context, index: usize, size: f32) -> Option<Mesh> {
        if let Some(player) = &self.game_state[index] {
            return match player {
                Player::Circle => Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    na::Point2::new(0.0, 0.0),
                    size / 2.0, // size should be the radius
                    0.1,
                    graphics::WHITE,
                )
                .ok(),
                Player::Square => Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    graphics::Rect::new(size / -2.0, size / -2.0, size, size),
                    graphics::WHITE,
                )
                .ok(),
            };
        }

        None
    }
    pub fn get_sq_for_point(&self, point: na::Point2<f32>) -> Option<usize> {
        let (x_pos, y_pos) = (point[0], point[1]);
        let center = na::Point2::new(self.width / 2.0, self.height / 2.0);
        let top_left = na::Point2::new(
            center[0] - self.grid_unit * 1.5,
            center[1] - self.grid_unit * 1.5,
        );

        for x in 0..3 {
            for y in 0..3 {
                let square_top_left = na::Point2::new(
                    top_left[0] + (x as f32 * self.grid_unit),
                    top_left[1] + (y as f32 * self.grid_unit),
                );
                let square_x_min = square_top_left[0];
                let square_x_max = square_top_left[0] + self.square_width;
                let square_y_min = square_top_left[1];
                let square_y_max = square_top_left[1] + self.square_width;

                if (square_x_min < x_pos)
                    && (x_pos < square_x_max)
                    && (square_y_min < y_pos)
                    && (y_pos < square_y_max)
                {
                    return Some((y * 3) + x);
                }
            }
        }

        None
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, self.square_width, self.square_width),
            graphics::BLACK,
        )?;

        // board top left calculated from center point
        let center = na::Point2::new(self.width / 2.0, self.height / 2.0);
        let top_left = na::Point2::new(
            center[0] - self.grid_unit * 1.5,
            center[1] - self.grid_unit * 1.5,
        );

        for x in 0..3 {
            for y in 0..3 {
                let square_origin = na::Point2::new(
                    top_left[0] + (x as f32 * self.grid_unit),
                    top_left[1] + (y as f32 * self.grid_unit),
                );
                graphics::draw(ctx, &rect, (square_origin,))?;

                if let Some(player_icon) =
                    self.player_icon_at_index(ctx, (y * 3) + x, self.icon_size)
                {
                    let player_point = na::Point2::new(
                        square_origin[0] + (self.grid_unit / 2.0),
                        square_origin[1] + (self.grid_unit / 2.0),
                    );
                    graphics::draw(ctx, &player_icon, (player_point,))?;
                }
            }
        }

        Ok(())
    }
}
