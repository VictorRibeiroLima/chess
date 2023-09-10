use engine::board::Board;

fn move_generation_helper(depth: u8, board: &mut Board) -> u64 {
    if depth == 0 {
        return 1;
    }

    let legal_moves = board.legal_moves();
    let mut num_positions = 0;

    for (from, to) in legal_moves {
        let mut new_board = *board;
        new_board.move_piece(from, to);
        num_positions += move_generation_helper(depth - 1, &mut new_board);
    }

    return num_positions;
}

#[cfg(test)]
mod new_board {
    use engine::board::Board;

    use crate::test::move_generation_helper;

    #[test]
    fn one_depth() {
        let mut board = Board::new();
        let num_positions = move_generation_helper(1, &mut board);
        assert_eq!(num_positions, 20);
    }

    #[test]
    fn two_depth() {
        let mut board = Board::new();
        let num_positions = move_generation_helper(2, &mut board);
        assert_eq!(num_positions, 400);
    }

    #[test]
    fn three_depth() {
        let mut board = Board::new();
        let num_positions = move_generation_helper(3, &mut board);
        assert_eq!(num_positions, 8902);
    }

    #[test]
    fn four_depth() {
        let mut board = Board::new();
        let num_positions = move_generation_helper(4, &mut board);
        assert_eq!(num_positions, 197281);
    }

    #[test]
    fn five_depth() {
        let mut board = Board::new();
        // let num_positions = move_generation_helper(5, &mut board);
        //assert_eq!(num_positions, 4865609);
    }

    #[test]
    fn six_depth() {
        let mut board = Board::new();
        //let num_positions = move_generation_helper(6, &mut board);
        //assert_eq!(num_positions, 119060324);
    }
}