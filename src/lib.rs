// lib
pub mod parser;
pub mod util;

use crate::parser::parse;
use crate::util::Setting;
use std::fs;
use std::io;
use std::path::Path;

pub fn scan(setting: Setting) {
    // val mode = if (exerciseEnv) "(mode EXC) " else ""

    let metadata = fs::metadata(&setting.src_dir);
    if let Err(e) = metadata {
        println!(
            "Error: Directory {} does not exist.\n{}",
            &setting.src_dir.display(),
            e
        );
        return;
    }
    if !setting.src_dir.is_dir() {
        println!("Error: {} is not a directory.", &setting.src_dir.display());
        return;
    }

    println!("Scanning...");
    let _r = scan_rec(&setting.src_dir, "", &setting); // no suffix path at beginning.
    println!("... done");
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
                    parse(path.as_path(), setting);
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

    use super::scan;
    use super::util::Setting;
    use std::path::Path;

    fn config() -> Setting<'static> {
        // Path is relative to project root.
        Setting {
            src_dir: Path::new("tests/testfiles/src"),
            snippet_target_dir: Path::new("tests/testfiles/snippets"),
            src_target_dir: Path::new("tests/testfiles/src_dest"),
            file_suffix: ".java",
            comment_escape: "//",
            comment_escape2: "#",
            exercise_env: false,
        }
    }

    #[test]
    fn it_works() {
        let setting = config();
        scan(setting);
        assert_eq!(true, true);
    }

    #[test]
    fn path_end_test() {
        let path = Path::new("foo/bar/file.java");
        let b = path.ends_with("file.java");
        assert_eq!(b, true);
    }
}
