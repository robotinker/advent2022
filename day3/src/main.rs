use std::{str::FromStr, string::ParseError};
use std::str;

const INPUT: &str = include_str!("input.txt");  // include in the binary a file on disk containing my input

fn get_priority(ch: &char) -> u32 {
    if (97 .. 97 + 26).contains(&(*ch as i32)) {
        *ch as u32 - 96
    }
    else if (65 .. 65 + 26).contains(&(*ch as i32)) {
        *ch as u32 - 64 + 26
    }
    else {
        0
    }
}

#[derive(Clone)]
struct Sack {
    a: Vec<char>,
    b: Vec<char>
}

impl Sack {
    fn find_shared(&self) -> Option<&char> {
        let mut matches = self.a.iter()
            .filter(|item| self.b.contains(item));
        matches.next()
    }

    fn get_priority(&self) -> u32 {
        get_priority(self.find_shared().unwrap_or_else(|| panic!("Tried to get priority value for sack that has no shared items between its compartments")))
    }

    fn contains(&self, c: &char) -> bool {
        self.a.contains(c) || self.b.contains(c)
    }

    fn item_types(&self) -> Vec<&char> {
        let mut output = Vec::new();
        for item in self.a.iter() {
            if !output.contains(&item) {
                output.push(item);
            }
        }
        for item in self.b.iter() {
            if !output.contains(&item) {
                output.push(item);
            }
        }
        output
    }
}

impl FromStr for Sack {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() % 2 > 0 {
            panic!("Tried to create a sack with an odd number of items");
        }

        let half_len = input.len() / 2;
        Ok(Sack{a: input[..half_len].chars().collect(), b: input[half_len..].chars().collect()})
    }
}

struct ElfGroup {
    members: Vec<Sack>
}

impl ElfGroup {
    fn find_badge(&self) -> char {
        let item_types = self.members[0].item_types();
        let common_item: Vec<bool> = item_types.iter()
            .map(|item| self.members[1..].iter()
                .map(|sack| sack.contains(item)).all(|x| x)
            ).collect();
        for i in 0..item_types.len() {
            if common_item[i] {
                return *item_types[i];
            }
        }
        '!'
    }
}

struct Input {
    sacks: Vec<Sack>
}

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(input_string: &str) -> Result<Self, Self::Err> {
        Ok(Input {sacks: input_string.lines().map(
            |s| s.parse::<Sack>().expect("Couldn't parse line as sack")).collect()})
    }
}

fn main() {
    let input = INPUT.parse::<Input>().expect("The input must parse");
    println!("{}", input.sacks.iter().map(|sack| sack.get_priority()).sum::<u32>());
    println!("{}", input.sacks
        .chunks(3)
        .map(|chunk| get_priority(&ElfGroup{members: chunk.to_vec()}.find_badge())).sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(get_priority(&'a'), 1);
        assert_eq!(get_priority(&'z'), 26);
        assert_eq!(get_priority(&'A'), 27);
        assert_eq!(get_priority(&'Z'), 52);
    }

    #[test]
    fn test_sack() {
        let sack = "abac".parse::<Sack>().expect("Couldn't parse sack");
        assert_eq!(sack.find_shared().unwrap_or(&'!'), &'a');
    }

    #[test]
    fn test_fixed_sack() {
        let sack = "aabc".parse::<Sack>().expect("Couldn't parse sack");
        assert_eq!(sack.find_shared().unwrap_or(&'!'), &'!');
    }

    #[test]
    fn test_find_badge() {
        let group = ElfGroup{members: vec!["aabb".parse::<Sack>().unwrap(), "ccbb".parse::<Sack>().unwrap(), "ddbb".parse::<Sack>().unwrap()]};
        assert_eq!(group.find_badge(), 'b');
    }

    #[test]
    fn test_sack_split()
    {
        let sack = "abac".parse::<Sack>().expect("Couldn't parse sack");
        assert_eq!(sack.a.len(), 2);
        assert_eq!(sack.b.len(), 2);
    }
}