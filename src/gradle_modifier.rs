use std::path::PathBuf;
use regex::Regex;
use crate::modifier::Modifier;

pub struct GradleModifier {
    path: PathBuf,
    regex: Regex,
    optinal_regex: Option<Regex>,
}

impl GradleModifier {

    pub fn new(path: PathBuf, kts: bool) -> Result<Self, regex::Error> {
        let regex = match kts {
            true => Regex::new(r"^\s*val\s+jimmerVersion\s*=")?,
            false => Regex::new(r"^\s*jimmerVersion\s*=")?
        };
        let optinal_regex = match kts {
            true => Some(Regex::new(r"^\s*val\s+jimmerVersion\s+by\s+extra\s*\{")?),
            false => None
        };
        Ok(GradleModifier{path, regex, optinal_regex })
    }
}

impl Modifier for GradleModifier {
    fn _path(&self) -> &PathBuf {
        &self.path
    }

    fn _replace(&mut self, line: &str, version: &str) -> Option<String> {
        if self.regex.is_match(line) {
            let eq = line.find("=");
            if let Some(eq) = eq {
                let line = format!(
                    "{}{}\"{}\"",
                    &line[0..eq+1],
                    " ",
                    version
                );
                return Some(line);
            }
        }
        if let Some(ref regex) = self.optinal_regex {
            if regex.is_match(line) {
                let start = line.find("{");
                if let Some(start) = start {
                    let line = format!(
                        "{}{}{}{}",
                        &line[0..start+1],
                        " \"",
                        version,
                        "\" }"
                    );
                    return Some(line);
                }
            }
        }
        None
    }
}
