use crate::board::*;

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

pub fn castle(board: &mut Board, queenside: bool) -> bool
{
    let mut can_castle = false;


    let rank;

    if board.active_player == Color::White
    {
        rank = 0;
    }
    else
    {
        rank = board.height - 1;
    }


    let rook_position;
    let king_position = Position{ x: 4, y: rank };

    if queenside
    {
        rook_position = Position{ x: 0, y: rank };
    }
    else
    {
        rook_position = Position{ x: board.width - 1, y: rank };
    }


    if board_piece(board, &king_position).move_count == 0 && board_piece(board, &rook_position).move_count == 0
    {
        let min_x;
        let max_x;

        if queenside
        {
            min_x = rook_position.x + 1;
            max_x = king_position.x;
        }
        else
        {
            min_x = king_position.x + 1;
            max_x = rook_position.x;
        }


        let mut all_empty = true;

        for x in min_x..max_x
        {
            if !is_empty(board, &Position{ x: x, y: rank })
            {
                all_empty = false;
            }
        }

        if all_empty
        {
            let mut rook = board_piece(board, &rook_position);
            let mut king = board_piece(board, &king_position);

            let new_rook_position;
            let new_king_position;

            if queenside
            {
                new_king_position = Position{ x: 2, y: rank };
                new_rook_position = Position{ x: 3, y: rank };
            }
            else
            {
                new_king_position = Position{ x: 6, y: rank };
                new_rook_position = Position{ x: 5, y: rank };
            }

            king.position = new_king_position.clone();
            rook.position = new_rook_position.clone();
            king.move_count += 1;
            rook.move_count += 1;


            place_piece(board, &king);
            place_piece(board, &rook);
            place_piece(board, &empty_piece(&king_position));
            place_piece(board, &empty_piece(&rook_position));


            if board.active_player == Color::White
            {
                board.active_player = Color::Black;
            }
            else
            {
                board.active_player = Color::White;
            }

            can_castle = true;
        }
    }

    return can_castle;
}

fn check_for_promotion(board: &mut Board, piece: &Piece, position: &Position)
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