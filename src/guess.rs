use std::collections::HashMap;
use std::fmt;

use termion::color;

use super::Word;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Green,
    Yellow,
    Gray,
}

#[derive(Debug, derive_more::Deref)]
pub struct Guess(Vec<(u8, Tile)>);

impl Guess {
    pub fn new(wordle: &Word, guess: &Word) -> Guess {
        let mut residuals = HashMap::new();
        for (guess_c, wordle_c) in guess.iter().zip(wordle.iter()) {
            if guess_c != wordle_c {
                *residuals.entry(wordle_c).or_insert(0) += 1;
            }
        }

        Guess(
            guess
                .iter()
                .zip(wordle.iter())
                .map(|(guess_c, wordle_c)| {
                    if guess_c == wordle_c {
                        (*guess_c, Tile::Green)
                    } else if residuals.get(guess_c).unwrap_or(&0) > &0 {
                        residuals.entry(guess_c).and_modify(|e| *e -= 1);
                        (*guess_c, Tile::Yellow)
                    } else {
                        (*guess_c, Tile::Gray)
                    }
                })
                .collect(),
        )
    }

    pub fn is_correct(&self) -> bool {
        self.iter().all(|(_, t)| t == &Tile::Green)
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let green = format!(
            "{}{}",
            color::Bg(color::LightGreen),
            color::Fg(color::Black)
        );
        let yellow = format!(
            "{}{}",
            color::Bg(color::LightYellow),
            color::Fg(color::Black)
        );
        let reset = format!("{}{}", color::Bg(color::Reset), color::Fg(color::Reset));

        for (c, t) in self.iter() {
            match t {
                Tile::Green => write!(f, "{}{}{}", green, *c as char, reset).unwrap(),
                Tile::Yellow => write!(f, "{}{}{}", yellow, *c as char, reset).unwrap(),
                Tile::Gray => write!(f, "{}", *c as char).unwrap(),
            };
        }
        Ok(())
    }
}
