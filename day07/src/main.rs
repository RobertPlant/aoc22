mod input;

use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct File {
    name: String,
    size: usize,
    is_dir: bool,
}

fn parse(input_data: &'static str) -> HashSet<File> {
    let mut structure: HashSet<File> = HashSet::new();
    let mut current_directory = String::from("");

    input_data.lines().for_each(|command| {
        let is_cmd = command.chars().nth(0).unwrap() == '$';
        let mut cmd = command.split(" ");

        if is_cmd {
            let binary = cmd.nth(1).unwrap();

            match binary {
                "cd" => {
                    let location = cmd.nth(0).unwrap();
                    if location == "/" {
                        current_directory = String::from("");
                    } else if location == ".." {
                        current_directory = current_directory
                            .clone()
                            .rsplit_once("/")
                            .unwrap_or(("", ""))
                            .0
                            .to_string();
                    } else {
                        current_directory = if current_directory == "" {
                            location.to_string()
                        } else {
                            current_directory.clone() + "/" + location
                        };
                    }
                }
                _ => (),
            }
        } else {
            let first_part = cmd.nth(0).unwrap().parse::<usize>();
            let file_name = cmd.nth(0).unwrap();
            let prefix = if current_directory == "" {
                String::from("")
            } else {
                current_directory.clone() + "/"
            };

            if first_part.is_ok() {
                structure.insert(File {
                    name: prefix + file_name,
                    size: first_part.unwrap(),
                    is_dir: false,
                });
            } else {
                structure.insert(File {
                    name: prefix + file_name,
                    size: 0,
                    is_dir: true,
                });
            }
        }
    });

    structure
        .iter()
        .map(|file| File {
            name: file.name.clone(),
            size: if file.is_dir {
                structure
                    .iter()
                    .filter(|f| f.name.starts_with(&file.name))
                    .map(|f| f.size)
                    .sum()
            } else {
                file.size
            },
            is_dir: file.is_dir,
        })
        .collect()
}

fn calc(input_data: &'static str) -> usize {
    parse(input_data)
        .iter()
        .filter(|file| file.is_dir && file.size < 100000)
        .map(|file| file.size)
        .sum()
}

fn calc_p2(input_data: &'static str) -> usize {
    let structure = parse(input_data);

    let total_size = structure
        .iter()
        .filter(|file| !file.name.contains("/"))
        .map(|f| f.size)
        .sum::<usize>();

    let mut folders: Vec<usize> = structure
        .iter()
        .filter(|file| file.is_dir)
        .map(|file| file.size)
        .collect();

    folders.sort();

    folders
        .into_iter()
        .find(|size| (70000000 - total_size) + size > 30000000)
        .unwrap()
        .clone()
}

fn main() {
    let input_data = input::get_input();

    println!("Found P1 {:?}", calc(input_data));
    println!("Found P2 {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            ),
            95437
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            ),
            24933642
        )
    }
}
