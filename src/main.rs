use std::collections::HashMap;
use wordler::filter::Filter;
use wordler::guess::Guess;
use wordler::Word;

fn parse_words(string: &str) -> Vec<Word> {
    string.lines().map(Word::from).collect()
}

fn main() {
    let words = parse_words(include_str!("../allowed_words.txt"));
    let wordles = parse_words(include_str!("../possible_words.txt"));

    for wordle in &wordles {
        println!("-- Guessing {} --", wordle);
        let mut guesses = Vec::new();
        loop {
            let filter = Filter::new(&guesses[..]);
            let potential_wordles: Vec<_> = wordles.iter().filter(|w| filter.filter(w)).collect();
            let guess_word = if potential_wordles.len() == 1 {
                potential_wordles.first().unwrap()
            } else {
                guess(&guesses[..], &potential_wordles[..], &words[..])
            };
            let guess = Guess::new(wordle, guess_word);
            println!("{}", &guess);
            if guess.is_correct() {
                break;
            }
            guesses.push(guess);
        }
    }
}

fn guess<'a>(
    guesses: &[Guess],
    potential_wordles: &[&Word],
    allowed_words: &'a [Word],
) -> &'a Word {
    if guesses.is_empty() {
        allowed_words
            .iter()
            .find(|w| w == &&Word::from("soare"))
            .unwrap()
    } else {
        allowed_words
            .iter()
            .min_by_key(|guess_word| {
                let s = average_words_left(guess_word, guesses, potential_wordles);
                //println!("{} {}", guess_word, s);
                s
            })
            .unwrap()
    }
}

// Sums the number of possible words after applying the guess word and guesses to each possible wordle
fn average_words_left(guess_word: &Word, guesses: &[Guess], potential_wordles: &[&Word]) -> usize {
    let mut cached_words_left = HashMap::new();
    potential_wordles
        .iter()
        .map(|wordle| {
            let mut guesses = guesses.to_vec();
            let guess = Guess::new(wordle, guess_word);
            guesses.push(guess.clone());
            *cached_words_left.entry(guess).or_insert_with(|| {
                let filter = Filter::new(&guesses[..]);
                potential_wordles
                    .iter()
                    .filter(|w| filter.filter(w))
                    .count()
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use wordler::{guess::Guess, Word};

    use super::average_words_left;

    #[test]
    fn it_works() {
        let words = vec![
            Word::from("aaaaa"), //1 3 3 2 2
            Word::from("aabbb"), //2 1 3 1 1
            Word::from("aaccc"), //2 3 1 1 1
            Word::from("aaabb"), //2 1 3 1 2
            Word::from("aaacc"), //2 3 1 2 1
        ];

        let words: Vec<_> = words.iter().collect();

        let guesses: Vec<Guess> = vec![];

        assert_eq!(average_words_left(words[0], &guesses[..], &words[..]), 9);
        assert_eq!(average_words_left(words[1], &guesses[..], &words[..]), 11);
        assert_eq!(average_words_left(words[2], &guesses[..], &words[..]), 11);
        assert_eq!(average_words_left(words[3], &guesses[..], &words[..]), 7);
        assert_eq!(average_words_left(words[4], &guesses[..], &words[..]), 7);
    }
}
