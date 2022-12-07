use std::fs;

fn main() {
    let input = fs::read_to_string("day5/src/input.txt").expect("Input file could not be read!");

    solve_problem(&input, perform_ops_1);
    solve_problem(&input, perform_ops_2);
}

fn parse_crates(mut lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let first_line = lines.pop().unwrap();
    first_line.split("   ").for_each(|_| stacks.push(Vec::new()));

    lines.reverse();

    lines.iter().for_each(|line| {
        let mut row_iter = (1..line.len()).step_by(4);
        (0..row_iter.size_hint().1.unwrap()).for_each(|i| {
            let c = line.chars().nth(row_iter.next().unwrap()).unwrap();
            match c {
                ' ' => (),
                _ => stacks[i].push(c)
            }
        })
    });
    stacks
}

fn perform_ops_1(my_stacks: Vec<Vec<char>>, lines: Vec<&&str>) -> Vec<Vec<char>> {
    let mut stacks = my_stacks;
    lines.iter().for_each(|line| {
        let op = parse_op(line);
        (0..op[0]).for_each(|_| {
            let c = stacks[op[1]-1].pop().unwrap();
            stacks[op[2]-1].push(c);
        });
    });
    stacks
}

fn perform_ops_2(my_stacks: Vec<Vec<char>>, lines: Vec<&&str>) -> Vec<Vec<char>> {
    let mut stacks = my_stacks;
    lines.iter().for_each(|line| {
        let op = parse_op(line);
        let pull_index = stacks[op[1]-1].len();
        (0..op[0]).for_each(|_| {
            let c = stacks[op[1]-1].remove(pull_index - op[0]);
            stacks[op[2]-1].push(c);
        });
    });
    stacks
}

fn parse_op(line: &str) -> Vec<usize> {
    line.split(' ').filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<usize>>()
}

fn solve_problem(input: &str, transform: fn(Vec<Vec<char>>, Vec<&&str>) -> Vec<Vec<char>>) {

    let divider = input.lines().position(|s| s.is_empty()).unwrap();

    let stacks: Vec<Vec<char>> = parse_crates(input.lines().take(divider).collect::<Vec<&str>>());
    let moved_stacks = transform(stacks, input.lines().collect::<Vec<&str>>()[divider + 1..].iter().collect::<Vec<&&str>>());

    println!("{}", moved_stacks.iter().map(|stack| stack.last().unwrap_or(&' ')).into_iter().collect::<String>());
}