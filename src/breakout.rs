use std::io::Write;

use grapes::events::input::Events;
use grapes::{
    colors::presets::GrapesColors,
    objects::line::Line2d,
    renderer::two_d::{Render, Renderer},
    state::two_d::State,
    vx2,
};

use crate::{
    common::{
        BALL_BASE_VEL_X, BALL_BASE_VEL_Y, FRAME_OFFSET, GameControl, HEADING_SIZE, HEIGHT, WIDTH,
    },
    heading::Heading,
    screens::{Screen, Screens},
};

pub fn get_level(score: usize) -> (f32, usize) {
    match score {
        0..100 => (1.0, 1),
        100..300 => (1.5, 2),
        300..600 => (2.0, 3),
        600..800 => (3.0, 4),
        800..1000 => (4.0, 5),
        _ => (5.0, 6),
    }
}

//TODO
// - init speed as level with 1
// - show that ball is lost
// - goodbye stuff

pub struct Breakout {
    /// 0: (top_left, top_right)
    /// 1: (top_right, bottom_right)
    /// 2: (bottom_left, top_left)
    /// 3: (bottom_right, bottom_left)
    /// 4: (game_top_left, game_top_right)
    frame: [Line2d; 5],
    screen: Screen,
    screens: Screens,
    heading: Heading,
    score: usize,
    level: usize,
    started: bool,
}

impl Breakout {
    pub fn reset(&mut self) {
        self.score = 0;
        self.level = 1;
        self.heading.reset();
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        let fg = GrapesColors::Teal;
        //the borders are displayed on every screen
        for (i, border) in self.frame.iter().enumerate() {
            if i < 3 || (self.screen == Screen::Play && i == 4) {
                border.draw_clr(renderer, fg);
            } else {
                border.draw_clr(renderer, GrapesColors::GrapesBlack);
            }
        }
        //draw the current screen
        self.screens[&self.screen].draw(renderer);
        if self.screen == Screen::Play {
            self.heading.draw(renderer);
        }
    }

    fn update(&mut self, renderer: &mut Renderer, events: Events) -> bool {
        if let Some(screen) = self.screens[&self.screen].update(renderer, &events) {
            self.screen = screen;
            if self.screen == Screen::Play {
                println!("[INFO] reset initialized");
                self.reset();
                self.screens[&self.screen].reset();
            }
        }

        if self.screen == Screen::Quit {
            return true;
        }

        if self.screen == Screen::Play {
            if !self.started {
                self.started = true;
            }
            if let Some(round) = self.screens[&self.screen].get_round() {
                self.heading.set_ball(round);
            }

            let mut check_speed = false;
            if let Some(points) = self.screens[&self.screen].get_points() {
                self.score += points;
                self.heading.set_score(self.score);
                check_speed = true;
            }
            if check_speed {
                let cur = self.level;
                let (multi, level) = get_level(self.score);
                if level > cur {
                    self.level = level;
                    let v = vx2!(multi * BALL_BASE_VEL_X, multi * BALL_BASE_VEL_Y);
                    self.screens[&self.screen].set_velocity(v, multi);
                    self.heading.set_speed(self.level);
                }
            }
        }

        if self.screen == Screen::Menu && self.started {
            self.screens[&self.screen].prepare_render_score(self.score, &self.heading.font);
        }

        return false;
    }
}

impl State for Breakout {
    fn user_init(
        _renderer: &mut grapes::renderer::two_d::Renderer,
        _camera: grapes::engine::camera_2d::Camera2dRef,
    ) -> Self {
        let top_left = vx2!(FRAME_OFFSET);
        let top_right = grapes::linal::vertx2::VX2::new(WIDTH - FRAME_OFFSET, FRAME_OFFSET);
        let bottom_right = vx2!(WIDTH - FRAME_OFFSET, HEIGHT - FRAME_OFFSET);
        let bottom_left = vx2!(FRAME_OFFSET, HEIGHT - FRAME_OFFSET);
        let game_top_left = vx2!(FRAME_OFFSET, HEADING_SIZE);
        let game_top_right = vx2!(WIDTH - FRAME_OFFSET, HEADING_SIZE);
        let frame = [
            Line2d::new(top_left, top_right),
            Line2d::new(top_right, bottom_right),
            Line2d::new(bottom_left, top_left),
            Line2d::new(bottom_right, bottom_left),
            Line2d::new(game_top_left, game_top_right),
        ];
        Breakout {
            frame: frame.clone(),
            screen: Screen::Menu,
            screens: Screens::init(frame),
            heading: Heading::init(),
            score: 0,
            level: 1,
            started: false,
        }
    }

    fn user_update(
        &mut self,
        renderer: &mut grapes::renderer::two_d::Renderer,
        events: grapes::events::input::Events,
        _camera: grapes::engine::camera_2d::Camera2dRef,
        _time_info: &grapes::context::time::TimeInfo,
    ) -> bool {
        if self.update(renderer, events) {
            return true;
        }
        renderer.clear_background(GrapesColors::GrapesBlack);
        self.draw(renderer);

        //        let buf = format!("\r{}", time_info.get_average_fps());
        //        let mut stdout = std::io::stdout().lock();
        //        stdout.write(buf.as_bytes()).unwrap();
        //        stdout.flush().unwrap();
        return false;
    }
}
