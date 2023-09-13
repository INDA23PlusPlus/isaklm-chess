use crate::board::*;

pub fn get_moves(board: &Board, piece: &Piece) -> Vec<Position>
{
    let mut possible_moves: Vec<Position> = vec![];

    match piece.piece_type
    {
        Piece_Type::King => add_king_moves(board, piece, &mut possible_moves),
        Piece_Type::Queen => add_queen_moves(board, piece, &mut possible_moves),
        Piece_Type::Rook => add_rook_moves(board, piece, &mut possible_moves),
        Piece_Type::Bishop => add_bishop_moves(board, piece, &mut possible_moves),
        Piece_Type::Knight => add_knight_moves(board, piece, &mut possible_moves),
        Piece_Type::Pawn => add_pawn_moves(board, piece, &mut possible_moves),
        _ => { },
    }

    return possible_moves;
}

pub fn add_king_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    let offsets_x = [ 0, 1, 1, 1, 0, -1, -1, -1 ];
    let offsets_y = [ 1, 1, 0, -1, -1, -1, 0, 1 ];

    for i in 0..offsets_x.len()
    {
        let position = Position{ x: piece.position.x + offsets_x[i], y: piece.position.y + offsets_y[i] };

        add_single_move(board, piece, &position, possible_moves);
    }
}

pub fn add_queen_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    add_vertical_and_horizontal_moves(board, piece, possible_moves);
    add_diagonal_moves(board, piece, possible_moves);
}

pub fn add_rook_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    add_vertical_and_horizontal_moves(board, piece, possible_moves);
}

pub fn add_bishop_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    add_diagonal_moves(board, piece, possible_moves);
}

pub fn add_knight_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    let offsets_x = [ 1, 2, 2, 1, -1, -2, -2, -1 ];
    let offsets_y = [ 2, 1, -1, -2, 2, 1, -1, -2 ];

    for i in 0..offsets_x.len()
    {
        let position = Position{ x: piece.position.x + offsets_x[i], y: piece.position.y + offsets_y[i] };

        add_single_move(board, piece, &position, possible_moves);
    }
}

// this is especially bad, will improve later
pub fn add_pawn_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    let offset_y;


    if piece.piece_color == Color::White
    {
        offset_y = 1;
    }
    else
    {
        offset_y = -1;
    }


    {
        let position = Position{ x: piece.position.x, y: piece.position.y + offset_y };

        if inside_board(board, &position) && (board_piece(board, &position).piece_color == Color::None)
        {
            add_single_move(board, piece, &position, possible_moves);
        }
    }

    if piece.move_count == 0
    {
        let position = Position{ x: piece.position.x, y: piece.position.y + offset_y * 2 };

        if inside_board(board, &position) && (board_piece(board, &position).piece_color == Color::None)
        {
            add_single_move(board, piece, &position, possible_moves);
        }
    }


    let mut enemy_color = Color::Black;

    if piece.piece_color == Color::Black
    {
        enemy_color = Color::White;
    }


    {
        let position = Position{ x: piece.position.x + 1, y: piece.position.y + offset_y };

        if inside_board(board, &position) && (board_piece(board, &position).piece_color == enemy_color)
        {
            add_single_move(board, piece, &position, possible_moves);
        }
    }

    {
        let position = Position{ x: piece.position.x - 1, y: piece.position.y + offset_y };

        if inside_board(board, &position) && (board_piece(board, &position).piece_color == enemy_color)
        {
            add_single_move(board, piece, &position, possible_moves);
        }
    }
}

pub fn add_vertical_and_horizontal_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    add_moves_in_line(board, piece, possible_moves, 1, 0);
    add_moves_in_line(board, piece, possible_moves, -1, 0);
    add_moves_in_line(board, piece, possible_moves, 0, 1);
    add_moves_in_line(board, piece, possible_moves, 0, -1);
}

pub fn add_diagonal_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    add_moves_in_line(board, piece, possible_moves, 1, 1);
    add_moves_in_line(board, piece, possible_moves, -1, 1);
    add_moves_in_line(board, piece, possible_moves, 1, -1);
    add_moves_in_line(board, piece, possible_moves, -1, -1);
}

pub fn add_moves_in_line(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>, step_x: i32, step_y: i32)
{
    let mut offset_x: i32 = 0;
    let mut offset_y: i32 = 0;

    loop
    {
        offset_x += step_x;
        offset_y += step_y;

        let position = Position{ x: piece.position.x + offset_x, y: piece.position.y + offset_y };

        add_single_move(board, piece, &position, possible_moves);

        if (!inside_board(board, &position)) || (board_piece(board, &position).piece_type != Piece_Type::None)
        {
            break;
        }
    }
}

pub fn add_single_move(board: &Board, piece: &Piece, position: &Position, possible_moves: &mut Vec<Position>)
{
    if inside_board(board, position)
    {
        if board_piece(board, position).piece_color != piece.piece_color
        {
            (*possible_moves).push(position.clone());
        }
    }
}