use crate::board::*;

pub fn create_board() -> Board
{
    let mut board = Board
    {
        width: 8,
        height: 8,
        pieces: Vec::new(),
        active_player: Color::White,

        white_en_passant_moves: Vec::new(),
        black_en_passant_moves: Vec::new(),

        promotion: false,
        promotion_position: Position{ x: -1, y: -1 },

        checkmate: Color::None
    };

    for y in 0..board.height
    {
        for x in 0..board.width
        {
             board.pieces.push(empty_piece(&Position{ x: x, y: y })); // initialize the board to be empty
        }
    }


    let positions = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    let mut y: i32 = board.height - 1;
    let mut x: i32 = 0;

    for (_i, symbol) in positions.chars().enumerate()
    {
        if symbol == '/'
        {
            y -= 1;
            x = 0;
        }
        else if symbol.is_numeric()
        {
            x += symbol.to_digit(10).unwrap() as i32;
        }
        else if symbol.is_alphabetic()
        {
            let piece_type;
            let piece_color;

            if symbol.is_ascii_uppercase()
            {
                piece_color = Color::White;
            }
            else 
            {
                piece_color = Color::Black;
            }

            match symbol.to_ascii_lowercase()
            {
                'k' => piece_type = Piece_Type::King,
                'q' => piece_type = Piece_Type::Queen,
                'r' => piece_type = Piece_Type::Rook,
                'b' => piece_type = Piece_Type::Bishop,
                'n' => piece_type = Piece_Type::Knight,
                'p' => piece_type = Piece_Type::Pawn,
                _ => piece_type = Piece_Type::None
            }


            let index = board_index(board.width, &Position{ x: x, y: y });

            board.pieces[index].piece_type = piece_type;
            board.pieces[index].piece_color = piece_color;

            x += 1;
        }
    }


    for _i in 0..board.width
    {
        board.white_en_passant_moves.push(false);
        board.black_en_passant_moves.push(false);
    }

    return board;
}