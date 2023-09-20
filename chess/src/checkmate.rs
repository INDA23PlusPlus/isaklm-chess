use crate::board::*;
use crate::moves::*;

pub fn checkmate(board: &mut Board)
{
    let mut enemy_moves: Vec<Position> = vec![];

    for piece in board.pieces.iter()
    {
        if piece.piece_color == board.active_player
        {
            let piece_moves = get_valid_moves(board, piece);

            for position in piece_moves
            {
                enemy_moves.push(position);
            }
        }
    }


    if enemy_moves.is_empty()
    {
        (*board).checkmate = board.active_player.clone();
    }
}