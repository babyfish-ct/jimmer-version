use std::path::PathBuf;
use crate::gradle_modifier::GradleModifier;
use crate::modifier::Modifier;
use crate::pom_modifier::PomModifier;

pub struct Processor<'a> {
    version: &'a str
}

impl<'a> Processor<'a> {

    pub fn new(version: &'a str) -> Self {
        Processor { version }
    }

    pub fn handle_dir(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        for entry in path.read_dir()? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                self.handle_dir(entry.path())?;
            } else if let Some(file_name) = entry.path().file_name() {
                if let Some(file_name) = file_name.to_str() {
                    match file_name {
                        "pom.xml" => PomModifier::new(entry.path())
                            .execute(self.version)?,
                        "build.gradle" => GradleModifier::new(entry.path(), false)?
                            .execute(self.version)?,
                        "build.gradle.kts" => GradleModifier::new(entry.path(), true)?
                            .execute(self.version)?,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}