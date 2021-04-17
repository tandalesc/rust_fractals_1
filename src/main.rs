use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self};
use ggez::ContextBuilder;

mod explorer;
mod fractal;

use crate::explorer::Explorer;

const DISPLAY_RES: (usize, usize) = (1000, 750);

pub fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Fractal Explorer", "Shishir Tandale")
        .window_setup(WindowSetup::default().title("Fractal Explorer"))
        .window_mode(WindowMode::default().dimensions(DISPLAY_RES.0 as f32, DISPLAY_RES.1 as f32))
        .build()
        .unwrap();
    let mut my_game = Explorer::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
