use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn to_bit_board(&self) -> u64 {
        1 << (self.x + self.y * 8)
    }

    pub fn from_bit_board(bit_board: u64) -> Position {
        let mut bit_board = bit_board;
        let mut x = 0;
        let mut y = 0;
        while bit_board & 1 == 0 {
            bit_board >>= 1;
            x += 1;
            if x == 8 {
                x = 0;
                y += 1;
            }
        }
        Position { x, y }
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut value: String = Deserialize::deserialize(deserializer)?;
        value.make_ascii_lowercase();
        let position = Position::from_str(&value);
        match position {
            Ok(position) => Ok(position),
            Err(_) => Err(serde::de::Error::custom("Invalid position")),
        }
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}", self).serialize(serializer)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self.x {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Invalid x position"),
        };
        let y = match self.y {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => panic!("Invalid y position"),
        };
        write!(f, "{}{}", x, y)
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(());
        }
        let x = match chars[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err(()),
        };
        let y = match chars[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err(()),
        };
        Ok(Position { x, y })
    }
}
