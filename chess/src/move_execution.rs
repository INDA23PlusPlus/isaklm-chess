use crate::board::*;

pub fn make_move(board: &mut Board, piece: &Piece, position: &Position, possible_moves: &Vec<Position>) -> bool
{
    for move_position in possible_moves
    {
        if move_position.x == position.x && move_position.y == position.y
        {
            (*board).pieces[board_index(board.width, &piece.position)] = 
            Piece{ piece_type: Piece_Type::None, piece_color: Color::None, position: piece.position.clone(), move_count: 0 };

            (*board).pieces[board_index(board.width, &position)] =
            Piece{ piece_type: piece.piece_type.clone(), piece_color: piece.piece_color.clone(), position: position.clone(), move_count: piece.move_count + 1 };


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