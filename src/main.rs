use wordler::{Word, guesser::Guesser};
use std::env;
use wordler::guesser;

fn parse_words(string: &str) -> Vec<Word> {
    string.lines().map(|l| Word::from(l)).collect()
}

fn main() {
    let words = parse_words(include_str!("../allowed_words.txt"));
    //let wordles = parse_words(include_str!("../possible_words.txt"));

    let wordle = Word::from(env::args().nth(1).unwrap());
    let scorer = guesser::Scorer::default();

    println!("Guessing {}", wordle);
    for guess in scorer.play(&words, &wordle) {
        println!("{}", guess);
    }
}
