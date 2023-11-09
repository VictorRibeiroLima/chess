use engine::board::Board;

fn move_generation(target: u8, board: &mut Board) -> u64 {
    let num_positions = move_generation_helper(0, target, board);
    return num_positions;
}

fn move_generation_helper(depth: u8, target: u8, board: &mut Board) -> u64 {
    if depth == target {
        return 1;
    }

    let legal_moves = board.legal_moves();

    let mut num_positions = 0;

    for (from, to) in legal_moves {
        let mut board = *board;
        let result = board.move_piece(from, to);

        assert!(result.is_ok());

        let poss = move_generation_helper(depth + 1, target, &mut board);

        num_positions += poss;
    }

    return num_positions;
}

#[cfg(test)]
mod visualize {
    use std::str::FromStr;

    use engine::{board::Board, piece::position::Position};

    use crate::test::move_generation;

    #[test]
    fn visualize() {
        let mut board = Board::new();
        let from = Position::from_str("c2").unwrap();
        let to = Position::from_str("c4").unwrap();
        board.move_piece(from, to).unwrap();
        let num_positions = move_generation(4, &mut board);
        assert_eq!(num_positions, 240082);
    }
}

#[cfg(test)]
mod new_board {
    use engine::board::Board;

    use crate::test::move_generation;

    #[test]
    fn one_depth() {
        let mut board = Board::new();
        let num_positions = move_generation(1, &mut board);
        assert_eq!(num_positions, 20);
    }

    #[test]
    fn two_depth() {
        let mut board = Board::new();
        let num_positions = move_generation(2, &mut board);
        assert_eq!(num_positions, 400);
    }

    #[test]
    fn three_depth() {
        let mut board = Board::new();
        let num_positions = move_generation(3, &mut board);
        assert_eq!(num_positions, 8902);
    }

    #[test]
    fn four_depth() {
        let mut board = Board::new();
        let num_positions = move_generation(4, &mut board);
        assert_eq!(num_positions, 197281);
    }

    #[test]
    fn five_depth() {
        let mut board = Board::new();
        let num_positions = move_generation(5, &mut board);
        assert_eq!(num_positions, 4865609);
    }

    #[test]
    fn six_depth() {
        let mut _board = Board::new();
        //let num_positions = move_generation(6, &mut board);
        //assert_eq!(num_positions, 119060324);
    }
}
