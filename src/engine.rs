use array2d::Array2D;
use cgmath::Vector2;
use raylib::prelude::*;

pub fn createdBoard(lines: usize, columns: usize) -> array2d::Array2D<i32> {
    // Create an array filled with the same element.
    let mut board = Array2D::filled_with(0, lines, columns);
    assert_eq!(board.num_rows(), lines);
    assert_eq!(board.num_columns(), columns);
    assert_eq!(board[(0, 0)], 0);

    // set starting points TODO: fix les starting points
    /*let result = board.set(
        board.row_len() - 1,
        board.column_len() - ((board.column_len() / 4) as usize) - 1,
        1,
    );
    assert_eq!(result, Ok(()));
    let result = board.set(
        board.row_len() - 1,
        board.column_len() - (board.column_len() / 4) - 2,
        1,
    );
    assert_eq!(result, Ok(()));
    let result = board.set(
        board.row_len() - 1,
        board.column_len() - (board.column_len() / 4) - 3,
        1,
    );
    assert_eq!(result, Ok(()));*/
    // Iterate over all rows or columns.
    println!("All elements:");
    for row_iter in board.rows_iter() {
        for element in row_iter {
            print!("{} ", element);
        }
        println!();
    }
    return board;
}

pub fn drawBoard(
    d2: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
    board: &mut array2d::Array2D<i32>,
) -> () {
    let spaceBetween: i32 = 40;
    let startPos: Vector2<i32> = Vector2 { x: 110, y: 100 };
    for row in 0..board.row_len() {
        for column in 0..board.column_len() {
            let mut color: Color = Color::GREEN;
            let caseValue = 1; // TODO: obtenir la valeur de la case via le board
            match caseValue {
                0 => color = Color::RED,
                1 => color = Color::PURPLE,
                2 => color = Color::BLUE,
                3 => color = Color::GREEN,
                _ => println!("Input does not equal any value"),
            }
            d2.draw_rectangle_rec(
                Rectangle::new(
                    (startPos.x + (spaceBetween * (column as i32))) as f32,
                    (startPos.y + (spaceBetween * (row as i32))) as f32,
                    35.0,
                    35.0,
                ),
                color,
            );
        }
    }
}

pub fn debugMovement(
    rl: &mut raylib::RaylibHandle,
    player: &mut raylib::prelude::Rectangle,
    camera: &mut raylib::prelude::Camera2D,
) -> () {
    use raylib::consts::KeyboardKey::*;

    if rl.is_key_down(KEY_RIGHT) {
        player.x += 2.0;
        camera.offset.x -= 2.0;
    } else if rl.is_key_down(KEY_LEFT) {
        player.x -= 2.0;
        camera.offset.x += 2.0;
    }
    if rl.is_key_pressed(KEY_R) {
        // reset zoom
        camera.zoom = 1.0;
        camera.rotation = 0.0;
    }
    camera.zoom += rl.get_mouse_wheel_move() as f32 * 0.05;
    // Limit camera rotation to 80 degrees
    camera.rotation = camera.rotation.max(-40.0).min(40.0);
    // zoom controls
    camera.zoom = camera.zoom.max(0.1).min(3.0);
}
