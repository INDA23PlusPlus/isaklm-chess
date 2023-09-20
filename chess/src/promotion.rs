use crate::board::*;

pub fn check_for_promotion(board: &mut Board, piece: &Piece, position: &Position)
{
    board.promotion = false; // reset it

    if piece.piece_type == Piece_Type::Pawn
    {
        if (piece.piece_color == Color::White && position.y == 7) || (piece.piece_color == Color::Black && position.y == 0)
        {
            board.promotion = true;
        }
    }
}

pub fn make_promotion(board: &mut Board, position: &Position, new_piece_type: Piece_Type) -> bool
{
    if new_piece_type == Piece_Type::Queen ||
    new_piece_type == Piece_Type::Rook ||
    new_piece_type == Piece_Type::Bishop ||
    new_piece_type == Piece_Type::Knight
    {
        let mut piece = board_piece(board, position);

        piece.piece_type = new_piece_type;


        place_piece(board, &piece);

        return true;
    }

    return false;
}