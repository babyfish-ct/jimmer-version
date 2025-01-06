use std::io::{Write};
use std::fs;
use std::path::PathBuf;

pub trait Modifier {

    /* protected */ fn _path(&self) -> &PathBuf;

    /* protected */ fn _replace(&mut self, line: &str, version: &str) -> Option<String>;

    fn execute(&mut self, version: &str) -> Result<(), std::io::Error> {
        enum Line<'a> {
            Old(&'a str),
            New(String),
        }
        let content = fs::read_to_string(&self._path())?;
        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines: Vec<Line<>> = Vec::new();
        for line in lines {
            if let Some(line) = self._replace(line, version) {
                new_lines.push(Line::New(line));
            } else {
                new_lines.push(Line::Old(line));
            }
        }
        if !new_lines.is_empty() {
            let mut file = fs::File::create(self._path())?;
            for line in new_lines {
                match line {
                    Line::New(line) => writeln!(file, "{}", line)?,
                    Line::Old(line) => writeln!(file, "{}", line)?,
                }
            }
        }
        Ok(())
    }
}