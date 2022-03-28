#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Letter {
    Green(char),
    Yellow(char),
    Grey(char),
}

#[derive(Debug, Clone)]
struct Guess(Vec<Letter>);
type Word = Vec<char>;

impl Guess {
    pub fn new(guess: &Word, wordle: &Word) -> Guess {
        let mut letters: Vec<Letter> = Vec::new();
        for (i, &c) in guess.iter().enumerate() {
            letters.push({
                if c == wordle[i] {
                    Letter::Green(c)
                } else if wordle.contains(&c)
                    && letters
                        .iter()
                        .filter(|&&l| l == Letter::Green(c) || l == Letter::Yellow(c))
                        .count()
                        >= wordle.iter().filter(|&&l| l == c).count()
                {
                    Letter::Yellow(c)
                } else {
                    Letter::Grey(c)
                }
            });
        }
        Guess(letters)
    }
}

fn main() {
    let g = Guess::new(&"hello".chars().collect(), &"hello".chars().collect());
    println!("{:?}", g)
}
/*
    wordle must contain c
    number of yellows in guess must not exceed number of this letter in wordle
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

