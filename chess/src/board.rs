pub fn board_index(width: i32, position: &Position) -> usize
{
    return (position.y * width + position.x) as usize;
}

pub fn board_piece(board: &Board, position: &Position) -> Piece
{
    return board.pieces[board_index(board.width, position)].clone();
}

pub fn place_piece(board: &mut Board, piece: &Piece)
{
    (*board).pieces[board_index(board.width, &piece.position)] = piece.clone();
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

pub fn is_empty(board: &Board, position: &Position) -> bool
{
    return board_piece(board, position).piece_type == Piece_Type::None;
}

pub fn reset_en_passant(board: &mut Board)
{
    for i in 0..board.width
    {
        board.white_en_passant_moves[i as usize] = false;
        board.black_en_passant_moves[i as usize] = false;
    }
}

pub fn add_en_passant(board: &mut Board, piece: &Piece, position: &Position)
{
    let move_y = position.y as usize;

    if piece.piece_type == Piece_Type::Pawn && piece.move_count == 0
    {
        if piece.piece_color == Color::White && move_y == 3
        {
            board.black_en_passant_moves[piece.position.x as usize] = true;
        }
        else if piece.piece_color == Color::Black && move_y == 4
        {
            board.white_en_passant_moves[piece.position.x as usize] = true;
        }
    }
}

pub fn empty_piece(position: &Position) -> Piece
{
    return Piece{ piece_type: Piece_Type::None, piece_color: Color::None, position: position.clone(), move_count: 0 };
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

#[derive(Debug, PartialEq, Eq)]
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
    pub active_player: Color, // only White or Black is ever used

    pub white_en_passant_moves: Vec<bool>,
    pub black_en_passant_moves: Vec<bool>,

    pub promotion: bool,

    pub checkmate: Color
}