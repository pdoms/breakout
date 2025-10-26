use grapes::{
    colors::presets::GrapesColors,
    events::keyboard::K,
    fonts::Font,
    objects::rectangle::Rectangle,
    renderer::two_d::{Render, Renderer},
    textures::Texture,
    vx2,
};

use crate::common::{GameControl, HEIGHT, MENU_ACTION, MENU_QUIT, MENU_SCORE, MENU_TITLE, WIDTH};

use super::Screen;

//TODO ADD Hit Escape at any time to Quit
pub struct Menu {
    title: Texture,
    title_rect: Rectangle,
    quit: Texture,
    quit_rect: Rectangle,
    score: Texture,
    score_rect: Rectangle,
    action: Texture,
    action_rect: Rectangle,
    render_score: bool,
}

impl Menu {
    pub fn init(font: &Font) -> Self {
        let title_width = font.width(MENU_TITLE, 48.0);
        let action_width = font.width(MENU_ACTION, 24.0);
        let quit_width = font.width(MENU_QUIT, 14.0);
        let score_width = font.width(MENU_SCORE, 32.0);

        let mut title = Texture::init_with_background_color(
            vx2!(title_width + 2.0 * 8.0, 60.0),
            GrapesColors::GrapesBlack.into(),
        );

        let title_rect = Rectangle::new(
            vx2!(WIDTH * 0.5, 300.0),
            vx2!(title_width + 2.0 * 8.0, 60.0),
        );

        font.render_into_texture(
            MENU_TITLE,
            vx2!(8.0, 56.0),
            48.0,
            GrapesColors::Teal.into(),
            &mut title,
        );

        let mut quit = Texture::init_with_background_color(
            vx2!(quit_width + 2.0 * 8.0, 50.0),
            GrapesColors::GrapesBlack.into(),
        );

        let quit_rect = Rectangle::new(
            vx2!(WIDTH * 0.5, HEIGHT - 50.0),
            vx2!(quit_width + 2.0 * 8.0, 50.0),
        );

        font.render_into_texture(
            MENU_QUIT,
            vx2!(8.0, 18.0),
            14.0,
            GrapesColors::Teal.into(),
            &mut quit,
        );

        let mut action = Texture::init_with_background_color(
            vx2!(action_width + 2.0 * 8.0, 50.0),
            GrapesColors::GrapesBlack.into(),
        );

        let action_rect = Rectangle::new(
            vx2!(WIDTH * 0.5, HEIGHT - 150.0),
            vx2!(action_width + 2.0 * 8.0, 50.0),
        );

        font.render_into_texture(
            MENU_ACTION,
            vx2!(8.0, 28.0),
            24.0,
            GrapesColors::Teal.into(),
            &mut action,
        );

        let score = Texture::init_with_background_color(
            vx2!(score_width + 2.0 * 8.0, 84.0),
            GrapesColors::GrapesBlack.into(),
        );

        let score_rect = Rectangle::new(
            vx2!(WIDTH * 0.5, HEIGHT - 300.0),
            vx2!(score_width + 2.0 * 8.0, 84.0),
        );

        Self {
            title,
            title_rect,
            action,
            action_rect,
            quit,
            quit_rect,
            score,
            score_rect,
            render_score: false,
        }
    }
}

impl GameControl for Menu {
    fn update(
        &mut self,
        _renderer: &mut Renderer,
        events: &grapes::events::input::Events,
    ) -> Option<Screen> {
        if events.key_down(K::Enter) {
            return Some(Screen::Play);
        }
        if events.key_pressed(K::Escape) {
            return Some(Screen::Quit);
        }
        return None;
    }

    fn draw(&self, renderer: &mut Renderer) {
        self.title_rect.with_texture(renderer, &self.title);
        self.action_rect.with_texture(renderer, &self.action);
        self.quit_rect.with_texture(renderer, &self.quit);
        if self.render_score {
            self.score_rect.with_texture(renderer, &self.score);
        }
    }

    fn prepare_render_score(&mut self, score: usize, font: &Font) {
        self.render_score = true;
        self.score.clear(GrapesColors::GrapesBlack.into());

        let sscore = score.to_string();
        let width = font.width(sscore.as_bytes(), 32.0);

        let texture_width = self.score.size().x;
        let color = GrapesColors::Teal;

        font.render_into_texture(
            MENU_SCORE,
            vx2!(8.0, 40.0),
            32.0,
            color.into(),
            &mut self.score,
        );

        let x_pos = (texture_width * 0.5) - (width * 0.5);
        font.render_into_texture(
            sscore.as_bytes(),
            vx2!(x_pos, 80.0),
            32.0,
            color.into(),
            &mut self.score,
        );
    }
}
