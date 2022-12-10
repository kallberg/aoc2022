use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Listing {
    path: PathBuf,
    files: Vec<File>,
    directories: Vec<PathBuf>,
}

pub type Listings = HashMap<PathBuf, Listing>;

#[derive(Clone, Debug)]
pub struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub children: Vec<Directory>,
    pub files: Vec<File>,
}

impl Directory {
    pub fn size(&self, cache: &mut HashMap<PathBuf, usize>) -> usize {
        if let Some(size) = cache.get(&self.path) {
            return *size;
        }

        let mut size = 0;

        size += self.files.iter().map(|file| file.size).sum::<usize>();

        for child in &self.children {
            size += child.size(cache)
        }

        size
    }

    pub fn descendants(&self) -> Vec<Directory> {
        let mut output = vec![];

        for child in &self.children {
            output.push(child.clone());
            let mut child_descendands = child.descendants();
            output.append(&mut child_descendands);
        }

        output
    }
}

pub fn parse_file(line: &str) -> Option<File> {
    let (size_str, name) = line.split_once(' ')?;

    let size: usize = size_str.parse().ok()?;

    Some(File {
        name: name.to_string(),
        size,
    })
}

pub fn parse_dir(current_path: PathBuf, line: &str) -> Option<PathBuf> {
    let dir_str = line.strip_prefix("dir ")?;

    let mut output = current_path;
    output.push(dir_str);

    Some(output)
}

pub fn listing_to_directory(listing: &Listing, listings: &Listings) -> Directory {
    let mut children = Vec::<Directory>::new();

    for directory_ref in &listing.directories {
        if let Some(directory_ref_listing) = listings.get(directory_ref) {
            let child = listing_to_directory(directory_ref_listing, listings);
            children.push(child)
        }
    }

    let files = &listing.files;

    Directory {
        path: listing.path.clone(),
        children,
        files: files.clone(),
    }
}

pub fn parse_listings(input: &str) -> Listings {
    let mut current_path: PathBuf = PathBuf::from("/");
    let mut listings = Listings::new();

    listings.insert(
        current_path.clone(),
        Listing {
            path: current_path.clone(),
            files: vec![],
            directories: vec![],
        },
    );

    for line in input.lines() {
        if line.starts_with('$') {
            let cmd = line.strip_prefix("$ ").unwrap();

            if let Some(dir) = cmd.strip_prefix("cd ") {
                if dir.eq("..") {
                    current_path.pop();
                } else {
                    current_path.push(dir);
                }

                if !listings.contains_key(&current_path) {
                    listings.insert(
                        current_path.clone(),
                        Listing {
                            path: current_path.clone(),
                            files: vec![],
                            directories: vec![],
                        },
                    );
                }
            }
        } else if line.starts_with("dir") {
            let listing = listings.get_mut(&current_path).unwrap();

            let dir_path = parse_dir(current_path.clone(), line).expect("parse dir");

            listing.directories.push(dir_path);
        } else {
            let listing = listings.get_mut(&current_path).unwrap();

            let file =
                parse_file(line).unwrap_or_else(|| panic!("parse file, from line: {}", line));

            listing.files.push(file);
        }
    }

    listings
}

pub fn directories_within_limit(size_limit: usize, root: &Directory) -> Vec<(PathBuf, usize)> {
    let mut output = vec![];
    let mut cache = HashMap::<PathBuf, usize>::new();

    let root_size = root.size(&mut cache);

    if root_size <= size_limit {
        output.push((root.path.clone(), root_size));
    }

    for descendant in root.descendants() {
        let descendand_size = descendant.size(&mut cache);

        if descendand_size <= size_limit {
            output.push((descendant.path.clone(), descendand_size));
        }
    }

    output
}

impl From<&str> for Directory {
    fn from(input: &str) -> Self {
        let listings = parse_listings(input);
        let root_listing = listings.get(&PathBuf::from("/")).unwrap();
        listing_to_directory(root_listing, &listings)
    }
}

pub fn solve(input: &str) -> String {
    let root = Directory::from(input);

    let mut sum = 0;

    for (_, size) in directories_within_limit(100000, &root) {
        sum += size;
    }

    sum.to_string()
}
