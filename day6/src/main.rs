use std::{str::FromStr, string::ParseError};

const INPUT: &str = include_str!("input.txt");  // include in the binary a file on disk containing my input

struct Input {
    letters: Vec<char>
}

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Input{letters: input.chars().collect::<Vec<char>>()})
    }
}

fn main() {
    let input = INPUT.parse::<Input>().expect("The input must parse");
    
    println!("{}", find_unique_sequence(&input.letters, 4));
    println!("{}", find_unique_sequence(&input.letters, 14));
}

fn find_unique_sequence(letters: &Vec<char>, n: usize) -> usize {
    let mut i = 0;
    let mut quad = letters[0..n].to_vec();
    while quad.iter().any(|x| quad.iter().filter(|n| *n == x).count() > 1) {
        i += 1;
        quad = letters[i..i + n].to_vec();
    }
    i + n
}
