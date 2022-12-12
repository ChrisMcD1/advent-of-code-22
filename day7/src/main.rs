use anyhow::anyhow;
use anyhow::Result;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

struct File {
    size: u32,
    name: String,
}

impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, name) = s.split_once(" ").unwrap();
        return Ok(Self {
            size: size.parse().unwrap(),
            name: name.trim().to_string(),
        });
    }
}

struct Directory {
    files: Vec<File>,
    parent: Option<Rc<RefCell<Directory>>>,
    nested_directories: Vec<Rc<RefCell<Directory>>>,
    name: String,
}

impl FromStr for Directory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_text, name) = s.split_once(" ").unwrap();
        if dir_text != "dir" {
            return Err(anyhow!("Parsing Dir from not a dir"));
        }
        return Ok(Directory::new(name.to_string()));
    }
}

impl Directory {
    fn new(name: String) -> Self {
        return Self {
            files: vec![],
            parent: None,
            nested_directories: vec![],
            name,
        };
    }
    fn add_directory(&mut self, new_node: Rc<RefCell<Directory>>) {
        self.nested_directories.push(new_node);
    }
    fn add_file(&mut self, new_file: File) {
        self.files.push(new_file);
    }
}

fn part_1(input: &str) -> Result<u32> {
    return Ok(0);
}

fn build_tree(input: &str) -> Result<Rc<RefCell<Directory>>> {
    let (init_cd, rest) = input.split_once("\n").unwrap();

    let mut tree_root = Rc::new(RefCell::new(Directory::new("/".to_string())));

    let mut pointer = tree_root.clone();

    rest.lines()
        .for_each(|line| match line.chars().nth(0).unwrap() {
            '$' => {
                let mut words = line.split(" ");
                let command = words.nth(1).unwrap();
                match command {
                    "ls" => return,
                    "cd" => {
                        let target = words.nth(0).unwrap();
                        match target {
                            ".." => {
                                pointer = pointer.clone().borrow().parent.clone().unwrap();
                            }
                            "/" => {
                                pointer = tree_root.clone();
                            }
                            _ => {
                                let target_child = pointer
                                    .borrow()
                                    .nested_directories
                                    .iter()
                                    .find(|&child| child.borrow().name == target)
                                    .clone();
                                pointer = target_child.unwrap();
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => {
                let dir = line.parse::<Directory>();
                match dir {
                    Ok(dir_ok) => {
                        pointer
                            .as_ref()
                            .borrow_mut()
                            .add_directory(Rc::new(RefCell::new(dir_ok)));
                    }
                    Err(_) => {
                        let file = line.parse::<File>().unwrap();
                        pointer.as_ref().borrow_mut().add_file(file);
                    }
                }
            }
        });

    Ok(tree_root)
}

#[cfg(test)]
mod test {
    use crate::part_1;

    #[test]
    fn part_1_given() {
        let input = include_str!("./given_1");

        let result = part_1(input).unwrap();

        assert_eq!(result, 95437);
    }
}
