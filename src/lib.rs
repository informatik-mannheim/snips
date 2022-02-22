// lib
pub mod file;
pub mod parser;
pub mod util;

use crate::file::write_files;
use crate::parser::parse;
use crate::util::Setting;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use log::{info, debug};

pub const DEFAULTLABEL: &str = "x8gfz4hd"; // just a crazy string.

pub fn scan(setting: Setting) -> Result<(), String> {
    // val mode = if (exerciseEnv) "(mode EXC) " else ""

    // First, we need to check if all directories are valid and available.

    // Verify that source directory exists:
    let metadata = fs::metadata(&setting.src_dir);
    if let Err(e) = metadata {
        let err = format!(
            "Error: Source directory {} does not exist.\n{}",
            &setting.src_dir.display(),
            e
        );
        return Err(err);
    }
    if !setting.src_dir.is_dir() {
        return Err(format!(
            "Error: Source directory {} is not a directory.",
            &setting.src_dir.display()
        ));
    }

    // Verify that snips directory is available:
    if !setting.snippet_dest_dir.is_dir() {
        info!(
            "Create snips destination directory: {}",
            &setting.snippet_dest_dir.display()
        );
        if let Err(e) = fs::create_dir(setting.snippet_dest_dir) {
            return Err(format!(
                "Error: snippet destination directory {} could not be created.\n{}",
                &setting.src_dir.display(),
                e
            ));
        }
    }

    // Verify that src_dest directory is available:
    if !setting.src_dest_dir.is_dir() {
        info!(
            "Create source destination directory: {}",
            &setting.src_dest_dir.display()
        );
        if let Err(e) = fs::create_dir(setting.src_dest_dir) {
            return Err(format!(
                "Error: Source destination directory {} could not be created.\n{}",
                &setting.src_dest_dir.display(),
                e
            ));
        }
    }

    info!("Scanning...");
    if let Err(e) = scan_rec(&setting.src_dir, &setting.src_dest_dir, &setting) {
        return Err(format!("Scanning files failed with error: {}", e));
    }
    info!("... done");
    Ok(())
}

/// Scan the files in directory `dir` recursively.
/// `dir_path` is the path of directories starting from src directory.
/// `setting` contains the environment for the scan.
fn scan_rec(dir: &Path, dir_path: &Path, setting: &Setting) -> Result<(), String> {
    debug!(" {}", dir.display());

    // Recursively scan other directories:
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let next_dir = entry.path();
        if next_dir.is_dir() {
            // Add next directory to dir path:
            let ext_dir_path = dir_path.join(&next_dir.file_name().unwrap());
            // Make sure nested source destination dirs exist:
            if !ext_dir_path.is_dir() {
                if let Err(e) = fs::create_dir(&ext_dir_path) {
                    return Err(format!(
                        "Error: Nested source destination directory {} could not be created.\n{}",
                        ext_dir_path.display(),
                        e
                    ));
                }
            }
            scan_rec(&next_dir, &ext_dir_path, setting)?;
        } else {
            // TODO consider path.extension()
            // Test if file matches suffix:
            if let Some(filename) = next_dir.to_str() {
                if filename.ends_with(setting.file_suffix) {
                    // Process file:
                    info!(" {}", next_dir.display());
                    parse_write(next_dir.as_path(), &dir_path, setting)?;
                } else {
                    info!(" skipped {}", next_dir.display());
                }
            }
        }
    }
    Ok(())
}

pub fn parse_write(filepath: &Path, dir_path: &Path, setting: &Setting) -> Result<(), String> {
    // Make vector of the lines in the text file:
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(&file);
    let v: Vec<String> = reader.lines().map(|e| e.unwrap()).collect();
    let lines: Vec<&str> = v.iter().map(|s| s as &str).collect();

    // Parse the content of the file:
    let coll = parse(&lines, setting)?;
    write_files(filepath, dir_path, &coll, setting);
    Ok(())
}

// Unit tests

#[cfg(test)]
mod tests {
    // There are no unit tests for lib. They are in ./tests
}
