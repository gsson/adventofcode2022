use std::cell::{Ref, RefCell, RefMut};
use std::collections::BTreeMap;
use std::rc::Rc;
use Entry::{Dir, File};

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

#[derive(Copy, Clone, Debug)]
enum Line<'a> {
    List,
    ChangeDirectory(&'a str),
    FileEntry(&'a str, usize),
    DirectoryEntry(&'a str),
}

fn parse_line(line: &str) -> Line {
    let parts = line.split_terminator(' ').collect::<Vec<_>>();
    match parts.as_slice() {
        ["$", "ls"] => Line::List,
        ["$", "cd", dir] => Line::ChangeDirectory(dir),
        ["dir", dir] => Line::DirectoryEntry(dir),
        [size, file] => Line::FileEntry(file, size.parse().unwrap()),
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Line> {
    input.lines().map(parse_line)
}

type Entries = Rc<RefCell<BTreeMap<String, Entry>>>;

#[derive(Debug)]
enum Entry {
    File(usize),
    Dir(Entries, Option<usize>),
}

impl Entry {
    fn new_dir() -> Self {
        Dir(Default::default(), None)
    }

    fn new_file(size: usize) -> Self {
        File(size)
    }

    fn entries(&self) -> Entries {
        if let Dir(entries, _) = self {
            entries.clone()
        } else {
            panic!();
        }
    }

    // Mutable since I memoize directory sizes
    fn size(&mut self) -> usize {
        match self {
            File(size) => *size,
            Dir(_, Some(size)) => *size,
            Dir(dir_entries, memo) => {
                let size = dir_entries
                    .borrow_mut()
                    .values_mut()
                    .map(|e| e.size())
                    .sum();
                *memo = Some(size);
                size
            }
        }
    }

    fn visit_directory_usage<F: FnMut(usize)>(&mut self, visitor: &mut F) {
        if let Dir(entries, _) = self {
            entries
                .borrow_mut()
                .values_mut()
                .for_each(|v| v.visit_directory_usage(visitor));
            visitor(self.size());
        }
    }
}

#[derive(Debug)]
struct Usage {
    current_directory_entries: Vec<Entries>,
}

impl Usage {
    fn new() -> Self {
        Self {
            current_directory_entries: vec![Default::default()],
        }
    }

    fn current_directory(&mut self) -> Ref<BTreeMap<String, Entry>> {
        self.current_directory_entries.last().unwrap().borrow()
    }

    fn current_directory_mut(&mut self) -> RefMut<BTreeMap<String, Entry>> {
        self.current_directory_entries.last().unwrap().borrow_mut()
    }

    fn root_mut(&mut self) -> RefMut<BTreeMap<String, Entry>> {
        self.current_directory_entries.first().unwrap().borrow_mut()
    }

    fn apply(&mut self, line: Line) {
        match line {
            Line::List => {}
            Line::ChangeDirectory("/") => {
                self.current_directory_entries.truncate(1);
            }
            Line::ChangeDirectory("..") => {
                self.current_directory_entries.pop().unwrap();
            }
            Line::ChangeDirectory(dir) => {
                let entries = self.current_directory().get(dir).unwrap().entries();
                self.current_directory_entries.push(entries);
            }
            Line::FileEntry(file, size) => {
                self.current_directory_mut()
                    .insert(file.to_string(), Entry::new_file(size));
            }
            Line::DirectoryEntry(dir) => {
                self.current_directory_mut()
                    .insert(dir.to_string(), Entry::new_dir());
            }
        }
    }

    fn visit_directory_usage<F: FnMut(usize)>(&mut self, visitor: &mut F) {
        self.root_mut()
            .values_mut()
            .for_each(|e| e.visit_directory_usage(visitor));
    }

    fn space_used(&mut self) -> usize {
        self.root_mut().values_mut().map(|e| e.size()).sum()
    }
}

fn part1(input: &str) -> usize {
    let mut usage = Usage::new();
    parse_input(input).for_each(|line| usage.apply(line));

    let mut sizes = 0;
    usage.visit_directory_usage(&mut |size| {
        if size < 100000 {
            sizes += size
        }
    });

    sizes
}

#[test]
fn part1_example() {
    assert_eq!(95437, part1(EXAMPLE))
}

#[test]
#[ignore]
fn part1_verify() {
    assert_eq!(1989474, part1(INPUT))
}

fn part2(input: &str) -> usize {
    const TOTAL_DISK_SPACE: usize = 70000000;
    const SPACE_REQUIRED: usize = 30000000;

    let mut usage = Usage::new();
    parse_input(input).for_each(|line| usage.apply(line));

    let space_used = usage.space_used();
    let free_space = TOTAL_DISK_SPACE - space_used;
    let additional_space_needed = SPACE_REQUIRED - free_space;

    let mut candidate_size = usize::MAX;
    usage.visit_directory_usage(&mut |size| {
        if size >= additional_space_needed {
            candidate_size = candidate_size.min(size);
        }
    });

    candidate_size
}

#[test]
fn part2_example() {
    assert_eq!(24933642, part2(EXAMPLE))
}

#[test]
#[ignore]
fn part2_verify() {
    assert_eq!(1111607, part2(INPUT))
}
