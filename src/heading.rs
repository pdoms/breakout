use grapes::{
    colors::{color::Color, presets::GrapesColors},
    fonts::Font,
    linal::vertx2::VX2,
    objects::{circle::Circle, rectangle::Rectangle},
    renderer::two_d::{Render, Renderer},
    textures::Texture,
    vx2,
};

use crate::common::{FRAME_OFFSET, GameControl, HEADING_SIZE, WIDTH};

const FS: f32 = 32.0;

const BALL_DELTA: f32 = 30.0;
const BALL_RADIUS: f32 = BALL_DELTA * 0.4;

pub struct Disp {
    texture: Texture,
    rectangle: Rectangle,
    fs: f32,
    background: Color,
}

impl Disp {
    pub fn new(size: VX2, pos: VX2, fs: f32, background: Color) -> Self {
        Self {
            rectangle: Rectangle::new(pos, size),
            texture: Texture::init_with_background_color(size, background),
            fs,
            background,
        }
    }

    pub fn set_display(&mut self, value: &[u8], font: &Font, color: Color) {
        self.texture.clear(self.background);
        font.render_into_texture(
            value,
            vx2!(4.0, self.fs + 4.0),
            self.fs,
            color,
            &mut self.texture,
        );
    }

    pub fn reset(&mut self) {
        self.texture.clear(self.background);
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.rectangle.with_texture(renderer, &self.texture);
    }
}

pub struct Heading {
    circle: Circle,
    color: Color,
    ball: usize,
    pub font: Font,
    displays: [Disp; 2],
}

impl Heading {
    pub fn reset(&mut self) {
        self.ball = 3;
        self.displays[0].set_display(b"Speed: 1", &self.font, GrapesColors::Teal.into());
        self.displays[1].set_display(b"Score: 0", &self.font, GrapesColors::Teal.into());
    }
    pub fn init() -> Self {
        let font = Font::load("./assets/NotoSansMono.ttf");
        let circle = Circle::new(
            vx2!(
                FRAME_OFFSET * 3.0 + BALL_DELTA,
                HEADING_SIZE * 0.5 + FRAME_OFFSET
            ),
            BALL_RADIUS,
        );
        let half_rect = (WIDTH - 2.0 * FRAME_OFFSET) * 0.15;
        let third = (WIDTH - 2.0 * FRAME_OFFSET) * 0.33;
        let mut displays = [
            Disp::new(
                vx2!(third, FS + 8.0),
                vx2!(third + half_rect, HEADING_SIZE * 0.5),
                FS,
                GrapesColors::GrapesBlack.into(),
            ),
            Disp::new(
                vx2!(third, FS + 8.0),
                vx2!(2.0 * third + half_rect, HEADING_SIZE * 0.5),
                FS,
                GrapesColors::GrapesBlack.into(),
            ),
        ];
        displays[0].set_display(b"Speed: 1", &font, GrapesColors::Teal.into());
        displays[1].set_display(b"Score: 0", &font, GrapesColors::Teal.into());
        Self {
            circle,
            color: GrapesColors::Teal.into(),
            ball: 3,
            font,
            displays,
        }
    }
    pub fn set_ball(&mut self, round: usize) {
        self.ball = 3 - round;
    }

    pub fn set_score(&mut self, p: usize) {
        self.displays[1].set_display(format!("Score: {p}").as_bytes(), &self.font, self.color);
    }
    pub fn set_speed(&mut self, s: usize) {
        self.displays[0].set_display(format!("Speed: {s}").as_bytes(), &self.font, self.color);
    }
}

impl GameControl for Heading {
    fn update(
        &mut self,
        renderer: &mut grapes::renderer::two_d::Renderer,
        events: &grapes::events::input::Events,
    ) -> Option<crate::screens::Screen> {
        todo!()
    }

    fn draw(&self, renderer: &mut grapes::renderer::two_d::Renderer) {
        for display in self.displays.iter() {
            display.render(renderer);
        }
        for i in 0..3 {
            let mut b = self.circle.clone();
            b.pos.x = (i + 1) as f32 * BALL_DELTA;
            b.fill_clr(renderer, Color::from(GrapesColors::GrapesBlack));
            b.draw_clr(renderer, self.color);

            if i < self.ball {
                //fill
                b.fill_clr(renderer, self.color);
            }
        }
    }
}
