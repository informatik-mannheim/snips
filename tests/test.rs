// Integration tests

mod tests {

  use std::path::Path;
  use snips::scan;
  use snips::util::Setting;

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
      let setting = public_config();
      let r = scan(setting);
      assert_eq!(r, Ok(()));
  }

  /// Run a full test.
  #[test]
  fn scan_new_dirs_solution() {
      // TODO
      let setting = solution_config();
      // scan(setting);
      assert_eq!(true, true);
  }  
}