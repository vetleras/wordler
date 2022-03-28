use std::cmp;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Letter {
    Green(char),
    Yellow(char),
    Grey(char),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Guess(Vec<Letter>);

type Word = Vec<char>;

impl Guess {
    pub fn new(wordle: &Word, guess: &Word) -> Guess {
        let mut letters: Vec<Letter> = Vec::new();
        // Green pass
        for (i, &c) in guess.iter().enumerate() {
            letters.push({
                if c == wordle[i] {
                    Letter::Green(c)
                } else {
                    Letter::Grey(c)
                }
            });
        }
        // Yellow pass
        for i in 0..letters.len() {
            if let Letter::Grey(c) = letters[i] {
                let wordle_occurances = wordle.iter().filter(|&&x| c == x).count();
                let guess_occurances = guess.iter().filter(|&&x| c == x).count();
                let letter_occurances = letters
                    .iter()
                    .filter(|&&x| x == Letter::Green(c) || x == Letter::Yellow(c))
                    .count();
                if letter_occurances < cmp::min(guess_occurances, wordle_occurances) {
                    letters[i] = Letter::Yellow(c);
                }
            }
        }
        Guess(letters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let wordle = "abaacc".chars().collect();
        let guess = "aaabad".chars().collect();
        let v: Vec<Letter> = vec![
            Letter::Green('a'),
            Letter::Yellow('a'),
            Letter::Green('a'),
            Letter::Yellow('b'),
            Letter::Grey('a'),
            Letter::Grey('d'),
        ];
        assert_eq!(Guess::new(&wordle, &guess), Guess(v));
    }
}
