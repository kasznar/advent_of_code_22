use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

// todo: solve without copy
#[derive(Debug, PartialEq, Copy, Clone)]
 pub enum Symbol {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loose = 0,
}

impl Symbol {
    fn defeat(self, symbol: Symbol) -> Outcome {
        match self {
            Self::Rock if symbol == Self::Scissors => Outcome::Win,
            Self::Paper if symbol == Self::Rock => Outcome::Win,
            Self::Scissors if symbol == Self::Paper => Outcome::Win,
            _ if self == symbol => Outcome::Draw,
            _ => Outcome::Loose,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolParseError;

impl FromStr for Symbol {
    type Err = SymbolParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(SymbolParseError),
        }
    }
}

#[derive(Debug)]
enum MainError {
    IOErr,
    ParseErr,
}

fn main() -> Result<(), MainError> {
    use MainError::IOErr;
    use MainError::ParseErr;

    println!("Day 2");

    let path ="./src/bin/day2_rock_paper_scissors/input.txt";

    let file = File::open(path).map_err(|_| IOErr)?;
    let reader = BufReader::new(file);
    let mut score = 0;

    
    for line in reader.lines() {
        let line_raw = line.map_err(|_| ParseErr)?;
        let (enemy_raw, my_raw ) = line_raw.split_once(" ").ok_or(ParseErr)?;

        let enemy_symbol: Symbol = enemy_raw.parse().map_err(|_| ParseErr)?;
        let my_symbol: Symbol = my_raw.parse().map_err(|_| ParseErr)?;

        score += my_symbol as i32;

        let outcome = my_symbol.defeat(enemy_symbol);
        score += outcome as i32;
    }

    println!("da solution is {:?}", score);

    Ok(())
}