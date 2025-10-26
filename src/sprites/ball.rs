use grapes::{
    colors::{color::Color, presets::GrapesColors},
    events::input::Events,
    linal::vertx2::VX2,
    objects::{
        Collision, SupportV, Vertices,
        circle::Circle,
        collision::epa::{EpaResult, EpaVertex},
    },
    renderer::two_d::{Render, Renderer},
    vx2,
};
use rand::Rng;

use crate::{
    common::{BALL_BASE_VEL_X, BALL_BASE_VEL_Y, BALL_RADIUS, GameControl, HEIGHT, WIDTH},
    screens::Screen,
};

pub struct Ball {
    pub circle: Circle,
    pub velocity: VX2,
    visible: bool,
    color: Color,
}

pub fn get_rand_init_vel(base: VX2) -> VX2 {
    let mut r = rand::rng();
    let x_dir = if r.random_bool(0.5) { base.x } else { -base.x };
    vx2!(x_dir, -base.y)
}

impl Ball {
    pub fn new(pos: VX2) -> Self {
        let circle = Circle::new(pos, BALL_RADIUS);

        Self {
            circle,
            velocity: vx2!(0.0, 0.0),
            visible: true,
            color: GrapesColors::Teal.into(),
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }
    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn set_ball_pos(&mut self, pos: VX2) {
        self.circle.pos = pos;
    }

    pub fn set_ball_vel(&mut self, vel: VX2) {
        self.velocity = vel;
    }

    pub fn move_ball(&mut self) {
        self.circle.pos += &self.velocity;
    }

    pub fn distance_x(&self, paddle_center: f32) -> f32 {
        self.circle.pos.x - paddle_center
    }

    pub fn hits<O: Vertices + SupportV + Sized>(&self, rec: &O) -> bool {
        self.circle.collides(rec)
    }
    pub fn hits_epa<O: Vertices + SupportV + Sized>(&self, rec: &O) -> Option<EpaResult> {
        self.circle.collides_epa(rec)
    }
}

impl GameControl for Ball {
    fn update(&mut self, renderer: &mut Renderer, events: &Events) -> Option<Screen> {
        self.move_ball();
        return None;
    }

    fn draw(&self, renderer: &mut Renderer) {
        if self.visible {
            self.circle.fill_clr(renderer, self.color);
        }
    }
}
