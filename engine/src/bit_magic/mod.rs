use crate::board::state::Color;

const FIRST_BIT: u8 = 1;
const LAST_BIT: u8 = 128;

pub fn vertical_move_bb(from: u64, to: u64) -> u64 {
    let mut from = from;
    let to = to;
    let mut result = 0;
    while from != to {
        if from < to {
            from <<= 8;
        } else {
            from >>= 8;
        }
        if from != to {
            result |= from;
        }
    }
    result
}

pub fn horizontal_move_bb(from: u64, to: u64) -> u64 {
    let mut from = from;
    let to = to;
    let mut result = 0;
    while from != to {
        if from < to {
            from <<= 1;
        } else {
            from >>= 1;
        }
        if from != to {
            result |= from;
        }
    }
    result
}

pub fn diagonal_move_bb(from: u64, to: u64) -> u64 {
    let mut from = from;
    let to = to;
    let mut result = 0;

    let bit_distance = bit_distance(from, to);

    let bit_distance = if bit_distance % 9 == 0 { 9 } else { 7 };

    while from != to {
        if from < to {
            from <<= bit_distance;
        } else {
            from >>= bit_distance;
        }
        if from != to {
            result |= from;
        }
    }
    result
}

pub fn is_moving_vertically(from: u64, to: u64) -> bool {
    if from == to {
        return false;
    }
    let bit_distance = bit_distance(from, to);

    bit_distance % 8 == 0
}

pub fn is_moving_horizontally(from: u64, to: u64) -> bool {
    if from == to {
        return false;
    }

    let from_row = bit_row(from);
    let to_row = bit_row(to);

    from_row == to_row
}

pub fn is_moving_diagonally(from: u64, to: u64) -> bool {
    if from == to {
        return false;
    }
    let from_row = bit_row(from);
    let from_pos = ((from >> (from_row * 8)) & 0xFF) as u8;
    let bit_distance = bit_distance(from, to);
    if from_pos == FIRST_BIT {
        if from < to {
            return bit_distance % 9 == 0;
        } else {
            return bit_distance % 7 == 0;
        }
    } else if from_pos == LAST_BIT {
        if from > to {
            return bit_distance % 9 == 0;
        } else {
            return bit_distance % 7 == 0;
        }
    } else {
        return bit_distance % 9 == 0 || bit_distance % 7 == 0;
    }
}

pub fn is_l_shaped(from: u64, to: u64) -> bool {
    return true;
}

pub fn is_pawn_at_initial_position(pawn: u64, color: Color) -> bool {
    let white_mask = 0b1111_1111_0000_0000;
    match color {
        Color::White => pawn ^ white_mask < white_mask,
        Color::Black => {
            let black_mask = white_mask << 40;
            pawn ^ black_mask < black_mask
        }
    }
}

#[inline]
pub fn bit_row(number: u64) -> u8 {
    for i in 0..8 {
        let result = ((number >> (i * 8)) & 0xFF) as u8;
        if result != 0 {
            return i;
        }
    }
    return 0;
}

#[inline]
pub fn bit_distance(from: u64, to: u64) -> u32 {
    let greater;
    let leaser;
    if from > to {
        greater = from;
        leaser = to;
    } else {
        greater = to;
        leaser = from;
    }

    let bit_distance = (greater - leaser).count_ones();
    return bit_distance;
}

#[cfg(test)]
mod test {

    #[test]
    fn test_moving_forward() {
        let a2 = 0b0000_0000_0000_0001_0000_0000;
        let a3 = 0b0000_0001_0000_0000_0000_0000;
        let a4 = 0b0000_0001_0000_0000_0000_0000_0000_0000;

        assert!(super::is_moving_vertically(a2, a3));
        assert!(super::is_moving_vertically(a3, a2));
        assert!(super::is_moving_vertically(a3, a4));
        assert!(super::is_moving_vertically(a4, a3));
        assert!(super::is_moving_vertically(a2, a4));

        let b2 = 0b0000_0000_0000_0010_0000_0000;
        let b3 = 0b0000_0010_0000_0000_0000_0000;

        assert!(super::is_moving_vertically(b2, b3));
        assert!(super::is_moving_vertically(b3, b2));

        assert!(!super::is_moving_vertically(a2, b2));
    }

    #[test]
    fn test_pawn_initial_position() {
        let white_pawn = 0b0000_0000_0000_0001_0000_0000;
        let black_pawn = 0b0000_0000_0000_0001_0000_0000_0000_0000_0000_0000_0000;

        assert!(super::is_pawn_at_initial_position(
            white_pawn,
            super::Color::White
        ));
        assert!(!super::is_pawn_at_initial_position(
            black_pawn,
            super::Color::White
        ));

        assert!(!super::is_pawn_at_initial_position(
            white_pawn,
            super::Color::Black
        ));
        assert!(super::is_pawn_at_initial_position(
            black_pawn,
            super::Color::Black
        ));
    }

    #[test]
    fn test_vertical_bb() {
        let a2 = 0b0000_0000_0000_0001_0000_0000;
        let a6 = a2 << 32;
        let expect = 0b00000000000000000000000000000100000001000000010000000000000000;

        let bb = super::vertical_move_bb(a2, a6);

        assert_eq!(bb, expect);

        let bb = super::vertical_move_bb(a6, a2);

        assert_eq!(bb, expect);
    }

    #[test]
    fn test_horizontal_bb() {
        let a1 = 0b0000_0000_0000_0001;
        let h1 = 0b0000_0000_1000_0000;
        let expect = 0b000000_0111_1110;

        let bb = super::horizontal_move_bb(a1, h1);

        assert_eq!(bb, expect);

        let bb = super::horizontal_move_bb(h1, a1);

        assert_eq!(bb, expect);
    }

    #[test]
    fn test_diagonal_bb() {
        let a1 = 0b0000_0000_0000_0001;
        let g1 = 0b0000_0000_0100_0000;
        let h8 = 0b1000_0000 << (8 * 7);
        let g7 = g1 << (8 * 6);

        let expect = 0x201008040200;
        let bb = super::diagonal_move_bb(a1, g7);

        assert_eq!(bb, expect);

        let bb = super::diagonal_move_bb(g7, a1);

        assert_eq!(bb, expect);

        let expect = 0b000000001000000001000000001000000001000000001000000001000000000;

        let bb = super::diagonal_move_bb(a1, h8);

        assert_eq!(bb, expect);

        let bb = super::diagonal_move_bb(h8, a1);

        assert_eq!(bb, expect);
    }

    #[test]
    fn test_bit_distance() {
        let e1: u64 = 0b0001_0000;
        let d1: u64 = 0b0000_1000;
        let f1: u64 = 0b0010_0000;
        let e5 = e1 << (8 * 5);
        let d6 = d1 << (8 * 6);
        let f6 = f1 << (8 * 6);

        let distance = super::bit_distance(e5, d6);
        assert_eq!(distance, 7);

        let distance = super::bit_distance(d6, e5);
        assert_eq!(distance, 7);

        let distance = super::bit_distance(e5, f6);
        assert_eq!(distance, 9);

        let distance = super::bit_distance(f6, e5);
        assert_eq!(distance, 9);
    }

    #[test]
    fn test_bit_row() {
        let a1 = 0b0000_0000_0000_0001;
        let b1 = 0b0000_0000_0000_0010;

        let row = super::bit_row(a1);
        assert_eq!(row, 0);
        let row = super::bit_row(b1);
        assert_eq!(row, 0);

        let a2 = a1 << 8;
        let b2 = b1 << 8;
        let row = super::bit_row(a2);
        assert_eq!(row, 1);
        let row = super::bit_row(b2);
        assert_eq!(row, 1);
    }

    #[test]
    fn test_is_moving_horizontally() {
        let a1 = 0b0000_0000_0000_0001;
        let b1 = 0b0000_0000_0000_0010;
        let a2 = a1 << 8;
        let b2 = b1 << 8;

        assert!(super::is_moving_horizontally(a1, b1));
        assert!(super::is_moving_horizontally(b1, a1));
        assert!(super::is_moving_horizontally(a2, b2));
        assert!(super::is_moving_horizontally(b2, a2));

        assert!(!super::is_moving_horizontally(a1, b2));
        assert!(!super::is_moving_horizontally(b2, a1));

        assert!(!super::is_moving_horizontally(a1, a2));
        assert!(!super::is_moving_horizontally(a2, a1));
        assert!(!super::is_moving_horizontally(b1, b2));
        assert!(!super::is_moving_horizontally(b2, b1));
    }

    #[test]
    fn test_is_moving_diagonally() {
        let a1 = 0b0000_0000_0000_0001;
        let b1 = 0b0000_0000_0000_0010;
        let g1 = 0b0100_0000;
        let h1 = 0b1000_0000;

        let a2 = a1 << 8;
        let b2 = b1 << 8;
        let h2 = h1 << 8;

        let h8 = h1 << (8 * 7);

        assert!(super::is_moving_diagonally(a1, b2));
        assert!(super::is_moving_diagonally(b2, a1));

        assert!(!super::is_moving_diagonally(b1, h1));
        assert!(!super::is_moving_diagonally(h1, b1));
        assert!(!super::is_moving_diagonally(a1, h1));
        assert!(!super::is_moving_diagonally(h1, a1));

        assert!(!super::is_moving_diagonally(a1, h2));
        assert!(!super::is_moving_diagonally(h2, a1));

        assert!(super::is_moving_diagonally(h2, g1));
        assert!(super::is_moving_diagonally(g1, h2));

        assert!(super::is_moving_diagonally(a2, b1));
        assert!(super::is_moving_diagonally(b1, a2));

        assert!(super::is_moving_diagonally(a1, h8));
        assert!(super::is_moving_diagonally(h8, a1));
    }

    #[test]
    fn test_l_shaped() {
        let a1 = 0b0000_0001;
        let b1 = 0b0000_0010;
        let c1 = 0b0000_0100;
        let d1 = 0b0000_1000;

        let a3 = a1 << (8 * 2);
        let c3 = c1 << (8 * 2);
        let d2 = d1 << (8 * 1);

        assert!(super::is_l_shaped(b1, c3));
    }

    #[test]
    fn test_l_shaped2() {
        let b1 = 0b0000_0010;
        let c1 = 0b0000_0100;
        let d1 = 0b0000_1000;
        let e1 = 0b0001_0000;
        let f1 = 0b0010_0000;

        let d5 = d1 << (8 * 4);

        let b4 = b1 << (8 * 3);
        let b7 = b1 << (8 * 6);

        let c3 = c1 << (8 * 2);
        let c7 = c1 << (8 * 6);

        let e3 = e1 << (8 * 2);
        let e7 = e1 << (8 * 6);

        let f4 = f1 << (8 * 3);
        let f7 = f1 << (8 * 6);

        let bit_distance = super::bit_distance(d5, b4);
        println!("d5 -> b4: {}", bit_distance);

        let bit_distance = super::bit_distance(d5, b7);
        println!("d5 -> b7: {}", bit_distance);

        let bit_distance = super::bit_distance(d5, c3);
        println!("d5 -> c3: {}", bit_distance);

        let bit_distance = super::bit_distance(d5, c7);
        println!("d5 -> c7: {}", bit_distance);

        let bit_distance = super::bit_distance(d5, e3);
        println!("d5 -> e3: {}", bit_distance);

        let bit_distance = super::bit_distance(d5, e7);
        println!("d5 -> e7: {}", bit_distance);

        let bit_distance = super::bit_distance(d5, f4);
        println!("d5 -> f4: {}", bit_distance);

        let bit_distance = super::bit_distance(d5, f7);
        println!("d5 -> f7: {}", bit_distance);
    }
}
