use std::path::Path;

/// The settings for a snippet run.
pub struct Setting<'a> {
    pub src_dir: &'a Path,
    pub snippet_dest_dir: &'a Path,
    pub src_dest_dir: &'a Path,
    pub file_suffix: &'a str,
    pub comment: &'a str,
    pub comment_alternative: &'a str,
    pub exercise_solution: bool,
}
