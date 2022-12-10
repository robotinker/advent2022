use std::{str::FromStr, string::ParseError};

const INPUT: &str = include_str!("input.txt");  // include in the binary a file on disk containing my input

struct Input {
    values: Vec<i32>
}

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut x = 1;
        let mut output = vec!(1);
        output.append(&mut input.lines().flat_map(|s| 
            if s.starts_with("addx") {
                let old_x = x;
                x += s.split(' ').collect::<Vec<_>>()[1].parse::<i32>().expect("Failed to parse int");
                vec!(old_x,  x)
            } 
            else {
                vec!(x)
            }).collect::<Vec<_>>());
        Ok(Input{values: output})
    }
}

fn main() {
    let input = INPUT.parse::<Input>().expect("The input must parse");

    println!("{}", (19..=219).step_by(40).collect::<Vec<_>>().iter().map(|c| (c+1) * input.values.get(*c as usize).unwrap()).sum::<i32>());

    let w = 40;
    let pxls = (0..input.values.len()).map(|c| if (input.values.get(c).unwrap() - ((c % w) as i32)).abs() <= 1 {'#'} else {'.'}).collect::<Vec<_>>();
    pxls.chunks(40).for_each(|chunk| println!("{}", chunk.iter().collect::<String>()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_adds() {
        let input = "addx 1\naddx 3\naddx 5".parse::<Input>().expect("The input must parse");
        let mut input_iter = input.values.iter();

        assert_eq!(input_iter.next().unwrap(), &1);
        assert_eq!(input_iter.next().unwrap(), &1);
        assert_eq!(input_iter.next().unwrap(), &2);
        assert_eq!(input_iter.next().unwrap(), &2);
        assert_eq!(input_iter.next().unwrap(), &5);
        assert_eq!(input_iter.next().unwrap(), &5);
        assert_eq!(input_iter.next().unwrap(), &10);

    }

    #[test]
    fn test_signal_noops() {
        let input = "noop\nnoop\nnoop".parse::<Input>().expect("The input must parse");
        let mut input_iter = input.values.iter();

        assert_eq!(input_iter.next().unwrap(), &1);
        assert_eq!(input_iter.next().unwrap(), &1);
        assert_eq!(input_iter.next().unwrap(), &1);
        assert_eq!(input_iter.next().unwrap(), &1);
    }

    #[test]
    fn test_signal_short_example() {
        let input = "noop\naddx 3\naddx -5".parse::<Input>().expect("The input must parse");
        let mut input_iter = input.values.iter();

        assert_eq!(input_iter.next().unwrap(), &1);
        assert_eq!(input_iter.next().unwrap(), &1);
        assert_eq!(input_iter.next().unwrap(), &1);
        assert_eq!(input_iter.next().unwrap(), &4);
        assert_eq!(input_iter.next().unwrap(), &4);
        assert_eq!(input_iter.next().unwrap(), &-1);
    }

    #[test]
    fn test_signal_long_example() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".parse::<Input>().expect("The input must parse");

        assert_eq!(input.values[19], 21);
        assert_eq!(input.values[59], 19);
        assert_eq!(input.values[99], 18);
        assert_eq!(input.values[139], 21);
        assert_eq!(input.values[179], 16);
        assert_eq!(input.values[219], 18);

        assert_eq!(input.values[19] * 20, 420);
        assert_eq!(input.values[59] * 60, 1140);
        assert_eq!(input.values[99] * 100, 1800);
        assert_eq!(input.values[139] * 140, 2940);
        assert_eq!(input.values[179] * 180, 2880);
        assert_eq!(input.values[219] * 220, 3960);

        assert_eq!((19..=219).step_by(40).collect::<Vec<_>>().iter().map(|c| (c+1) * input.values.get(*c as usize).unwrap()).sum::<i32>(), 13140);
    }
}