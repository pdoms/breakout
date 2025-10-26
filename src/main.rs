mod breakout;
mod common;
mod screens;
mod sprites;
mod heading;

use std::{cell::RefCell, process, rc::Rc};

use breakout::Breakout;
use common::{HEIGHT, WIDTH};
use grapes::
    engine::{camera_2d::Camera2d, two_d::Engine}
;

fn main() {
    let camera = Rc::new(RefCell::new(Camera2d::default()));
    let mut engine = match Engine::<Breakout>::create_window(
        "Breakout",
        WIDTH as usize,
        HEIGHT as usize,
        camera.clone(),
    ) {
        Ok(e) => e,
        Err(err) => {
            println!("{err:?}");
            process::exit(1);
        }
    };
    match engine.run() {
        Ok(_) => (),
        Err(err) => {
            println!("{err:?}");
            process::exit(1);
        }
    };
}
