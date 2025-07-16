use std::collections::HashSet;

pub struct TextViewer {
    pub lines: Vec<Vec<String>>,
    pub highlighted: HashSet<(usize, usize)>,
}

impl TextViewer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            highlighted: HashSet::new(),
        }
    }

    pub fn load_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        self.lines = content
            .lines()
            .map(|line| line.split_whitespace().map(String::from).collect())
            .collect();
        self.highlighted.clear();
        Ok(())
    }
}
