use std::{str::FromStr, string::ParseError};

const INPUT: &str = include_str!("input.txt");  // include in the binary a file on disk containing my input

struct Input {
    pairs: Vec<Vec<WorkZone>>
}

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Input{pairs: input.lines().map(|s| s.split(',').map(|p| p.parse::<WorkZone>().unwrap()).collect::<Vec<WorkZone>>()).collect()})
    }
}

struct WorkZone {
    min: u32,
    max: u32
}

impl WorkZone {
    fn contains(&self, other: &WorkZone ) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &WorkZone) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

impl FromStr for WorkZone {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split_input = input.split('-');
        Ok(WorkZone{min: split_input.next().unwrap().parse::<u32>().unwrap(), max: split_input.next().unwrap().parse::<u32>().unwrap()})
    }
}

fn main() {
    let input = INPUT.parse::<Input>().expect("The input must parse");
    println!("{}", input.pairs.iter().map(|pair| {
        if  pair[0].contains(&pair[1]) || pair[1].contains(&pair[0]) {
            Some(pair)
        }
        else {
            None
        }
    }).flatten().count());
    println!("{}", input.pairs.iter().map(|pair| {
        if pair[0].overlaps(&pair[1]) {
            Some(pair)
        }
        else {
            None
        }
    }).flatten().count());
}
