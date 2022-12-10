use std::{str::FromStr, string::ParseError, collections::HashMap};
use vector_math::V2;

const INPUT: &str = include_str!("input.txt");  // include in the binary a file on disk containing my input

struct Input {
    steps: Vec<V2>,
}

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Input{steps: input.lines().flat_map(|l| {
            let tokens = l.split(' ').collect::<Vec<&str>>();
            let mag = tokens[1].parse::<i32>().unwrap();
            match tokens[0] {
                "U" => vec!(V2::UP; mag as usize),
                "D" => vec!(V2::DOWN; mag as usize),
                "L" => vec!(V2::LEFT; mag as usize),
                "R" => vec!(V2::RIGHT; mag as usize),
                _ => panic!("Invalid direction")
            }
        }).collect::<Vec<V2>>()})
    }
}

fn main() {
    let input = INPUT.parse::<Input>().expect("The input must parse");
    
    let mut visits = sim_rope(&input.steps, &1);
    println!("{}", visits.len());

    //PART 2
    visits = sim_rope(&input.steps, &9);
    println!("{}", visits.len());
}

fn follow(head: &V2, tail: &V2) -> (V2, Vec<String>) {
    let mut diff = head.sub(tail);
    let mut my_tail = tail.clone();
    let mut visits = Vec::new();
    
    while diff.x.abs() > 1 || diff.y.abs() > 1 {
        my_tail += diff.squish();
        diff -= diff.squish();
        visits.push(my_tail.to_string());
    }
    (my_tail, visits)
}

fn sim_rope(steps: &Vec<V2>, rope_length: &usize) -> Vec<V2> {
    let mut head = V2::ZERO.clone();
    let mut visits: HashMap<String, usize> = HashMap::new();
    visits.insert(head.to_string(), 1);

    let mut rope = vec!(head.clone(); *rope_length);
    steps.iter().for_each(|step| {
        head = head.add(step);
        (0..rope.len()).for_each(|i| {
            let my_head = if i == 0 {&head} else {&rope.get(i-1).unwrap()};
            let my_tail = &rope.get(i).unwrap();
            let flw = follow(my_head, my_tail);
            rope[i] = flw.0;
            if i == rope.len() - 1 {
                flw.1.iter().for_each(|place| {
                    visits.entry(place.to_string()).and_modify(|x| *x += 1).or_insert(1);
                });
            }
        });
    });

    visits.keys().map(|k| k.parse::<V2>().expect("Couldn't parse key into V2")).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use vector_math::to_string;

    use super::*;

    #[test]
    fn test_input_parse() {
        let input = "R 1\nU 1\nL 1\nD 1\nR 3\nU 3\nL 3\nD 3".parse::<Input>().expect("Input must parse");

        let mut step_iter = input.steps.iter();
        assert_eq!(*step_iter.next().unwrap(), V2::RIGHT);
        assert_eq!(*step_iter.next().unwrap(), V2::UP);
        assert_eq!(*step_iter.next().unwrap(), V2::LEFT);
        assert_eq!(*step_iter.next().unwrap(), V2::DOWN);

        assert_eq!(*step_iter.next().unwrap(), V2::RIGHT);
        assert_eq!(*step_iter.next().unwrap(), V2::RIGHT);
        assert_eq!(*step_iter.next().unwrap(), V2::RIGHT);

        assert_eq!(*step_iter.next().unwrap(), V2::UP);
        assert_eq!(*step_iter.next().unwrap(), V2::UP);
        assert_eq!(*step_iter.next().unwrap(), V2::UP);

        assert_eq!(*step_iter.next().unwrap(), V2::LEFT);
        assert_eq!(*step_iter.next().unwrap(), V2::LEFT);
        assert_eq!(*step_iter.next().unwrap(), V2::LEFT);

        assert_eq!(*step_iter.next().unwrap(), V2::DOWN);
        assert_eq!(*step_iter.next().unwrap(), V2::DOWN);
        assert_eq!(*step_iter.next().unwrap(), V2::DOWN);
    }

    #[test]
    fn test_step() {
        let input = "R 1\nU 1\nL 1\nD 1\nR 3\nU 3\nL 3\nD 3".parse::<Input>().expect("Input must parse");

        let mut head = V2{x:0, y:0};
        let mut step_iter = input.steps.iter();

        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 1, y: 0});

        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 1, y: 1});

        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 0, y: 1});

        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 0, y: 0});

        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 1, y: 0});
        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 2, y: 0});
        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 3, y: 0});

        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 3, y: 1});
        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 3, y: 2});
        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 3, y: 3});

        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 2, y: 3});
        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 1, y: 3});
        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 0, y: 3});

        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 0, y: 2});
        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 0, y: 1});
        head = head.add(step_iter.next().unwrap());
        assert_eq!(head, V2{x: 0, y: 0});
    }

    #[test]
    fn test_follow_no_move() {
        let head = V2::ZERO.clone();
        let tail = head.clone();

        assert_eq!(follow(&head, &tail).0, V2::ZERO);
        assert_eq!(follow(&head, &tail).1.len(), 0);
    }

    #[test]
    fn test_follow_no_move_cardinal() {
        let mut head = V2::ZERO.clone();
        let tail = head.clone();

        head += V2::UP;

        assert_eq!(follow(&head, &tail).0, V2::ZERO);
        assert_eq!(follow(&head, &tail).1.len(), 0);
    }

    #[test]
    fn test_follow_no_move_diagonal() {
        let mut head = V2::ZERO.clone();
        let tail = head.clone();

        head += V2::ONE;

        assert_eq!(follow(&head, &tail).0, V2::ZERO);
        assert_eq!(follow(&head, &tail).1.len(), 0);
    }

    #[test]
    fn test_follow_move_1_cardinal() {
        let mut head = V2::ZERO.clone();
        let tail = head.clone();

        head += V2::UP * 2;

        assert_eq!(follow(&head, &tail).0, V2::UP);
        assert_eq!(follow(&head, &tail).1.len(), 1);
    }

    #[test]
    fn test_follow_move_5_cardinal() {
        let mut head = V2::ZERO.clone();
        let tail = head.clone();

        head += V2::UP * 5;

        assert_eq!(follow(&head, &tail).0, V2::UP * 4);
        assert_eq!(follow(&head, &tail).1.len(), 4);
    }

    #[test]
    fn test_follow_move_1_diagonnal() {
        let mut head = V2::ZERO.clone();
        let tail = head.clone();

        head += V2::ONE * 2;

        assert_eq!(follow(&head, &tail).0, V2::ONE);
        assert_eq!(follow(&head, &tail).1.len(), 1);
    }

    #[test]
    fn test_follow_move_5_diagonal() {
        let mut head = V2::ZERO.clone();
        let tail = head.clone();

        head += V2::ONE * 5;

        assert_eq!(follow(&head, &tail).0, V2::ONE * 4);
        assert_eq!(follow(&head, &tail).1.len(), 4);
    }

    #[test]
    fn test_follow_move_semidiagonal() {
        let mut head = V2::ZERO.clone();
        let tail = head.clone();

        head += V2::UP * 2 + V2::RIGHT;

        assert_eq!(follow(&head, &tail).0, V2::ONE);
        assert_eq!(follow(&head, &tail).1.len(), 1);
    }

    #[test]
    fn test_follow_move_far_semidiagonal() {
        let mut head = V2::ZERO.clone();
        let tail = head.clone();

        head += V2::UP * 2 + V2::LEFT * 5;

        assert_eq!(follow(&head, &tail).0, V2::LEFT * 4 + V2::UP * 2);
        assert_eq!(follow(&head, &tail).1.len(), 4);
    }

    #[test]
    fn test_long_rope() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20".parse::<Input>().expect("The input must parse");
        let visits = sim_rope(&input.steps, &9);

        println!("{}", to_string(&visits));

        assert_eq!(visits.len(), 36);
    }

    #[test]
    fn test_long_rope_step_2() {
        let input = "R 5\nU 8".parse::<Input>().expect("The input must parse");
        let visits = sim_rope(&input.steps, &9);

        println!("{}", to_string(&visits));

        assert_eq!(visits.len(), 1);
    }

    #[test]
    fn test_long_rope_step_3() {
        let input = "R 5\nU 8\nL 8".parse::<Input>().expect("The input must parse");
        let visits = sim_rope(&input.steps, &9);

        println!("{}", to_string(&visits));

        assert_eq!(visits.len(), 4);
    }
}