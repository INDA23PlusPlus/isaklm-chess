use crate::board::*;
use crate::moves::*;

// returns true if the selected piece is valid, i.e it belongs to the player whose turn it is
pub fn select_piece(board: &Board, piece: &mut Piece, position: &Position) -> bool
{
    if inside_board(&board, &position)
    {
        *piece = board_piece(&board, &position);

        if piece.piece_color == board.active_player
        {
            let possible_moves = get_valid_moves(board, &(*piece).clone());

            if possible_moves.len() > 0 // can't select a piece that has no valid moves
            {
                return true;
            }
        }
    }

    return false;
}