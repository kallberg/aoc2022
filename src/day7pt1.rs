use std::{collections::HashMap, path::PathBuf};

type FileSystem = HashMap<PathBuf, Vec<usize>>;
type SizedFileSystem = HashMap<PathBuf, usize>;

pub fn parse_file(value: &str) -> Option<usize> {
    let (size_str, _) = value.split_once(' ')?;

    let size: usize = size_str.parse().ok()?;

    Some(size)
}

pub fn direct_size(file_system: &HashMap<PathBuf, Vec<usize>>, path: &PathBuf) -> usize {
    file_system
        .get(path)
        .expect("list directory contents")
        .iter()
        .sum()
}

pub fn sub_directories(
    file_system: &HashMap<PathBuf, Vec<usize>>,
    directory: &PathBuf,
) -> Vec<PathBuf> {
    let children = file_system
        .keys()
        .filter(|path| path.ancestors().any(|a| a.eq(directory)))
        .filter(|path| !path.eq(&directory))
        .cloned();

    children.collect()
}

pub fn size_file_system(
    file_system: &HashMap<PathBuf, Vec<usize>>,
    directory: &PathBuf,
    sized_file_system: &mut SizedFileSystem,
) -> usize {
    if let Some(size) = sized_file_system.get(directory) {
        return *size;
    }

    let mut size = 0;

    for child in sub_directories(file_system, directory) {
        size += size_file_system(file_system, &child, sized_file_system);
    }

    size += direct_size(file_system, directory);

    sized_file_system.insert(directory.clone(), size);

    size
}

pub fn parse_filesystem(input: &str) -> FileSystem {
    let mut current_path: PathBuf = PathBuf::from("/");
    let mut file_system = FileSystem::new();
    file_system.insert(current_path.clone(), vec![]);

    for line in input.lines() {
        if line.starts_with('$') {
            let cmd = line.strip_prefix("$ ").unwrap();

            if let Some(dir) = cmd.strip_prefix("cd ") {
                if dir.eq("..") {
                    current_path.pop();
                } else {
                    current_path.push(dir)
                }

                if !file_system.contains_key(&current_path) {
                    file_system.insert(current_path.clone(), vec![]);
                }
            }
        } else if line.starts_with("dir") {
            continue;
        } else {
            let files: &mut Vec<usize> = file_system.get_mut(&current_path).unwrap();

            let file =
                parse_file(line).unwrap_or_else(|| panic!("parse file, from line: {}", line));

            files.push(file);
        }
    }

    file_system
}

pub fn directory_finder(file_system: &FileSystem, size_limit: usize) -> Vec<(PathBuf, usize)> {
    let mut cache = SizedFileSystem::new();

    file_system
        .keys()
        .map(|path| {
            (
                path.clone(),
                size_file_system(file_system, path, &mut cache),
            )
        })
        .filter(|(_, size)| *size <= size_limit)
        .collect()
}

pub fn solve(input: &str) -> usize {
    let file_system = parse_filesystem(input);

    let directories = directory_finder(&file_system, 100000);

    let mut sum = 0;

    for (path, size) in directories {
        println!("directory={}, size={}", path.to_str().unwrap(), size);
        sum += size
    }

    sum
}
