// Integration tests

mod tests {

    use file_diff::diff_files;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use snips::scan;
    use snips::util::Setting;

    // Here a the correct files:
    fn src_templ() -> &'static Path {
        Path::new("./tests/testfiles/template")
    }

    // All file pairs for public generated snippet files:
    fn public_files() -> Vec<(PathBuf, PathBuf)> {
        let gen = public_config().snippet_dest_dir;
        let corr = src_templ();
        // generated file; correct file
        vec![
            (
                gen.join("Testfile_IN_Slide.java"),
                corr.join("Testfile_IN_Slide-public.java"),
            ),
            (
                gen.join("Testfile_IN_Slide_Slide.java"),
                corr.join("Testfile_IN_Slide_Slide-public.java"),
            ),
        ]
    }

    // Compare all files for equality.
    // Important: event the newline representation (Windows vs. Unix)
    // must be equal.
    fn check_files(file_pairs: &Vec<(PathBuf, PathBuf)>, _s: &Setting) -> bool {
        for (filepath1, filepath2) in file_pairs {
            let mut file1 = File::open(filepath1).expect("f1");
            let mut file2 = File::open(filepath2).expect("f1");
            let t = diff_files(&mut file1, &mut file2);
            if !t {
                println!("{} vs: {}", filepath1.display(), filepath2.display());
                return false;
            }
        }
        true
    }

    fn public_config() -> Setting {
        // Path is relative to project root.
        Setting {
            src_dir: PathBuf::from("tests/testfiles/src"),
            snippet_dest_dir: PathBuf::from("tests/testfiles/public/snippets"),
            src_dest_dir: PathBuf::from("tests/testfiles/public/src_dest"),
            file_suffix: vec![".java".to_string()],
            comment: "//".to_string(),
            comment_alternative: "#".to_string(),
            exercise_solution: false,
            verbosity: 0
        }
    }

    fn solution_config() -> Setting {
        // Path is relative to project root.
        Setting {
            src_dir: PathBuf::from("tests/testfiles/src"),
            snippet_dest_dir: PathBuf::from("tests/testfiles/solution/snippets"),
            src_dest_dir: PathBuf::from("tests/testfiles/solution/src_dest"),
            file_suffix: vec![".java".to_string()],
            comment: "//".to_string(),
            comment_alternative: "#".to_string(),
            exercise_solution: true,
            verbosity: 0
        }
    }

    /// Run a full test.
    #[test]
    fn scan_new_dirs_public() {
        let s = &public_config();
        let r = scan(s);
        assert_eq!(r, Ok(()));
        assert_eq!(check_files(&public_files(), s), true);
    }

    /// Run a full test.
    #[test]
    fn scan_new_dirs_solution() {
        // TODO
        let _setting = solution_config();
        // scan(setting);
        assert_eq!(true, true);
    }
}
