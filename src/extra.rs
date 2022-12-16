use std::{env, fmt::Display, path::PathBuf};

use crate::day7::Directory;

pub struct GraphMetadata {
    pub x: i64,
    pub y: i64,
    pub width: u32,
    pub height: u32,
    pub legend_step_x: u32,
    pub legend_step_y: u32,
    pub legend_y_width: u32,
    pub legend_x_width: u32,
}

pub trait ChristmasGraph {
    fn as_graph_metadata(&self) -> GraphMetadata;
    fn graph_legend_x(&self, value: i64) -> Vec<char>;
    fn graph_legend_y(&self, value: i64) -> Vec<char>;
    fn graph_value(&self, x: i64, y: i64) -> Option<char>;
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let graph_metadata = self.as_graph_metadata();

        let graph_x = graph_metadata.x;
        let graph_y = graph_metadata.y;
        let width = graph_metadata.width;
        let height = graph_metadata.height;
        let end_x = graph_x + width as i64;
        let end_y = graph_y + height as i64;
        let step_x = graph_metadata.legend_step_x;
        let step_y = graph_metadata.legend_step_y;

        let legend_x_width = graph_metadata.legend_x_width;
        let legend_y_width = graph_metadata.legend_y_width;

        for y in 0..legend_x_width as usize {
            write!(f, "{:>width$}", "", width = legend_y_width as usize + 1)?;
            for x in graph_x..end_x {
                if x % step_x as i64 == 0 {
                    let legend = *self.graph_legend_x(x).get(y).unwrap_or(&' ');
                    write!(f, "{} ", legend)?;
                } else if x + 1 - graph_x != width as i64 {
                    write!(f, "  ")?;
                }
            }
            write!(f, " ")?;
            writeln!(f)?;
        }

        for y in graph_y..end_y {
            if y % step_y as i64 == 0 {
                let legend = self.graph_legend_y(y);

                for _ in 0..(legend_y_width as usize - legend.len()) {
                    write!(f, " ")?;
                }

                for char in legend {
                    char.fmt(f)?;
                }
            } else {
                write!(f, "{:>width$}", "", width = legend_y_width as usize)?;
            }

            write!(f, " ")?;

            for x in graph_x..end_x {
                if let Some(value) = self.graph_value(x, y) {
                    value.fmt(f)?;
                } else {
                    write!(f, ".")?;
                }
                if x + 1 - graph_x != width as i64 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

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

pub fn visualize<T>(day: u8, part: u8, value: &T)
where
    T: Display,
{
    fn perform_print<T>(_day: u8, _part: u8, value: &T)
    where
        T: Display,
    {
        // let line = format!("Day {day:} part {part:}");
        // let line = format!("{line:-^80}");
        // println!("{line:}");
        println!("{value:}")
    }

    if let Ok(visualize) = env::var("VISUALIZE") {
        if visualize != "true" && visualize != "1" {
            return;
        }
    }

    match env::var("DAY") {
        Ok(day_str) => {
            if day_str.parse::<u8>().unwrap() != day {
                return;
            }

            match env::var("PART") {
                Ok(part_str) => {
                    if part_str.parse::<u8>().unwrap() != part {
                        return;
                    }
                    perform_print(day, part, value)
                }
                Err(_) => perform_print(day, part, value),
            }
        }
        Err(_) => {
            if cfg!(test) {
                perform_print(day, part, value)
            }
        }
    }
}
