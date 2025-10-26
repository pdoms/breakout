use grapes::{events::input::Events, fonts::Font, linal::vertx2::VX2, renderer::two_d::Renderer};

use crate::screens::Screen;

pub const WIDTH: f32 = 760.0;
pub const HEIGHT: f32 = 900.0;
pub const FRAME_OFFSET: f32 = 4.0;
pub const PADDLE_WIDTH: f32 = 100.0;
pub const PADDLE_HEIGHT: f32 = 30.0;
pub const PADDLE_VEL: f32 = 8.0;
pub const BALL_RADIUS: f32 = 10.0;
pub const BALL_BASE_VEL_Y: f32 = 6.0;
pub const BALL_BASE_VEL_X: f32 = 1.0;
pub const MENU_TITLE: &[u8; 8] = b"Breakout";
pub const MENU_ACTION: &[u8; 18] = b"Hit Enter To Start";
pub const MENU_QUIT: &[u8; 18] = b"Hit Escape To Quit";
pub const MENU_SCORE: &[u8; 10] = b"Your Score";

pub const START_IN: &[u8; 8] = b"Start in";
pub const NEXT_BALL_IN: &[u8; 12] = b"Next Ball in";
pub const DELAY_ITER: usize = 45;
pub const DELEAY_TO_BALL: usize = 3 * DELAY_ITER;
pub const BRICK_ROWS: usize = 8;
pub const BRICK_COLS: usize = 14;
pub const BRICK_MARGIN: f32 = 2.0;
pub const BRICK_HEIGHT: f32 = 28.0;
pub const HEADING_SIZE: f32 = 50.0;
pub const PAUSE_TEXT_WIDTH: f32 = 130.0;
pub const PAUSE_TEXT_HEIGHT: f32 = 44.0;
pub const PAUSE_MARGIN_LEFT: f32 = 4.0;
pub const PAUSE_FS: f32 = 32.0;

pub trait GameControl {
    fn update(&mut self, renderer: &mut Renderer, events: &Events) -> Option<Screen>;
    fn draw(&self, renderer: &mut Renderer);
    fn get_points(&mut self) -> Option<usize> {
        return None;
    }
    fn set_velocity(&mut self, vel: VX2, level: f32) {
        unimplemented!("set_velocity");
    }

    fn get_round(&self) -> Option<usize> {
        None
    }

    fn prepare_render_score(&mut self, score: usize, font: &Font) {
        unimplemented!("get_score")
    }
    fn reset(&mut self) {
        unimplemented!("reset")
    }
}
