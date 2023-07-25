pub struct File {
    pub name: Box<str>,
    pub path: Box<str>,
    pub size: u64,
}

impl File {
    pub fn new(name: Box<str>, path: Box<str>, size: u64) -> Self {
        Self { name, path, size }
    }
}
