use std::fs;

fn main() {
    let input = fs::read_to_string("day2/src/input.txt").expect("Input file could not be read!");

    let mut total = 0;
    for round in input.lines() {
        let moves: Vec<&str> = round.split(' ').collect();
        match moves[1] {
            "X" => total += 1,
            "Y" => total += 2,
            "Z" => total += 3,
            _ => println!("Invalid response move")
        } 

        match moves[0] {
            "A" => {
                if moves[1] == "X" {
                    total += 3;
                }
                else if moves[1] == "Y" {
                    total += 6;
                }
            },
            "B" => {
                if moves[1] == "Y" {
                    total += 3;
                }
                else if moves[1] == "Z" {
                    total += 6;
                }
            }
            "C" => {
                if moves[1] == "Z" {
                    total += 3;
                }
                else if moves[1] == "X" {
                    total += 6;
                }
            }
            _ => println!("Invalid opponent move!")
        } 
    }
    println!("{}", total);

    total = 0;
    for round in input.lines() {
        let moves: Vec<&str> = round.split(' ').collect();
        match moves[1] {
            "X" => total += match moves[0] {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0
            },
            "Y" => total += 3 + match moves[0] {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0
            },
            "Z" => total += 6 + match moves[0] {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0
            },
            _ => println!("Invalid response move")
        }
    }
    println!("{}", total);
}
