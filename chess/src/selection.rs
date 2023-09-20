use crate::board::*;
use crate::moves::*;

pub fn select_piece(board: &Board, piece: &mut Piece, position: &Position) -> bool
{
    let mut valid_piece = false;

    if inside_board(&board, &position)
    {
        *piece = board_piece(&board, &position);

        if piece.piece_color == board.active_player
        {
            let possible_moves = get_valid_moves(board, &(*piece).clone());

            if possible_moves.len() > 0
            {
                valid_piece = true;
            }
        }
    }

    return valid_piece;
}