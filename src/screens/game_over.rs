use grapes::{events::keyboard::K, objects::rectangle::Rectangle, textures::Texture, vx2};

use crate::common::GameControl;

use super::Screen;

pub struct GameOver {
    score: Texture,
    text_continue: Texture,
    text_quit: Texture,
    rectangle: Rectangle,
}

impl GameOver {
    pub fn init() -> Self {
        Self {
            score: Texture::init(vx2!(10.0)),
            text_continue: Texture::init(vx2!(10.0)),
            text_quit: Texture::init(vx2!(10.0)),
            rectangle: Rectangle::new(vx2!(10.0), vx2!(10.0)),
        }
    }
}

impl GameControl for GameOver {
    fn update(
        &mut self,
        renderer: &mut grapes::renderer::two_d::Renderer,
        events: &grapes::events::input::Events,
    ) -> Option<super::Screen> {
        match events.key_pressed(K::Escape) {
            true => return Some(Screen::Quit),
            false => return None,
        }
    }

    fn draw(&self, renderer: &mut grapes::renderer::two_d::Renderer) {
        todo!()
    }

    fn get_score(&mut self, score: usize) {
        
    }
}
