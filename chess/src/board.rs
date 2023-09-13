pub fn create_board(fen_string: &str) -> Board
{
    let mut board = Board
    {
        width: 8,
        height: 8,
        pieces: Vec::new(),
        active_player: Color::White
    };

    for y in 0..board.height
    {
        for x in 0..board.width
        {
            board.pieces.push(Piece{ 
                piece_type: Piece_Type::None, 
                piece_color: Color::None,
                position: Position{ x: x, y: y },
                move_count: 0
             }); // initialize the board to be empty
        }
    }


    let fen_segments = fen_string.split_whitespace().collect::<Vec<&str>>();


    let positions = fen_segments[0];

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


    if fen_segments[1] == "b"
    {
        board.active_player = Color::Black;
    }

    return board;
}

pub fn board_index(width: i32, position: &Position) -> usize
{
    return (position.y * width + position.x) as usize;
}

pub fn board_piece(board: &Board, position: &Position) -> Piece
{
    return board.pieces[board_index(board.width, position)].clone();
}

pub fn inside_board(board: &Board, position: &Position) -> bool
{
    let mut inside = false;

    if position.x >= 0 && position.x < board.width && position.y >= 0 && position.y < board.height
    {
        inside = true;
    }

    return inside;
}

#[derive(Debug, PartialEq, Eq)]
#[derive(Clone)]
pub enum Piece_Type
{
    None, King, Queen, Rook, Bishop, Knight, Pawn
}

#[derive(Debug, PartialEq, Eq)]
#[derive(Clone)]
pub enum Color
{
    None, White, Black
}

#[derive(Clone)]
pub struct Piece // a piece on the board, empty squares have None pieces
{
    pub piece_type: Piece_Type,
    pub piece_color: Color,
    pub position: Position,
    pub move_count: i32
}

#[derive(Clone)]
pub struct Position
{
    pub x: i32,
    pub y: i32
}

#[derive(Clone)]
pub struct Board
{
    pub width: i32,
    pub height: i32,
    pub pieces: Vec<Piece>,
    pub active_player: Color // only White or Black is ever used
}