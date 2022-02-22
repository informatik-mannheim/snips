// Integration tests

mod util {
    use std::path::Path;

    /// Compare two text files.
    /// Important: Even the newlines (Windows vs. Unix) have to be
    /// identical.
    pub fn compare_files(filepath1: &Path, filepath2: &Path) -> bool {
        use file_diff::diff_files;
        use std::fs::File;

        let mut file1 = File::open(filepath1).expect("file 1");
        let mut file2 = File::open(filepath2).expect("file 2");
        diff_files(&mut file1, &mut file2)
    }
}

mod tests {

    use super::util::compare_files;
    use snips::scan;
    use snips::util::Setting;
    use std::path::Path;

    fn src_templ() -> &'static Path {
        Path::new("./tests/testfiles/template")
    }

    fn public_files() -> Vec<(&'static str, &'static str)> {
        // generated file; correct file
        vec![
            ("Testfile_IN_Slide.java", "Testfile_IN_Slide-public.java"),
            (
                "Testfile_IN_Slide_Slide.java",
                "Testfile_IN_Slide_Slide-public.java",
            ),
        ]
    }

    fn check_files(file_pairs: &Vec<(&str, &str)>, s: &Setting) -> bool {
        for (f1, f2) in file_pairs {
            let t = compare_files(&s.snippet_dest_dir.join(f1), &src_templ().join(f2));
            if !t {
                println!("{} vs: {}", f1, f2);
                return false;
            }
        }
        true
    }

    fn public_config() -> Setting<'static> {
        // Path is relative to project root.
        Setting {
            src_dir: Path::new("tests/testfiles/src"),
            snippet_dest_dir: Path::new("tests/testfiles/public/snippets"),
            src_dest_dir: Path::new("tests/testfiles/public/src_dest"),
            file_suffix: ".java",
            comment: "//",
            comment_alternative: "#",
            exercise_solution: false,
        }
    }

    fn solution_config() -> Setting<'static> {
        // Path is relative to project root.
        Setting {
            src_dir: Path::new("tests/testfiles/src"),
            snippet_dest_dir: Path::new("tests/testfiles/solution/snippets"),
            src_dest_dir: Path::new("tests/testfiles/solution/src_dest"),
            file_suffix: ".java",
            comment: "//",
            comment_alternative: "#",
            exercise_solution: true,
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
