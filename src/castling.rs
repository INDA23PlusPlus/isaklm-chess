use crate::board::*;
use crate::moves::*;

pub fn castle(board: &mut Board, queenside: bool) -> bool
{
    if check(board, board.active_player.clone()) // if the king is in check then castling is illegal
    {
        return false;
    }


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

    // checks that the rook and king haven't moved
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


        let mut king_can_pass_trough = true;

        for x in min_x..max_x
        {
            let test_position = Position{ x: x, y: rank };

            if !is_empty(board, &test_position)
            {
                king_can_pass_trough = false;
            }
            else if test_for_check(board, &king_position, &test_position) // tests if the king passes through a square that is in check
            {
                king_can_pass_trough = false;
            }
        }

        if king_can_pass_trough
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

            
            let mut board_copy = board.clone();

            place_piece(&mut board_copy, &king);
            place_piece(&mut board_copy, &rook);
            place_piece(&mut board_copy, &empty_piece(&king_position));
            place_piece(&mut board_copy, &empty_piece(&rook_position));


            if !check(&board_copy, board.active_player.clone())
            {
                *board = board_copy;

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
    }

    return false;
}

// tests if the king passes through a square that is in check
fn test_for_check(board: &mut Board, king_position: &Position, test_position: &Position) -> bool
{
    let mut king = board_piece(board, king_position);

    king.position = test_position.clone();
    king.move_count += 1;


    let mut board_copy = board.clone();

    place_piece(&mut board_copy, &king);
    place_piece(&mut board_copy, &empty_piece(&king_position));


    return check(&board_copy, board_copy.active_player.clone());
}