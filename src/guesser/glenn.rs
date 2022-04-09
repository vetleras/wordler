use crate::guess::Guess;
use crate::{guess::Tile, Word};
use std::collections::HashSet;

use super::Guesser;

enum Constraint {
    Include(u8),
    Exclude(HashSet<u8>),
}

pub struct GlennFilter {
    constraints: Vec<Constraint>,
}

impl GlennFilter {
    pub fn new(guesses: &[Guess]) -> Self {
        assert!(!guesses.is_empty());

        let mut constraints: Vec<Constraint> = guesses
            .first()
            .unwrap()
            .iter()
            .map(|_| Constraint::Exclude(HashSet::new()))
            .collect();

        for guess in guesses {
            for (i, (c, t)) in guess.iter().enumerate() {
                if let Constraint::Exclude(ref mut excluded) = constraints[i] {
                    if t == &Tile::Green {
                        constraints[i] = Constraint::Include(*c);
                    } else {
                        excluded.insert(*c);
                    }
                }
            }
        }
        GlennFilter { constraints }
    }

    pub fn filter(&self, word: &Word) -> bool {
        word.iter()
            .zip(self.constraints.iter())
            .all(|(character, constraint)| match constraint {
                Constraint::Include(included) => included == character,
                Constraint::Exclude(excluded) => !excluded.contains(&character),
            })
    }
}

#[derive(Default)]
pub struct Glenn {}

impl Guesser for Glenn {
    fn guess<'a>(&self, words: &'a Vec<Word>, guesses: &Vec<Guess>) -> &'a Word {
        if guesses.is_empty() {
            return words.first().unwrap();
        }
        let filter = GlennFilter::new(guesses);
        words.iter().filter(|w| filter.filter(w)).next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{guesser::Guesser, Word};

    use super::Glenn;

    #[test]
    fn it_works() {
        let words = vec![
            Word::from("aaaaa"),
            Word::from("aaaab"),
            Word::from("aaaac"),
            Word::from("baaac"),
        ];

        let guesser = Glenn::default();

        assert_eq!(guesser.play(&words, &words[0]).len(), 1);
        assert_eq!(guesser.play(&words, &words[1]).len(), 2);
        assert_eq!(guesser.play(&words, &words[2]).len(), 3);
        assert_eq!(guesser.play(&words, &words[3]).len(), 2);
    }
}
