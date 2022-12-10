use std::{str::FromStr, string::ParseError, collections::HashMap};

const INPUT: &str = include_str!("input.txt");  // include in the binary a file on disk containing my input

struct Input {
    dir_hash: HashMap<String, ElfDirectory>
}

struct ElfDirectory {
    path: String,
    files: Vec<ElfFile>,
    dirs: Vec<String>,
    parent: Option<String>
}

impl ElfDirectory {
    fn add_file(&mut self, my_name: &str, my_size: &i32) {
        self.files.push(ElfFile{name: my_name.to_string(), size: *my_size})
    }

    fn get_size(&self, dir_hash: &HashMap<String, ElfDirectory>) -> i32 {
        self.files.iter().map(|f| f.size).sum::<i32>() + self.dirs.iter().map(|d| dir_hash.get(d).unwrap().get_size(dir_hash)).sum::<i32>()
    }
}

struct ElfFile {
    name: String,
    size: i32
}

impl FromStr for Input {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut dirs = HashMap::new();
        dirs.insert("/".to_string(), ElfDirectory{path: "/".to_string(), files: Vec::new(), dirs: Vec::new(), parent: None});
        let mut curr_head = "/".to_string();
        input.lines().for_each(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            match tokens[0] {
                "$" => {
                    if let "cd" = tokens[1] {
                        match tokens[2] {
                            ".." => {
                                curr_head = dirs[&curr_head].parent.as_ref().unwrap().to_string();
                            },
                            "/" => curr_head = "/".to_string(),
                            _ => {
                                curr_head.push('/');
                                curr_head.push_str(&tokens[2].to_string());
                            }
                        }
                    }
                },
                "dir" => {
                    let pth = vec!(curr_head.clone(), "/".to_string(), tokens[1].to_string()).join("");
                    dirs.insert(pth.clone(), ElfDirectory{path: pth.clone(), files: Vec::new(), dirs: Vec::new(), parent: Some(curr_head.to_string())});
                    dirs.get_mut(&curr_head.to_string()).unwrap().dirs.push(pth.clone());
                }
                _ => {
                    dirs.get_mut(&curr_head).unwrap_or_else(|| panic!("Couldn't find dir info!"))
                        .add_file(tokens[1], &tokens[0].parse::<i32>().ok().unwrap_or_else(|| panic!("Could not parse first character as number!")));
                }
            }
        });
        Ok(Input{dir_hash: dirs})
    }
}

fn main() {
    let input = INPUT.parse::<Input>().expect("The input must parse");

    println!("{}", input.dir_hash.values().map(|dir| dir.get_size(&input.dir_hash)).filter(|x| x <= &100000).sum::<i32>());

    let total_size = 70000000;
    let space_needed = 30000000;
    let space_to_free = space_needed - (total_size - input.dir_hash.get("/").unwrap().get_size(&input.dir_hash));
    println!("{}", input.dir_hash.values().filter(|dir| dir.get_size(&input.dir_hash) >= space_to_free).min_by(|a, b| {
        a.get_size(&input.dir_hash).cmp(&b.get_size(&input.dir_hash))
    }).unwrap().get_size(&input.dir_hash));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parse() {
        let input = "$ ls\ndir A\ndir B\ndir C\n100 ted.png\n$ cd A\n$ ls\ndir D\ndir E\n$ cd D\n$ ls\n300 Shogen.jpg\n$ cd ..\n$ cd E\n$ ls\n200 lynx.png\n$ cd /\n$ cd B\n$ ls\n12 foo.png\n45 bar.png"
        .parse::<Input>().expect("Input must parse");

        assert_eq!(input.dir_hash.get("//A/D").unwrap().files[0].name, "Shogen.jpg");
        assert_eq!(input.dir_hash.get("//A/D").unwrap().get_size(&input.dir_hash), 300);
        assert_eq!(input.dir_hash.get("//A").unwrap().get_size(&input.dir_hash), 500);
        assert_eq!(input.dir_hash.get("//B").unwrap().get_size(&input.dir_hash), 57);
        assert_eq!(input.dir_hash.get("/").unwrap().get_size(&input.dir_hash), 657);
    }
}