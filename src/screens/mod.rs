use std::ops::{Index, IndexMut};

use grapes::{
    colors::presets::GrapesColors, fonts::Font, objects::line::Line2d, textures::Texture, vx2,
};
use menu::Menu;
use play::Play;

use crate::common::{
    GameControl, PAUSE_FS, PAUSE_MARGIN_LEFT, PAUSE_TEXT_HEIGHT, PAUSE_TEXT_WIDTH,
};

pub mod menu;
pub mod play;

#[derive(Clone, PartialEq, Eq)]
pub enum Screen {
    Menu,
    Play,
    GameOver,
    Quit,
}

pub struct Screens {
    screens: [Box<dyn GameControl>; 2],
}

impl Screens {
    pub fn init(frame: [Line2d; 5]) -> Self {
        let font = Font::load("./assets/NotoSansMono.ttf");
        let menu = Menu::init(&font);
        let mut pause_texture = Texture::init_with_background_color(
            vx2!(PAUSE_TEXT_WIDTH, PAUSE_TEXT_HEIGHT),
            GrapesColors::GrapesBlack.into(),
        );
        font.render_into_texture(
            b"PAUSED",
            vx2!(PAUSE_MARGIN_LEFT, PAUSE_FS),
            PAUSE_FS,
            GrapesColors::Maroon.into(),
            &mut pause_texture,
        );
        let play = Play::init(frame, pause_texture);

        Self {
            screens: [Box::new(menu), Box::new(play)],
        }
    }
}

impl Index<&Screen> for Screens {
    type Output = Box<dyn GameControl>;

    fn index(&self, index: &Screen) -> &Self::Output {
        match index {
            Screen::Menu => &self.screens[0],
            Screen::Play => &self.screens[1],
            _ => panic!(),
        }
    }
}

impl IndexMut<&Screen> for Screens {
    fn index_mut(&mut self, index: &Screen) -> &mut Self::Output {
        match index {
            Screen::Menu => &mut self.screens[0],
            Screen::Play => &mut self.screens[1],
            _ => panic!(),
        }
    }
}
