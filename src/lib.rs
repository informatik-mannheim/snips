// lib
pub mod parser;
pub mod util;

use crate::parser::parse_write;
use crate::util::Setting;
use std::fs;
use std::io;
use std::path::Path;

pub fn scan(setting: Setting) -> Result<(), String> {
    // val mode = if (exerciseEnv) "(mode EXC) " else ""

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
        println!(
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
        println!(
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

    println!("Scanning...");
    let _r = scan_rec(&setting.src_dir, "", &setting); // no suffix path at beginning.
    println!("... done");
    Ok(())
}

fn scan_rec(dir: &Path, suffix_path: &str, setting: &Setting) -> io::Result<()> {
    println!(" {}", dir.display());

    // Recursively scan other directories:
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            scan_rec(&path, suffix_path, setting)?;
        } else {
            // TODO consider path.extension()
            // Test if file matches suffix:
            if let Some(filename) = path.to_str() {
                if filename.ends_with(setting.file_suffix) {
                    // Process file:
                    println!(" {}", path.display());
                    parse_write(path.as_path(), setting);
                } else {
                    println!(" skipped {}", path.display());
                }
            }
        }
    }
    Ok(())
}

// Unit tests

#[cfg(test)]
mod tests {
    // There are no unit tests for lib. They are in ./tests
}
