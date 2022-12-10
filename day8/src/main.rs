use std::{str::FromStr, string::ParseError};
use vector_math::V2;

const INPUT: &str = include_str!("input.txt");  // include in the binary a file on disk containing my input

struct Input {
    forest: Vec<Vec<usize>>
}

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Input{forest: input.lines().map(|s| s.chars().map(|c| c.to_string().parse::<usize>().ok().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>()})
    }
}

impl Input {
    fn get_width(&self) -> usize {
        self.forest.len()
    }

    fn get_height(&self) -> usize {
        self.forest[0].len()
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.get_width() as i32 && y >= 0 && y < self.get_height() as i32
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let dirs = vec!{V2{x: 1, y: 0}, V2{x: -1, y: 0}, V2{x: 0, y: 1} ,V2{x: 0, y: -1}};
        dirs.iter().any(|dir| {
            let h = self.forest[x][y];
            let mut cursor = V2{x: x as i32, y: y as i32} + dir.clone();
            while self.in_bounds(cursor.x, cursor.y) {
                if self.forest[cursor.x as usize][cursor.y as usize] >= h {
                    return false;
                }
                cursor = cursor + dir.clone();
            }
            true
        })
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> usize {
        let dirs = vec!{V2{x: 1, y: 0}, V2{x: -1, y: 0}, V2{x: 0, y: 1} ,V2{x: 0, y: -1}};
        dirs.iter().map(|dir| {
            let h = self.forest[x][y];
            let mut score = 0;
            let mut cursor = V2{x: x as i32, y: y as i32} + dir.clone();
            while self.in_bounds(cursor.x, cursor.y) {
                score += 1;
                if self.forest[cursor.x as usize][cursor.y as usize] >= h {
                    break;
                }
                cursor = cursor + dir.clone();
            }
            score
        }).product()
    }
}

fn main() {
    let input = INPUT.parse::<Input>().expect("The input must parse");
    println!("{}", (0..input.get_width()).map(|x| (0..input.get_height()).map(|y| if input.is_visible(x, y) {1} else {0}).sum::<usize>()).sum::<usize>());
    println!("{}", (0..input.get_width()).flat_map(|x| (0..input.get_height()).map(|y| input.get_scenic_score(x, y)).collect::<Vec<_>>()).max().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_tree_visible() {
        let input = "123\n456\n789".parse::<Input>().unwrap();
        assert!(input.is_visible(0,0));
    }

    #[test]
    fn test_middle_tree_visible() {
        let input = "123\n456\n789".parse::<Input>().unwrap();
        assert!(input.is_visible(1,1));
    }

    #[test]
    fn test_middle_tree_invisible() {
        let input = "111\n111\n111".parse::<Input>().unwrap();
        assert!(!input.is_visible(1,1));
    }

    #[test]
    fn test_edge_tree_scenicness() {
        let input = "111\n111\n111".parse::<Input>().unwrap();
        assert_eq!(input.get_scenic_score(0,0), 0);
    }

    #[test]
    fn test_choked_tree_scenicness() {
        let input = "111\n111\n111".parse::<Input>().unwrap();
        assert_eq!(input.get_scenic_score(1,1), 1);
    }

    #[test]
    fn test_valley_tree_scenicness() {
        let input = "11011\n11011\n11111\n11011\n11011".parse::<Input>().unwrap();
        assert_eq!(input.get_scenic_score(2,2), 4);
    }

    #[test]
    fn test_open_tree_scenicness() {
        let input = "11011\n11011\n00100\n11011\n11011".parse::<Input>().unwrap();
        assert_eq!(input.get_scenic_score(2,2), 16);
    }
}