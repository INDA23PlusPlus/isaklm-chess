use std::io;
use std::io::prelude::*;

use crate::board::*;

pub mod board;


fn main()
{
    let board: Board = initialize_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    loop
    {
        draw_board(&board);

        let piece_position = get_position("select piece (file, rank)");
    }
}

fn get_position(message: &str) -> Position
{
    println!("{}", message);


    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Error");


    let numbers = input.split_whitespace().collect::<Vec<&str>>();
        
    let files = numbers[0].parse::<u32>().unwrap();
    let ranks = numbers[1].parse::<u32>().unwrap();


    return Position{ x: files - 1, y: ranks - 1 };
}

fn draw_board(board: &Board)
{
    for y in (0..board.height).rev()
    {
        print!("{}+\n", "+---".repeat(board.width as usize));


        for x in 0..board.width
        {
            let square = board_piece(board, &Position{ x: x, y: y });

            let mut symbol: char;

            match square.piece_type
            {
                Piece_Type::King => symbol = 'k',
                Piece_Type::Queen => symbol = 'q',
                Piece_Type::Rook => symbol = 'r',
                Piece_Type::Bishop => symbol = 'b',
                Piece_Type::Knight => symbol = 'n',
                Piece_Type::Pawn => symbol = 'p',
                _ => symbol = ' '
            }

            if square.piece_color == Color::White
            {
                symbol = symbol.to_ascii_uppercase();
            }

            print!("| {} ", symbol);
        }

        print!("| {}\n", y + 1);
    }
    

    print!("{}+\n", "+---".repeat(board.width as usize));

    for x in 0..board.width
    {
        print!("  {} ", x + 1);
    }

    println!("");
}