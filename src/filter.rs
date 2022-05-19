use crate::guess::Guess;

use crate::{guess::Tile, Word};
use std::collections::{HashMap, HashSet};

use std::cmp;

#[derive(Clone)]
enum TileConstraint {
    Include(u8),
    Exclude(HashSet<u8>),
}

#[derive(Debug, Default)]
struct WordConstraint {
    count: usize,
    capped: bool,
}
pub struct Filter {
    tile_constraints: Vec<TileConstraint>,
    word_constraints: HashMap<u8, WordConstraint>,
}

impl Filter {
    pub fn new(guesses: &[Guess]) -> Filter {
        let mut tile_constraints = match guesses.first() {
            Some(guess) => vec![TileConstraint::Exclude(HashSet::new()); guess.len()],
            None => Vec::new(),
        };
        let mut word_constraints: HashMap<u8, WordConstraint> = HashMap::new();

        for guess in guesses {
            let mut non_gray_count = HashMap::new();
            for ((char, tile), constraint) in guess.iter().zip(tile_constraints.iter_mut()) {
                match tile {
                    Tile::Green => {
                        *constraint = TileConstraint::Include(*char);
                        *non_gray_count.entry(*char).or_default() += 1;
                    }
                    Tile::Yellow => {
                        if let TileConstraint::Exclude(ref mut excluded) = constraint {
                            excluded.insert(*char);
                        }
                        *non_gray_count.entry(*char).or_default() += 1;
                    }
                    Tile::Gray => {
                        if let TileConstraint::Exclude(ref mut excluded) = constraint {
                            excluded.insert(*char);
                        }
                    }
                }
            }

            for (c, count) in non_gray_count {
                let constraint = word_constraints.entry(c).or_default();
                constraint.count = cmp::max(constraint.count, count);
            }
        }

        Filter {
            tile_constraints,
            word_constraints,
        }
    }

    pub fn filter(&self ,word: &Word) -> bool {
        if word.iter().zip(self.tile_constraints.iter()).any(
            |(character, constraint)| match constraint {
                TileConstraint::Include(included) => included != character,
                TileConstraint::Exclude(excluded) => excluded.contains(character),
            },
        ) {
            return false;
        }

        let mut counts = HashMap::new();
        for c in word.iter() {
            *counts.entry(c).or_insert(0) += 1;
        }

        !self.word_constraints.iter().any(|(c, constraint)| {
            match constraint.count.cmp(counts.get(c).unwrap_or(&0)) {
                cmp::Ordering::Less => constraint.capped,
                cmp::Ordering::Equal => false,
                cmp::Ordering::Greater => true,
            }
        })

    }
}

#[cfg(test)]
mod tests {
    use crate::{guess::Guess, Word};

    use super::Filter;

    #[test]
    fn no_filter() {
        let guesses = Vec::new();
        let filter = Filter::new(&guesses);
        assert!(filter.filter(&Word::from("hello")));
    }

    #[test]
    fn multiple_equal_letters() {
        let guess = Guess::new(&Word::from("aback"), &Word::from("aabaa"));
        let guesses = vec![guess];
        let filter = Filter::new(&guesses[..]);
        assert!(!filter.filter(&Word::from("aabaa")));
        assert!(!filter.filter(&Word::from("abaae")));
        assert!(filter.filter(&Word::from("aeabe")));
    }
}
