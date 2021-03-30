#![allow(non_snake_case)]
//#![windows_subsystem = "windows"]
use raylib::prelude::*;
use structopt::StructOpt;

mod engine;
mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Stacker");

    let mut player = Rectangle::new(0.0, 0.0, 0.0, 0.0);

    let mut camera = Camera2D {
        target: Vector2::new(player.x + 20.0, player.y + 20.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut board = engine::createdBoard(7, 15);
    let mut lifes = 3;

    while !rl.window_should_close() {
        engine::debugMovement(&mut rl, &mut player, &mut camera);
        // setup drawing stuff
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLUE);
        let mut d2 = d.begin_mode2D(camera);

        engine::drawBoard(&mut d2, &mut board)
    }
}
