use engine::{
    board::Board,
    piece::{position::Position, ChessPiece},
};

#[cfg(test)]
mod test;

pub fn make_move(board: &Board) -> (Position, Position) {
    let valid_moves = board.legal_moves();
    let rand_value = rand::random::<usize>() % valid_moves.len();
    valid_moves[rand_value]
}

pub fn make_promotion(board: &Board) -> ChessPiece {
    return ChessPiece::create_queen(board.get_turn());
}
