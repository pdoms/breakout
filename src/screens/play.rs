use std::f32;

use grapes::{
    colors::{color::Color, presets::GrapesColors},
    events::keyboard::K,
    linal::vertx2::VX2,
    objects::{line::Line2d, rectangle::Rectangle},
    renderer::two_d::Render,
    textures::Texture,
    vx2,
};

use crate::{
    common::{
        BALL_BASE_VEL_X, BALL_BASE_VEL_Y, BALL_RADIUS, GameControl, HEIGHT, PAUSE_TEXT_HEIGHT,
        PAUSE_TEXT_WIDTH, WIDTH,
    },
    sprites::{
        ball::{Ball, get_rand_init_vel},
        bricks::Bricks,
        paddle::Paddle,
    },
};

use super::Screen;

const POINTS: [usize; 8] = [100, 100, 50, 50, 25, 25, 10, 10];

#[derive(Debug)]
pub struct NewBallTransition {
    ticks: isize,
    idle: bool,
    start_x: f32,
    duration: isize,
}

impl NewBallTransition {
    pub fn new() -> Self {
        Self {
            duration: 75,
            ticks: 0,
            idle: true,
            start_x: 0.0,
        }
    }

    fn start(&mut self, x: f32) {
        self.ticks = self.duration;
        self.idle = false;
        self.start_x = x;
    }

    fn is_done(&self) -> bool {
        self.ticks <= 0
    }

    fn next(&mut self) -> f32 {
        let t = (1.0 - (self.ticks as f32 / self.duration as f32)).clamp(0.0, 1.0);
        self.ticks -= 1;
        t
    }
    fn set_idle(&mut self) {
        self.idle = true;
    }
}

pub struct Play {
    ball: Ball,
    bricks: Bricks,
    paddle: Paddle,
    frame: [Line2d; 5],
    round: usize,
    has_started: bool,
    is_paused: bool,
    points: usize,
    transition: NewBallTransition,
    pause_text: Texture,
    pause_rect: Rectangle,
    last_ball_vel: VX2,
}

impl Play {
    pub fn init(frame: [Line2d; 5], pause: Texture) -> Self {
        let paddle = Paddle::init();
        let mut ball_pos = paddle.rect.pos;
        let y_offset = paddle.rect.size.y * 0.5 + BALL_RADIUS + 2.0;
        ball_pos.y -= y_offset;
        Self {
            ball: Ball::new(ball_pos),
            bricks: Bricks::init(),
            paddle: Paddle::init(),
            frame,
            round: 0,
            has_started: false,
            is_paused: false,
            points: 0,
            pause_text: pause,
            pause_rect: Rectangle::new(
                vx2!(WIDTH * 0.5, HEIGHT * 0.5),
                vx2!(PAUSE_TEXT_WIDTH, PAUSE_TEXT_HEIGHT),
            ),
            last_ball_vel: vx2!(BALL_BASE_VEL_X, BALL_BASE_VEL_Y),
            transition: NewBallTransition::new(),
        }
    }

    pub fn reset_after_ball(&mut self) {
        self.paddle.reset();
        let mut ball_pos = self.paddle.rect.pos;
        let y_offset = self.paddle.rect.size.y * 0.5 + BALL_RADIUS + 2.0;
        ball_pos.y -= y_offset;
        self.ball.set_ball_pos(ball_pos);
        self.ball.set_ball_vel(vx2!(0.0));
        self.has_started = false;
        self.is_paused = false;
    }

    fn ball_velocity_hit_paddle(&mut self) {
        let s = self.ball.velocity.length();
        let pad_center = self.paddle.rect.pos.x;
        let rel_x = self.ball.distance_x(pad_center).clamp(-1.0, 1.0);
        let t = rel_x.signum() * rel_x.abs().powf(1.5);
        let f_max = 0.2;
        let vx = t * f_max * s;
        let vy = -(0.0f32.max(s * s - vx * vx).sqrt());
        let min_vy = 0.25 * s;
        if vy.abs() < min_vy {
            let sign_x = vx.signum();
            let vy_adj = -min_vy;
            let vx_adj = sign_x * 0.0f32.max(s * s - vy_adj * vy_adj).sqrt();
            self.ball.velocity.x = vx_adj;
            self.ball.velocity.y = vy_adj;
        } else {
            self.ball.velocity.x = vx;
            self.ball.velocity.y = vy;
        }
    }
}

impl GameControl for Play {
    fn update(
        &mut self,
        renderer: &mut grapes::renderer::two_d::Renderer,
        events: &grapes::events::input::Events,
    ) -> Option<Screen> {
        if events.key_down(K::Escape) {
            return Some(Screen::Quit);
        }

        if !self.transition.is_done() {
            let t = self.transition.next();
            let x = self.transition.start_x + (WIDTH * 0.5 - self.transition.start_x) * t;
            self.paddle.set_x(x);

            if t > 0.2 {
                let new_t = (t - 0.5) / 0.5;
                self.paddle.trans_color = Color::lerp(
                    &GrapesColors::Maroon.into(),
                    &GrapesColors::Teal.into(),
                    new_t,
                )
            } else {
                self.paddle.trans_color = GrapesColors::Maroon.into()
            }

            return None;
        }

        if self.transition.is_done() && !self.transition.idle {
            self.reset_after_ball();
            self.ball.show();
            self.paddle.in_trans = false;

            self.transition.set_idle();
            return None;
        }

        if events.key_pressed(K::Space) {
            if !self.has_started {
                self.ball
                    .set_ball_vel(get_rand_init_vel(self.last_ball_vel));
                self.has_started = true;
            } else {
                self.is_paused = !self.is_paused;
            }
        }

        if self.is_paused {
            return None;
        }

        self.paddle.update(renderer, events);
        self.ball.update(renderer, events);

        if !self.has_started {
            let mut ball_pos = self.paddle.rect.pos;
            let y_offset = self.paddle.rect.size.y * 0.5 + BALL_RADIUS + 2.0;
            ball_pos.y -= y_offset;
            self.ball.set_ball_pos(ball_pos);
            return None;
        }

        if let Some(coll_data) = self.ball.hits_epa(&self.paddle.rect) {
            if coll_data.contact_a.y - coll_data.contact_b.y < f32::EPSILON {
                self.ball.velocity.x *= -1.0;
            } else {
                self.ball_velocity_hit_paddle();
            }
            self.last_ball_vel = self.ball.velocity;
            return None;
        }

        for i in 1..5 {
            if self.ball.hits(&self.frame[i]) {
                if i == 3 {
                    if self.round == 2 {
                        return Some(Screen::Menu);
                    } else {
                        self.round += 1;
                        self.transition.start(self.paddle.rect.pos.x);
                        self.ball.hide();
                        self.paddle.in_trans = true;
                        self.paddle.trans_color = GrapesColors::Maroon.into();

                        //self.reset_after_ball();
                        return None;
                    }
                } else if i == 4 {
                    //top -> reverse y
                    self.ball.velocity.y *= -1.0;
                } else {
                    self.ball.velocity.x *= -1.0;
                }
                self.last_ball_vel = self.ball.velocity;
                return None;
            }
        }

        if let Some(coll_data) = self.bricks.ball_hits(&self.ball) {
            self.bricks.update(renderer, events);
            if let Some(idx) = self.bricks.get_hit() {
                self.points = POINTS[idx];
            }
            if coll_data.contact_a.y - coll_data.contact_b.y < f32::EPSILON {
                self.ball.velocity.y *= -1.0;
            } else {
                self.ball.velocity.x *= -1.0;
            }
        }

        return None;
    }

    fn draw(&self, renderer: &mut grapes::renderer::two_d::Renderer) {
        if self.is_paused {
            self.pause_rect.with_texture(renderer, &self.pause_text);
        }

        self.paddle.draw(renderer);
        self.ball.draw(renderer);
        self.bricks.draw(renderer);
    }
    fn get_points(&mut self) -> Option<usize> {
        if self.points > 0 {
            let ret = Some(self.points);
            self.points = 0;
            ret
        } else {
            None
        }
    }
    fn set_velocity(&mut self, vel: grapes::linal::vertx2::VX2, level: f32) {
        self.ball.set_ball_vel(vel);
        self.last_ball_vel = vel;
        self.paddle.set_velocity(vx2!(0.0), level);
    }
    fn get_round(&self) -> Option<usize> {
        Some(self.round)
    }

    fn reset(&mut self) {
        self.reset_after_ball();
        self.ball.show();
        self.paddle.in_trans = false;
        self.transition.set_idle();
        self.bricks.reset();
        self.round = 0;
        self.has_started = false;
        self.is_paused = false;
    }
}
