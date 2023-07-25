use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::file::File;

pub struct Directory {
    pub name: Box<str>,
    pub path: Box<str>,
    pub files: Vec<File>,
    pub directories: Vec<Directory>,
}

impl Directory {
    pub fn new(name: Box<str>, path: Box<str>) -> Self {
        Self {
            name,
            path,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn directories(&self) -> &Vec<Directory> {
        &self.directories
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn add_directory(&mut self, directory: Directory) {
        self.directories.push(directory);
    }

    pub fn get_file(&self, name: &str) -> Option<&File> {
        self.files.iter().find(|f| f.name == name)
    }

    pub fn get_directory(&self, name: &str) -> Option<&Directory> {
        self.directories.iter().find(|d| d.name == name)
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}

impl PartialEq<&str> for Directory {
    fn eq(&self, other: &&str) -> bool {
        self.name == *other || self.path == *other
    }
}

impl PartialEq<str> for Directory {
    fn eq(&self, other: &str) -> bool {
        self.name == other || self.path == other
    }
}

impl PartialEq<String> for Directory {
    fn eq(&self, other: &String) -> bool {
        self.name == *other || self.path == *other
    }
}

impl PartialEq<Box<str>> for Directory {
    fn eq(&self, other: &Box<str>) -> bool {
        self.name == *other || self.path == *other
    }
}

impl Into<String> for Directory {
    fn into(self) -> String {
        self.path.into()
    }
}

impl Into<Box<str>> for Directory {
    fn into(self) -> Box<str> {
        self.path
    }
}

impl Into<&str> for Directory {
    fn into(self) -> &'static str {
        self.path.as_ref()
    }
}

impl Eq for Directory {}

impl Add for Directory {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            name: self.name,
            path: self.path,
            files: self.files.into_iter().chain(other.files).collect(),
            directories: self
                .directories
                .into_iter()
                .chain(other.directories)
                .collect(),
        }
    }
}

impl AddAssign for Directory {
    fn add_assign(&mut self, other: Self) {
        self.add(other);
    }
}

impl Sub for Directory {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            name: self.name,
            path: self.path,
            files: self
                .files
                .into_iter()
                .filter(|f| !other.files.contains(f))
                .collect(),
            directories: self
                .directories
                .into_iter()
                .filter(|d| !other.directories.contains(d))
                .collect(),
        }
    }
}

impl SubAssign for Directory {
    fn sub_assign(&mut self, other: Self) {
        self.sub(other);
    }
}

pub struct DirectoryBuilder {
    name: Box<str>,
    path: Box<str>,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl DirectoryBuilder {
    pub fn new(name: Box<str>, path: Box<str>) -> Self {
        Self {
            name,
            path,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    pub fn add_file(mut self, file: File) -> Self {
        self.files.push(file);
        self
    }

    pub fn add_directory(mut self, directory: Directory) -> Self {
        self.directories.push(directory);
        self
    }

    pub fn add_files(mut self, files: Vec<File>) -> Self {
        self.files = files;
        self
    }

    pub fn add_directories(mut self, directories: Vec<Directory>) -> Self {
        self.directories = directories;
        self
    }

    pub fn build(self) -> Directory {
        Directory {
            name: self.name,
            path: self.path,
            files: self.files,
            directories: self.directories,
        }
    }
}
