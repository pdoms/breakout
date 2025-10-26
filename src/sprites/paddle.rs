use grapes::{
    colors::{color::Color, presets::GrapesColors},
    events::{input::Events, keyboard::K},
    objects::{rectangle::Rectangle, utils::BBox2d},
    renderer::two_d::{Render, Renderer},
    vx2,
};

use crate::{
    common::{FRAME_OFFSET, GameControl, HEIGHT, PADDLE_HEIGHT, PADDLE_VEL, PADDLE_WIDTH, WIDTH},
    screens::Screen,
};

pub struct Paddle {
    pub rect: Rectangle,
    color: Color,
    pub trans_color: Color,
    pub in_trans: bool,
    vel: f32,
}

impl Paddle {
    pub fn init() -> Self {
        Self {
            rect: Rectangle::new(
                vx2!(WIDTH * 0.5, HEIGHT - (FRAME_OFFSET + PADDLE_HEIGHT)),
                vx2!(PADDLE_WIDTH, PADDLE_HEIGHT),
            ),
            vel: PADDLE_VEL,
            color: GrapesColors::Teal.into(),
            trans_color: GrapesColors::Teal.into(),
            in_trans: false,
        }
    }




    pub fn reset(&mut self) {
        self.rect.pos = vx2!(WIDTH * 0.5, HEIGHT - (FRAME_OFFSET + PADDLE_HEIGHT));
    }
    pub fn set_x(&mut self, x: f32) {
        self.rect.pos.x = x;
    }
}

impl GameControl for Paddle {
    fn update(&mut self, _renderer: &mut Renderer, events: &Events) -> Option<Screen> {
        let paddle_bbox = self.rect.bbox();
        if events.key_pressed(K::ArrowRight) || events.key_down(K::ArrowRight) {
            if (paddle_bbox.max_x + self.vel) < WIDTH - FRAME_OFFSET {
                self.rect.pos.x += self.vel;
            }
        }
        if events.key_pressed(K::ArrowLeft) || events.key_down(K::ArrowLeft) {
            if (paddle_bbox.min_x - self.vel) > FRAME_OFFSET {
                self.rect.pos.x -= self.vel;
            }
        }

        return None;
    }

    fn draw(&self, renderer: &mut Renderer) {
        if self.in_trans {
            self.rect.fill_clr(renderer, self.trans_color);
        } else {
            self.rect.fill_clr(renderer, self.color);
        }
    }

    fn set_velocity(&mut self, _vel: grapes::linal::vertx2::VX2, level: f32) {
        self.vel *= level;
    }
}
