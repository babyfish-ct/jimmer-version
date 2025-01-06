use std::path::PathBuf;
use crate::modifier::Modifier;

pub struct PomModifier {
    path: PathBuf,
    properties_depth: i32,
    dependencies_depth: i32,
    build_depth: i32
}

impl PomModifier {

    pub fn new(path: PathBuf) -> Self {
        PomModifier {
            path,
            properties_depth: 0,
            dependencies_depth:0,
            build_depth: 0
        }
    }
}

impl Modifier for PomModifier {

    fn _path(&self) -> &PathBuf {
        &self.path
    }

    fn _replace(&mut self, line: &str, version: &str) -> Option<String> {
        let trimmed_line = line.trim();
        match trimmed_line {
            "<properties>" => {
                self.properties_depth += 1;
            }
            "</properties>" => {
                self.properties_depth -= 1;
            }
            "<dependencies>" => {
                self.dependencies_depth += 1;
            }
            "</dependencies>" => {
                self.dependencies_depth -= 1;
            }
            "<build>" => {
                self.build_depth += 1;
            }
            "</build>" => {
                self.build_depth -= 1;
            }
            _ => if self.properties_depth > 0 {
                let start = line.find(PREFIX);
                let end = line.rfind(SUFFIX);
                if let (Some(start), Some(end)) = (start, end) {
                    if start < end {
                        let line = format!(
                            "{}{}{}",
                            &line[0..start + PREFIX.len()],
                            version,
                            &line[end..]
                        );
                        return Some(line);
                    }
                }
            } else if self.dependencies_depth == 0 && self.build_depth == 0 {
                let start = line.find("<version>");
                let end = line.rfind("</version>");
                if let (Some(start), Some(end)) = (start, end) {
                    if start < end {
                        let line = format!(
                            "{}{}{}",
                            &line[0..start + 9],
                            version,
                            &line[end..]
                        );
                        return Some(line);
                    }
                }
            }
        }
        None
    }
}

static PREFIX: &str = "<jimmer.version>";
static SUFFIX: &str = "</jimmer.version>";