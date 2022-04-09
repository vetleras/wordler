use super::glenn;
use crate::guess::Guess;
use crate::{guess::Tile, Word};
use std::cmp;
use std::collections::HashMap;

use super::Guesser;

#[derive(Debug, Default)]
struct Constraint {
    count: usize,
    capped: bool,
}

pub struct SigridFilter {
    constraints: HashMap<u8, Constraint>,
}

impl SigridFilter {
    pub fn new(guesses: &[Guess]) -> Self {
        assert!(!guesses.is_empty());

        let mut constraints: HashMap<u8, Constraint> = HashMap::new();
        for guess in guesses {
            let mut non_gray_count = HashMap::new();
            for (c, t) in guess.iter().cloned() {
                if t == Tile::Gray {
                    constraints.entry(c).or_default().capped = true;
                } else {
                    *non_gray_count.entry(c).or_default() += 1;
                }
            }

            for (c, count) in non_gray_count {
                let constraint = constraints.entry(c).or_default();
                constraint.count = cmp::max(constraint.count, count);
            }
        }
        SigridFilter { constraints }
    }

    pub fn filter(&self, word: &Word) -> bool {
        let mut counts = HashMap::new();
        for c in word.iter() {
            *counts.entry(c).or_insert(0) += 1;
        }

        self.constraints.iter().all(|(c, constraint)| {
            match constraint.count.cmp(counts.get(c).unwrap_or(&0)) {
                cmp::Ordering::Less => !constraint.capped,
                cmp::Ordering::Equal => true,
                cmp::Ordering::Greater => false,
            }
        })
    }
}

#[derive(Default)]
pub struct Sigrid {}

impl Guesser for Sigrid {
    fn guess<'a>(&self, words: &'a Vec<Word>, guesses: &Vec<Guess>) -> &'a Word {
        if guesses.is_empty() {
            return words.first().unwrap();
        }
        let glenn = glenn::GlennFilter::new(guesses);
        let sigrid = SigridFilter::new(guesses);
        words
            .iter()
            .filter(|w| glenn.filter(w))
            .filter(|w| sigrid.filter(w))
            .next()
            .unwrap()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{guesser::Guesser, Word};

//     use super::Basic;

//     #[test]
//     fn it_works() {
//         let words = vec![
//             Word::from("aaaaa"),
//             Word::from("aaaab"),
//             Word::from("aaaac"),
//             Word::from("baaac"),
//         ];

//         let guesser = Basic::default();

//         assert_eq!(guesser.play(&words, &words[0]).len(), 1);
//         assert_eq!(guesser.play(&words, &words[1]).len(), 2);
//         assert_eq!(guesser.play(&words, &words[2]).len(), 3);
//         assert_eq!(guesser.play(&words, &words[3]).len(), 2);
//     }
// }
