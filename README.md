__HOW TO USE__

First initialize the board:
  **let mut board = create_board();**


To select a piece first create an "empty" one
  **let mut piece = empty_piece(&Position{ x: 0, y: 0 });**

then call this function with the position of the piece that you want to select (x and y should be between 0 and 7)
  **select_piece(&board, &mut piece, &piece_position)**

This function will return **true** if the selection is valid, otherwise **false**. Choose a new position if it wasn't valid.


To get the legal moves for the piece, call this function:
  **let possible_moves = get_valid_moves(&board, &piece);**


Pick a move (i.e. position) from this array of moves and then call this function to make the move:
  **make_move(&board, &piece, &move_position, &possible_moves)**

This function will return **true** if the selected move is a part of the possible moves, otherwise **false**.


After you have made a move you should check for checkmate. To do this first call this function:
  **checkmate(&mut board);**

Then check if **board.checkmate == Color::White** or **board.checkmate == Color::Black** to see if White or Black got checkmate.
If neither White or Black got checkmate then the variable will be set to **Color::None**.



__HOW TO HANDLE SPECIAL MOVES__

PROMOTION:

After you make a move you can check the boolean variable board.promotion to see if you should promote.

If the variable is **true**, then pick a **Piece_Type** (Queen, Rook, Bishop, Knight) and call this function:
  **make_promotion(&board, new_piece_type)**

The function will return **true** if you selected a valid **Piece_Type** to promote to, otherwise **false**.


CASTLING:

To castle call the function:
**pub fn castle(board: &mut Board, queenside: bool) -> bool**

For the parameter *queenside*, pass in **true** to castle queenside and **false** to castle kingside.

If you can't castle then the function will return **false**.


EN PASSANT:

Handled just like the normal moves, nothing extra needs to be done.
