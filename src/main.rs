use std::{ env, fs };

use domain::{ FileReader, QuerySearcher };

pub struct SystemFileReader;

impl FileReader for SystemFileReader {
    fn read_to_string(&self, file_path: String) -> Result<String, ()> {
        fs::read_to_string(file_path).map_err(|_| ())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Parsing arguments error: {}", err);
        std::process::exit(1);
    });

    let lines = QuerySearcher::new(SystemFileReader).search(&config.query, &config.file_path);

    lines.iter().for_each(|(line_nr, line)| println!("{}: {}", line_nr, line));
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("not enough arguments specified".to_string());
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query: query, file_path: file_path })
    }
}
