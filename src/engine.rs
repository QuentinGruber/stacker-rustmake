use array2d::Array2D;
use cgmath::Vector2;
use raylib::prelude::*;

pub fn cleanCurrentLine(
    board: &mut array2d::Array2D<i32>,
    current_line: usize,
) -> &mut array2d::Array2D<i32> {
    for column in 0..board.column_len() {
        let result = board.set(column, current_line, 0);
        assert_eq!(result, Ok(()));
    }
    return board;
}

pub fn createdBoard(lines: usize, columns: usize) -> array2d::Array2D<i32> {
    // Create an array filled with the same element.
    let mut board = Array2D::filled_with(0, lines, columns);
    assert_eq!(board.num_rows(), lines);
    assert_eq!(board.num_columns(), columns);
    assert_eq!(board[(0, 0)], 0);
    // Iterate over all rows or columns.
    println!("All elements:");
    for row_iter in board.columns_iter() {
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
            let result = board.get(column, row);
            match result {
                Some(result) => match result {
                    0 => color = Color::RED,
                    1 => color = Color::PURPLE,
                    2 => color = Color::BLUE,
                    3 => color = Color::GREEN,
                    _ => println!("Input does not equal any value"),
                },
                _ => println!("Index doesn't exist"),
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

pub fn checkLoosedLifes(board: &mut array2d::Array2D<i32>, current_line: usize) -> usize {
    if current_line == board.row_len() - 1 {
        return 0;
    } else {
        for column in 0..board.column_len() {
            let test: usize = match board.get(column, current_line) {
                Some(x) => match x {
                    1 => return 1,
                    2 => return 2,
                    3 => return 3,
                    _ => return 0,
                },
                None => return 0,
            };
        }
        return 1;
    }
}
