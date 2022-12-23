// Module file

use crate::parser::Record;
use crate::util::Setting;
use crate::DEFAULTLABEL;
use log::{trace, warn};
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::fs::copy;
use std::path::Path;
use try_catch::catch;

/// Write all snippets collected in `coll` to their files.
/// `filepath` is the file to write.
/// `dir_path` is the current directory for the (nested)
/// source files.
/// The environment is controlled by `setting`.
pub fn write_files(
    filepath: &Path,
    dir_path: &Path,
    coll: &HashMap<String, Record>,
    setting: &Setting,
) {
    for (label, record) in &*coll {
        // println!("\nFile {}", label);
        // println!("{}", record.buffer);

        let filename = filepath.file_name().unwrap();
        let filestem = filepath.file_stem().unwrap();
        let suffix = filepath.extension().unwrap();

        if label == DEFAULTLABEL {
            // Write snippet:
            let snippet_file = setting.snippet_dest_dir.join(filename);
            write_file(&snippet_file, record);
            // Also write full file to src dest:
            let full_file = dir_path.join(filename);
            write_file(&full_file, record);
        } else {
            // Insert label into file's name:
            let ext_filename = format!(
                "{}_{}.{}",
                filestem.to_str().unwrap(),
                label,
                suffix.to_str().unwrap()
            );
            trace!("Write file: {}", ext_filename);
            let file = setting.snippet_dest_dir.join(ext_filename);
            write_file(&file, record);
        }
    }
}

fn write_file(filepath: &Path, record: &Record) {
    fs::write(filepath, record.buffer.as_str()).expect("Unable to write file");
}

/// Test if the file to be processed (represented by `filepath`) is modified,
/// i.e. newer than the file(s) being created. The time stamp of the files is compared.
/// `src_dest_path` is the path to the current source destination folder.
/// `setting` contains, among other things, the path to the snippets folder.
/// Returns true if time stamp of file is newer than
/// processed file or if the processed file
/// does not exist yet. Returns false if not.
pub fn test_if_modified(filepath: &Path, src_dest_path: &Path, setting: &Setting) -> bool {
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

pub fn copy_file(filepath: &Path, src_dest_path: &Path, _setting: &Setting) -> Result<u64> {
    let dest = src_dest_path.join(filepath.file_name().unwrap());
    copy(filepath, dest)?;
    Ok(0)
}
