use itertools::Itertools;
use crate::day7::FileSystemEntry::{Dir, File};
use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day7.txt");

    let fs = reverse_engineer_file_system(input);
    println!("Day 6, Part 1: {:?}", fs.clone().part_1());
    println!("Day 6, Part 2: {:?}", fs.part_2());
}

#[derive(PartialEq, Debug, Clone)]
enum FileSystemEntry {
    Dir { name: String, entries: Vec<FileSystemEntry> },
    File { name: String, size: u32 },
}

impl FileSystemEntry {
    fn navigate<'a>(&'a mut self, fragment: String) -> Option<&'a mut FileSystemEntry> {
        match self {
            Dir { entries, .. } => {
                entries.into_iter().find(|e| {
                    match e {
                        Dir { name, .. } | File { name, .. } => {
                            name == &fragment
                        }
                    }
                })
            }
            File { .. } => None
        }
    }

    fn calc_size(self) -> u32 {
        match self {
            Dir { entries, .. } => {
                entries.into_iter().map(|x| x.calc_size()).sum()
            }
            File { size, .. } => size
        }
    }

    fn part_1(self) -> u32 {
        let size = self.clone().calc_size();
        match self {
            Dir { entries, .. } => {
                let rec: u32 = entries.into_iter().map(|x| x.part_1()).sum();
                if size <= 100_000 {
                    size + rec
                } else {
                    rec
                }
            }
            _  => 0
        }
    }

    fn dirs(self) -> Vec<FileSystemEntry> {
        let cop = self.clone();
        match self {
            Dir { entries, .. } => {
                let mut vec = entries.into_iter().flat_map(|e| e.dirs()).collect_vec();
                vec.push(cop);
                vec
            }
            File { .. } => Vec::new()
        }
    }

    fn part_2(self) -> u32 {
        let used_space = self.clone().calc_size();
        let total_space = 70000000u32;
        let free_space = total_space - used_space;
        let needed_space = 30000000 - free_space;

        let dirs = self.clone().dirs();
        dirs.into_iter()
            .map(|e| e.calc_size())
            .filter(|e| e.clone() >= needed_space)
            .min().unwrap()
    }
}

fn reverse_engineer_file_system(console: Vec<String>) -> FileSystemEntry {
    let mut fs = Dir { name: "/".to_string(), entries: vec![] };

    let mut path: Vec<String> = vec![];

    let mut it = console.iter();

    while let Some(line) = it.next() {
        match line.split(" ").collect_vec()[..] {
            ["$", "cd", "/"] => path = vec![],
            ["$", "cd", ".."] => { path.pop(); }
            ["$", "cd", fragment] => path.push(fragment.to_string()),
            ["$", "ls"] => {}
            ["dir", name] => {
                create_if_not_exists(path.clone(), Dir { name: name.to_string(), entries: vec![] }, &mut fs)
            }
            [number, name] if number.parse::<u32>().is_ok() => {
                create_if_not_exists(path.clone(), File { name: name.to_string(), size: number.parse().unwrap() }, &mut fs)
            }
            _ => {}
        }
    }

    fs
}

fn create_if_not_exists(path: Vec<String>, new: FileSystemEntry, fs: &mut FileSystemEntry) {
    match path.iter().next() {
        None => {
            if let Dir { entries, .. } = fs {
                entries.push(new);
            }
        }
        Some(fragment) => {
            let without_first = path.clone().into_iter().dropping(1).collect_vec();
            match fs.navigate(fragment.clone()) {
                Some(refer) => {
                    create_if_not_exists(without_first, new, refer);
                }
                None => {
                    if let Dir { entries, .. } = fs {
                        let new_dir = Dir { name: fragment.clone(), entries: vec![] };
                        entries.push(new_dir);
                    }
                    create_if_not_exists(path, new, fs)
                }
            }
        }
    }
}

// fn create_if_not_exists(path: &Vec<String>, new: FileSystemEntry, mut fs: &mut FileSystemEntry) {
//     let mut dir: &mut FileSystemEntry = &mut fs;
//     for fragment in path {
//         match dir.navigate(fragment.clone()) {
//             None => {
//                 if let Dir { entries, .. } = dir {
//                     let mut new_dir = Dir {name: fragment.clone(), entries: vec![]};
//                     entries.push(new_dir);
//                     dir = &mut &mut new_dir
//                 }
//             }
//             Some(x) => {
//                 dir = &mut
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::day7::*;
    use crate::day7::FileSystemEntry::{Dir, File};

    #[test]
    fn find_packet_marker_for_example() {
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];

        let expected = Dir {
            name: "/".to_string(),
            entries: vec![
                Dir {
                    name: "a".to_string(),
                    entries: vec![
                        Dir {
                            name: "e".to_string(),
                            entries: vec![File { name: "i".to_string(), size: 584 }],
                        },
                        File { name: "f".to_string(), size: 29116 },
                        File { name: "g".to_string(), size: 2557 },
                        File { name: "h.lst".to_string(), size: 62596 },
                    ],
                },
                File { name: "b.txt".to_string(), size: 14848514 },
                File { name: "c.dat".to_string(), size: 8504156 },
                Dir {
                    name: "d".to_string(),
                    entries: vec![
                        File { name: "j".to_string(), size: 4060174 },
                        File { name: "d.log".to_string(), size: 8033020 },
                        File { name: "d.ext".to_string(), size: 5626152 },
                        File { name: "k".to_string(), size: 7214296 },
                    ],
                },
            ],
        };

        assert_eq!(reverse_engineer_file_system(input), expected.clone());
        assert_eq!(expected.clone().calc_size(), 48381165);
        assert_eq!(expected.clone().part_1(), 95437);
        assert_eq!(expected.part_2(), 24933642)

    }
}









