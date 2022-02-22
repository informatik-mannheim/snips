// Module file

use crate::parser::Record;
use crate::util::Setting;
use crate::DEFAULTLABEL;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use log::{trace};

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
