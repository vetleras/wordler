pub mod glenn;
pub mod sigrid;

use std::collections::HashMap;

use crate::guesser;
use crate::{guess::Guess, Word};

pub trait Guesser {
    fn guess<'a>(&self, words: &'a Vec<Word>, guesses: &Vec<Guess>) -> &'a Word;

    fn play(&self, words: &Vec<Word>, wordle: &Word) -> Vec<Guess> {
        let mut guesses = Vec::new();
        let guess = Guess::new(wordle, &Word::from("crate"));
        guesses.push(guess);
        while {
            let guess = Guess::new(wordle, self.guess(&words, &guesses));
            guesses.push(guess);
            !guesses.last().unwrap().is_correct()
        } {}
        guesses
    }
}

#[derive(Debug, Default)]
pub struct Scorer {}

impl Guesser for Scorer {
    fn guess<'a>(&self, words: &'a Vec<Word>, guesses: &Vec<Guess>) -> &'a Word {
        let mut count: Vec<HashMap<u8, usize>> = words
            .first()
            .unwrap()
            .iter()
            .map(|_| HashMap::new())
            .collect();

        if guesses.is_empty() {
            for w in words {
                for (i, c) in w.iter().enumerate() {
                    *count[i].entry(*c).or_default() += 1;
                }
            }
            words
                .iter()
                .max_by_key::<usize, _>(|w| {
                    w.iter()
                        .enumerate()
                        .map(|(i, c)| count[i].get(c).cloned().unwrap_or_default())
                        .sum()
                })
                .unwrap()
        } else {
            let glenn = guesser::glenn::GlennFilter::new(guesses);
            let sigrid = guesser::sigrid::SigridFilter::new(guesses);

            for w in words
                .iter()
                .filter(|w| glenn.filter(w))
                .filter(|w| sigrid.filter(w))
            {
                for (i, c) in w.iter().enumerate() {
                    *count[i].entry(*c).or_default() += 1;
                }
            }

            words
                .iter()
                .filter(|w| glenn.filter(w))
                .filter(|w| sigrid.filter(w))
                .max_by_key::<usize, _>(|w| {
                    w.iter()
                        .enumerate()
                        .map(|(i, c)| count[i].get(c).cloned().unwrap_or_default())
                        .sum()
                })
                .unwrap()
        }
    }
}
