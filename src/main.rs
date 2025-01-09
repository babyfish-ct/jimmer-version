mod pom_modifier;
mod gradle_modifier;
mod modifier;
mod processor;

use std::{env};
use std::path::PathBuf;
use crate::processor::Processor;

fn main() {
    println!("Please input version: ");
    let mut version = String::new();
    std::io::stdin().read_line(&mut version).expect("Failed to read version");
    version = version.trim().to_string();
    let path = root_dir().expect("Failed to get root directory");
    Processor::new(version.as_str()).handle_dir(path).expect("Failed to read directory");
}

fn root_dir() -> Result<PathBuf, std::io::Error> {
    let path = env::current_dir()?;
    for ancestor in path.ancestors() {
        if let Some(file_name) = ancestor.file_name() {
            if file_name == JIMMER_EXAMPLES {
                return Ok(ancestor.to_path_buf());
            }
        }
    }
    println!(
        "Cannot find '{}' in ancestors of current path, uses the default path '{}'",
        JIMMER_EXAMPLES,
        DEFAULT_PATH
    );
    Ok(PathBuf::from(DEFAULT_PATH))
}

static JIMMER_EXAMPLES : &str = "jimmer-examples";
static DEFAULT_PATH : &str = "/Users/chentao/projects/git/jimmer-examples";