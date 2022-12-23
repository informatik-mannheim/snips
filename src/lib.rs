pub mod file;
pub mod parser;
pub mod util;

use crate::file::write_files;
use crate::parser::parse;
use crate::util::Setting;
use log::{debug, info, warn};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use try_catch::catch;

pub const DEFAULTLABEL: &str = "x8gfz4hd"; // crazy string as an ID for default label

/// Scan all files as specified in `setting`.
pub fn scan(setting: &Setting) -> Result<(), String> {
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

    // Verify that snippet directory is available:
    if !setting.snippet_dest_dir.is_dir() {
        warn!(
            "Create snips destination directory: {}",
            &setting.snippet_dest_dir.display()
        );
        if let Err(e) = fs::create_dir(&setting.snippet_dest_dir) {
            return Err(format!(
                "Error: snippet destination directory {} could not be created.\n{}",
                &setting.src_dir.display(),
                e
            ));
        }
    }

    // Verify that src_dest directory is available:
    if !setting.src_dest_dir.is_dir() {
        warn!(
            "Create source destination directory: {}",
            &setting.src_dest_dir.display()
        );
        if let Err(e) = fs::create_dir(&setting.src_dest_dir) {
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

/// Scan the files in directory `src_dir` recursively. `src_dir` is the root
/// directory as specified in `setting` when `scan_rec` is called for the first time.
/// `src_dest_dir` is the destination source directory. It is the root
/// directory as specified in `setting`when `scan_rec` is called for the first time.
/// `setting` contains the environment for the scan.
fn scan_rec(src_dir: &Path, src_dest_dir: &Path, setting: &Setting) -> Result<(), String> {
    debug!(" {}", src_dir.display());

    // Recursively scan other directories:
    for entry in fs::read_dir(src_dir).unwrap() {
        let entry = entry.unwrap();
        let next_dir_or_file = entry.path();
        if next_dir_or_file.is_dir() {
            let dir = next_dir_or_file; // for better reading...
                                        // Add next directory to dir path:
            let ext_dir_path = src_dest_dir.join(&dir.file_name().unwrap());
            // Make sure nested source destination dirs exist (actually it should):
            if !ext_dir_path.is_dir() {
                if let Err(e) = fs::create_dir(&ext_dir_path) {
                    return Err(format!(
                        "Error: Nested source destination directory {} could not be created.\n{}",
                        ext_dir_path.display(),
                        e
                    ));
                }
            }
            scan_rec(&dir, &ext_dir_path, setting)?;
        } else {
            // file
            let file = next_dir_or_file; // for better reading...
                                         // Test if file matches suffix:
            if let Some(filename) = file.to_str() {
                // Check if file ends with provided suffixes:
                let file_match = setting.file_suffix.iter().any(|s| filename.ends_with(s));
                if file_match {
                    // Process file. Check if source files are modified:
                    if test_if_modified(file.as_path(), &src_dest_dir, setting) {
                        info!(" {}", file.display());
                        parse_write(file.as_path(), &src_dest_dir, setting)?;
                    } else {
                        debug!(" {} not modified", file.display());
                    }
                } else {
                    // TODO skip or just copy...?
                    debug!(" skipped {}", file.display());
                }
            }
        }
    }
    Ok(())
}

pub fn parse_write(filepath: &Path, src_dest_dir: &Path, setting: &Setting) -> Result<(), String> {
    // Make vector of the lines in the text file:
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(&file);
    let v: Vec<String> = reader.lines().map(|e| e.unwrap()).collect();
    let lines: Vec<&str> = v.iter().map(|s| s as &str).collect();

    // Parse the content of the file:
    let coll = parse(&lines, setting)?;
    write_files(filepath, src_dest_dir, &coll, setting);
    Ok(())
}

/// Test if the file to be processed (represented by `filepath`) is modified,
/// i.e. newer than the file(s) being created. The time stamp of the files is compared.
/// `src_dest_path` is the path to the current source destination folder.
/// `setting` contains, among other things, the path to the snippets folder.
/// Returns true if time stamp of file is newer than
/// processed file or if the processed file
/// does not exist yet. Returns false if not.
fn test_if_modified(filepath: &Path, src_dest_path: &Path, setting: &Setting) -> bool {
    if setting.force_update {
        return true; // Update always.
    }

    catch! {
        try {
            // Time stamp (in ms) when source file was last modified:
            let src_mod_time = fs::metadata(filepath)?.modified()?;
            // Target files:
            let filename = filepath.file_name().unwrap();
            let snippet_filepath = setting.snippet_dest_dir.join(filename);
            let snippet_file = fs::metadata(snippet_filepath);
            let src_dest_filepath = src_dest_path.join(filename);
            let src_dest_file = fs::metadata(src_dest_filepath);
            // Both, the snippet and the source file must exist:
            if snippet_file.is_ok() && src_dest_file.is_ok() {
                // Time stamp (in ms) when target file was last modified:
                let target_mod_time = snippet_file?.modified()?;
                src_mod_time > target_mod_time // file newer?
            } else {
                true // This file needs to be (re-)created.
            }
        }
        catch err {
            warn!("Internal error: checking file modification failed: {}", err);
            true // Update file anyway then.
        }
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    // There are no unit tests for lib. They are in ./tests
}
