#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub mod board;
pub mod board_initialization;
pub mod selection;
pub mod moves;
pub mod castling;
pub mod promotion;
pub mod checkmate;

use board::*;
use board_initialization::*;
use selection::*;
use moves::*;
use castling::*;
use promotion::*;
use checkmate::*;

use std::io;

// an example of how the api could be used
pub fn example_program()
{
    let mut board = create_board(); // start by creating the board, make it mutable

    loop
    {
        draw_board(&board);

        if board.active_player == Color::White // use the "active_player" variable to see whose turn it is
        {
            println!("white's turn")
        }
        else
        {
            println!("black's turn")
        }

        loop
        {
            println!("do you want to castle? (q/k/no)");

            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Error");


            if input.trim() == "q" || input.trim() == "k"
            {
                let mut queenside = true;

                if input.trim() == "k"
                {
                    queenside = false;
                }


                if castle(&mut board, queenside) // returns true if the castling is legal, otherwise false
                {
                    println!("valid castling");

                    break;
                }
                
                println!("invalid castling");
            }
            else // if the player does not castle, then play some other move
            {
                play_move(&mut board);

                break;
            }
        }


        checkmate(&mut board); // run after every new move. this will update the "checkmate" variable in the board

        if board.checkmate != Color::None // check if there was a checkmate
        {
            if board.checkmate == Color::White
            {
                println!("white checkmate");
            }
            else
            {
                println!("black checkmate");
            }

            break;
        }
    }
}

fn play_move(board: &mut Board)
{
    let mut piece = empty_piece(&Position{ x: 0, y: 0 });

    loop
    {
        let piece_position = get_position("select piece (file, rank)");

        if select_piece(board, &mut piece, &piece_position) // returns true if the piece is valid, otherwise false
        {
            println!("valid piece");
            break;
        }

        println!("invalid piece");
    }


    println!("possible moves:");

    let possible_moves = get_valid_moves(board, &piece); // get all valid moves of the piece

    for position in possible_moves.iter()
    {
        println!("{0} {1}", position.x + 1, position.y + 1);
    }


    loop
    {
        let move_position = get_position("select move position (file, rank)");

        if make_move(board, &piece, &move_position, &possible_moves) // returns true if the move is legal, otherwise false
        {
            println!("valid move");

            break;
        }

        println!("invalid move");
    }


    if board.promotion // check if there is a promotion
    {
        loop
        {
            println!("promote to q/r/b/n");

            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Error");


            let new_piece_type;

            match input.trim()
            {
                "q" => new_piece_type = Piece_Type::Queen,
                "r" => new_piece_type = Piece_Type::Rook,
                "b" => new_piece_type = Piece_Type::Bishop,
                "n" => new_piece_type = Piece_Type::Knight,
                _ => new_piece_type = Piece_Type::None
            }

            if make_promotion(board, new_piece_type) // returns true if new_piece_type is a valid type, otherwise false
            {
                println!("valid promotion");

                break;
            }

            println!("invalid promotion");
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