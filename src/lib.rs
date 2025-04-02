pub trait FileReader {
    fn read_to_string(&self, file_path: String) -> Result<String, ()>;
}
pub struct QuerySearcher<F: FileReader> {
    file_reader: F,
}

impl<F: FileReader> QuerySearcher<F> {
    pub fn new(file_reader: F) -> Self {
        QuerySearcher { file_reader: file_reader }
    }

    pub fn search(&self, query: &str, file_path: &str) -> Vec<(usize, String)> {
        let content = self.file_reader.read_to_string(file_path.to_string()).unwrap();
        let mut matches: Vec<(usize, String)> = vec![];

        for (index, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                matches.push((index + 1, line.to_string()));
            }
        }

        return matches;
    }
}
