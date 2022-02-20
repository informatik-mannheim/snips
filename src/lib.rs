// lib

pub mod util;

use crate::util::Setting;
use std::fs;
use std::io;
use std::path::Path;

pub fn scan(setting: Setting) {
    // val mode = if (exerciseEnv) "(mode EXC) " else ""
    println!("Scanning {}", setting.src_dir.display());

    let metadata = fs::metadata(&setting.src_dir);
    if let Err(e) = metadata {
        println!(
            "Error: {} is not a directory. {}",
            &setting.src_dir.display(),
            e
        );
        return;
    }

    scan_rec(&setting.src_dir, "", &setting); // no suffix path at beginning.
    println!(" ... done");
}

fn scan_rec(dir: &Path, suffix_path: &str, setting: &Setting) -> io::Result<()> {
    println!(" {}", dir.display());
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                scan_rec(&path, suffix_path, setting)?;
            } else {
                // cb(&entry);
            }
        }
    }
    Ok(())
}

// def scanRec(dir: File, suffixPath: String) {

//   def createDirIfNotExists(newDir: String) {
//     val dirFile = new File(newDir) // Access to file.
//     if (!(dirFile.exists() && dirFile.isDirectory)) {
//       dirFile.mkdir() // Directory did not exist, create it.
//     }
//   }

//   val fullTargetDirPackage = srcTargetDir + suffixPath
//   val allFiles = dir.listFiles().filter(f => !f.isDirectory && f.getName.endsWith(suffix))
//   for (file <- allFiles) {
//     new ExtractCodeSnippet(file, commentEscape, commentEscape2, snippetTargetDir,
//       fullTargetDirPackage, exerciseEnv)
//   }
//   val dirs = dir.listFiles().filter(_.isDirectory)
//   for (dir <- dirs) {
//     // Append this directory to the suffix path:
//     val newSuffixPath = suffixPath + "/" + dir.getName
//     createDirIfNotExists(srcTargetDir + newSuffixPath)
//     scanRec(dir, newSuffixPath)
//   }
// }
