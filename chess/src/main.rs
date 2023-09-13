use std::io;
use std::io::prelude::*;

use crate::board::*;
use crate::selection::*;
use crate::moves::*;
use crate::move_execution::*;

pub mod board;
pub mod selection;
pub mod moves;
pub mod move_execution;


fn main()
{
    let mut board: Board = create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    loop
    {
        draw_board(&board);


        let mut piece = Piece{ piece_type: Piece_Type::None, piece_color: Color::None, position: Position{ x: 0, y: 0 }, move_count: 0 };

        loop
        {
            let piece_position = get_position("select piece (file, rank)");

            if select_piece(&board, &mut piece, &piece_position)
            {
                println!("valid piece");
                break;
            }

            println!("invalid piece");
        }


        println!("possible moves:");

        let possible_moves = get_moves(&board, &piece);

        for position in possible_moves.iter()
        {
            println!("{0} {1}", position.x + 1, position.y + 1);
        }


        loop
        {
            let move_position = get_position("select move position (file, rank)");

            if make_move(&mut board, &piece, &move_position, &possible_moves)
            {
                println!("valid move");

                break;
            }

            println!("invalid move");
        }
    }
}

fn get_position(message: &str) -> Position
{
    println!("{}", message);


    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Error");


    let numbers = input.split_whitespace().collect::<Vec<&str>>();
        
    let files = numbers[0].parse::<i32>().unwrap();
    let ranks = numbers[1].parse::<i32>().unwrap();


    return Position{ x: files - 1, y: ranks - 1 };
}

fn draw_board(board: &Board)
{
    for y in (0..board.height).rev()
    {
        print!("{}+\n", "+---".repeat(board.width as usize));


        for x in 0..board.width
        {
            let piece = board_piece(board, &Position{ x: x, y: y });

            let mut symbol: char;

            match piece.piece_type
            {
                Piece_Type::King => symbol = 'k',
                Piece_Type::Queen => symbol = 'q',
                Piece_Type::Rook => symbol = 'r',
                Piece_Type::Bishop => symbol = 'b',
                Piece_Type::Knight => symbol = 'n',
                Piece_Type::Pawn => symbol = 'p',
                _ => symbol = ' '
            }

            if piece.piece_color == Color::White
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