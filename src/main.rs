#![allow(non_snake_case)]
#![windows_subsystem = "windows"]
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::time::Instant;
use structopt::StructOpt;

mod engine;
mod options;

fn main() {
    let opt = options::Opt::from_args();
    let (mut rl, thread) = opt.open_window("Stacker");
    let player = Rectangle::new(0.0, 0.0, 0.0, 0.0);

    let camera = Camera2D {
        target: Vector2::new(player.x + 20.0, player.y + 20.0),
        offset: Vector2::new(0.0, 0.0),
        rotation: 0.0,
        zoom: 1.0,
    };
    let mut board = engine::createdBoard(7, 15);
    let mut lifes = 3;
    let mut pos = board.column_len() - ((board.row_len() / 3) as usize);
    let mut current_line = board.row_len() - 1;
    let mut go_right = true;
    let mut time = Instant::now();
    let mut move_speed = 0.5;
    while !rl.window_should_close() {
        for life in 0..lifes {
            let result = board.set(pos + life, current_line, 1);
            assert_eq!(result, Ok(()));
        }
        if rl.is_key_down(KEY_SPACE) {
            move_speed /= 1.15;
            let loosed_lifes: usize = engine::checkLoosedLifes(&mut board, current_line);
            lifes -= loosed_lifes;
            current_line -= 1;
        }
        // setup drawing stuff
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLUE);
        let mut d2 = d.begin_mode2D(camera);
        engine::drawBoard(&mut d2, &mut board);
        if time.elapsed().as_secs_f64() > move_speed {
            engine::cleanCurrentLine(&mut board, current_line);
            if go_right {
                if pos <= board.column_len() - (lifes + 1) {
                    pos += 1;
                } else {
                    go_right = false;
                };
            } else {
                if pos > 0 {
                    pos -= 1;
                } else {
                    go_right = true;
                };
            }
            time = Instant::now();
        }
    }
}
