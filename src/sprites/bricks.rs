use grapes::{
    colors::color::Color,
    events::input::Events,
    objects::{collision::epa::EpaResult, rectangle::Rectangle},
    renderer::two_d::{Render, Renderer},
    state::two_d::State,
    vx2,
};

use crate::{
    common::{
        BRICK_COLS, BRICK_HEIGHT, BRICK_MARGIN, BRICK_ROWS, FRAME_OFFSET, GameControl,
        HEADING_SIZE, WIDTH,
    },
    screens::Screen,
};

use super::ball::Ball;

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum BrickState {
    #[default]
    Alive,
    Hit,
    Dead,
}

#[derive(Default, Clone, Copy)]
pub struct Brick {
    rect: Rectangle,
    color: Color,
    state: BrickState,
}

impl Brick {
    fn is_alive(&self) -> bool {
        self.state == BrickState::Alive
    }
}

pub struct Bricks {
    bricks: [Brick; BRICK_COLS * BRICK_ROWS],
    hit: Option<usize>,
}

fn brick_width() -> f32 {
    let mut w = WIDTH as f32 - FRAME_OFFSET * 2.0 - 2.0;
    w -= BRICK_COLS as f32 * 2.0 * BRICK_MARGIN;
    w / BRICK_COLS as f32
}

fn brick_color(row: usize) -> Color {
    match row {
        0 | 1 => Color::new(0xDA, 0x2A, 0x47, 0xFF),
        2 | 3 => Color::new(0xB4, 0x4A, 0x5F, 0xFF),
        4 | 5 => Color::new(0x8E, 0x6F, 0x77, 0xFF),
        _ => Color::new(0x04, 0x8C, 0x7F, 0xFF),
    }
}

impl Bricks {
    pub fn init() -> Self {
        let width = brick_width();
        let height = BRICK_HEIGHT;
        let start_x = FRAME_OFFSET + 1.0 + BRICK_MARGIN + width * 0.5;
        let mut x = start_x;
        let mut y = HEADING_SIZE + FRAME_OFFSET + 1.0 + height * 0.5;
        let x_offset = width + 2.0 * BRICK_MARGIN;
        let y_offset = height + 2.0 * BRICK_MARGIN;

        let mut bricks = Self {
            bricks: [Brick::default(); BRICK_ROWS * BRICK_COLS],
            hit: None,
        };

        for row in 0..BRICK_ROWS {
            //color
            let c = brick_color(row);

            for col in 0..BRICK_COLS {
                bricks.bricks[row * BRICK_COLS + col].rect =
                    Rectangle::new(vx2!(x, y), vx2!(width, height));
                bricks.bricks[row * BRICK_COLS + col].color = c;
                x += x_offset;
            }

            y += y_offset;
            x = start_x;
        }

        bricks
    }

    pub fn get_hit(&mut self) -> Option<usize> {
        let ret = self.hit.map(|v| v / BRICK_COLS);
        self.hit = None;
        ret
    }

    pub fn ball_hits(&mut self, ball: &Ball) -> Option<EpaResult> {
        for (i, brick) in self.bricks.iter().enumerate() {
            if let Some(coll_data) = ball.hits_epa(&brick.rect) {
                if brick.is_alive() {
                    self.hit = Some(i);
                    return Some(coll_data);
                }
            }
        }
        return None;
    }
}

impl GameControl for Bricks {
    fn update(&mut self, _renderer: &mut Renderer, _events: &Events) -> Option<Screen> {
        if let Some(hit) = self.hit {
            self.bricks[hit].state = BrickState::Dead;
        }
        return None;
    }

    fn draw(&self, renderer: &mut Renderer) {
        for brick in self.bricks.iter() {
            if brick.is_alive() {
                let mut fill = brick.color.clone();
                fill.saturation(0.4);
                brick.rect.fill_clr(renderer, fill);
                brick.rect.draw_clr(renderer, brick.color);
            }
        }
    }
    fn reset(&mut self) {
        self.hit = None;
        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COLS {
                self.bricks[row * BRICK_COLS + col].state = BrickState::Alive;
            }
        }
    }
}
