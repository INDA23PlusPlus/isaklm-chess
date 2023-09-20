use crate::board::*;
use crate::promotion::*;

// only gets valid moves, i.e those that don't result in check
pub fn get_valid_moves(board: &Board, piece: &Piece) -> Vec<Position>
{
    let mut valid_moves: Vec<Position> = vec![];

    
    let possible_moves = get_all_moves(board, piece);

    for position in possible_moves.iter()
    {
        if !move_results_in_check(board, piece, position, &possible_moves)
        {
            valid_moves.push(position.clone());
        }
    }
    
    return valid_moves;
}

// gets all moves, including ones that would result in check
pub fn get_all_moves(board: &Board, piece: &Piece) -> Vec<Position>
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


    let mut blocked = false; // checks if the pawn is blocked from moving 2 squares forward

    {
        let position = Position{ x: piece.position.x, y: piece.position.y + offset_y };

        if inside_board(board, &position) && is_empty(board, &position)
        {
            add_single_move(board, piece, &position, possible_moves);
        }
        else
        {
            blocked = true;
        }
    }

    if piece.move_count == 0
    {
        let position = Position{ x: piece.position.x, y: piece.position.y + offset_y * 2 };

        if !blocked && inside_board(board, &position) && is_empty(board, &position)
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


    add_en_passant_moves(board, piece, possible_moves);
}

fn add_en_passant_moves(board: &Board, piece: &Piece, possible_moves: &mut Vec<Position>)
{
    let piece_x = piece.position.x as usize;

    if piece.piece_color == Color::White && piece.position.y == 4
    {
        if piece.position.x > 0 && board.white_en_passant_moves[piece_x - 1]
        {
            let position = Position{ x: piece.position.x - 1, y: piece.position.y + 1 };

            add_single_move(board, piece, &position, possible_moves)
        }
        if piece.position.x < (board.width - 1) && board.white_en_passant_moves[piece_x + 1]
        {
            let position = Position{ x: piece.position.x + 1, y: piece.position.y + 1 };
            
            add_single_move(board, piece, &position, possible_moves)
        }
    }
    else if piece.piece_color == Color::Black && piece.position.y == 3
    {
        if piece.position.x > 0 && board.black_en_passant_moves[piece_x - 1]
        {
            let position = Position{ x: piece.position.x - 1, y: piece.position.y - 1 };

            add_single_move(board, piece, &position, possible_moves)
        }
        if piece.position.x < (board.width - 1) && board.black_en_passant_moves[piece_x + 1]
        {
            let position = Position{ x: piece.position.x + 1, y: piece.position.y - 1 };
            
            add_single_move(board, piece, &position, possible_moves)
        }
    }
}

// removes the piece that was caught by en-passant from the board
fn catch_with_en_passant(board: &mut Board, piece: &Piece, position: &Position)
{
    if piece.piece_type == Piece_Type::Pawn
    {
        if board_piece(board, position).piece_type == Piece_Type::None && piece.position.x != position.x
        {
            let catch_position = Position{ x: position.x, y: piece.position.y };

            place_piece(board, &empty_piece(&catch_position));
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

// adds move in a line until it reaches a stop
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
        if board_piece(board, position).piece_color != piece.piece_color // can't move onto a piece of the same color
        {
            (*possible_moves).push(position.clone());
        }
    }
}


// returns true if the king is in check
pub fn check(board: &Board, player: Color) -> bool
{
    let mut enemy_color = Color::Black;

    if player == Color::Black
    {
        enemy_color = Color::White;
    }


    let mut king_position = Position{ x: -1, y: -1 };

    for piece in board.pieces.iter()
    {
        if piece.piece_type == Piece_Type::King && piece.piece_color == player
        {
            king_position = piece.position.clone();

            break;
        }
    }


    let mut enemy_moves: Vec<Position> = vec![];

    for piece in board.pieces.iter()
    {
        if piece.piece_color == enemy_color
        {
            let piece_moves = get_all_moves(board, &piece);

            for position in piece_moves
            {
                enemy_moves.push(position);
            }
        }
    }


    for position in enemy_moves
    {
        if position == king_position // checks if any of the enemy pieces can move to the king
        {
            return true;
        }
    }


    return false;
}

// returns true if the move results in a check
pub fn move_results_in_check(board: &Board, piece: &Piece, position: &Position, possible_moves: &Vec<Position>) -> bool
{
    let mut board_copy = board.clone();

    make_move(&mut board_copy, piece, position, possible_moves);

    return check(&board_copy, board.active_player.clone());
}


// returns true if the move was valid
pub fn make_move(board: &mut Board, piece: &Piece, position: &Position, possible_moves: &Vec<Position>) -> bool
{
    reset_en_passant(board);

    for move_position in possible_moves
    {
        if move_position.x == position.x && move_position.y == position.y
        {
            add_en_passant(board, piece, position);

            catch_with_en_passant(board, piece, position);

            check_for_promotion(board, piece, position);


            place_piece(board, &empty_piece(&piece.position));

            let mut new_piece = piece.clone();

            new_piece.position = position.clone();
            new_piece.move_count += 1;

            place_piece(board, &new_piece);


            if board.active_player == Color::White
            {
                board.active_player = Color::Black;
            }
            else
            {
                board.active_player = Color::White;
            }


            return true;
        }
    }

    return false;
}


// tests

#[cfg(test)]
mod tests
{
    use crate::board::*;
    use crate::board_initialization::*;
    use crate::moves::*;

    #[test]
    fn test_moves()
    {
        let board = create_board();

        for piece in board.pieces.iter()
        {
            let possible_moves = get_all_moves(&board, &piece);

            for position in possible_moves.iter()
            {
                if position.x < 0 || position.x > 7 || position.y < 0 || position.y > 7
                {
                    panic!("moves outside board!");
                }

                if piece.piece_color == board_piece(&board, &position).piece_color
                {
                    panic!("move to piece of same color!");
                }
            }


            let valid_moves = get_valid_moves(&board, &piece);

            for position in valid_moves.iter()
            {
                if move_results_in_check(&board, &piece, &position, &valid_moves)
                {
                    panic!("move results in check!");
                }
            }
        }
    }
}
