use std::{fmt::Display, path::PathBuf};

use crate::day7pt1::Directory;

impl Directory {
    pub fn name(&self) -> String {
        if self.path.eq(&PathBuf::from("/")) {
            return "/".into();
        }

        self.path.file_name().unwrap().to_string_lossy().to_string()
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- {} (dir)", self.name())?;

        for child in &self.children {
            for child_line in format!("{}", child).lines() {
                writeln!(f, "  {}", child_line)?;
            }
        }

        for file in &self.files {
            writeln!(f, "  - {} (file, {})", file.name, file.size)?;
        }

        Ok(())
    }
}
