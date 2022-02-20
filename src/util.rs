use std::path::Path;

/// The settings for a snippet run.
pub struct Setting<'a> {
    pub src_dir: &'a Path,
    pub snippet_target_dir: &'a Path,
    pub src_target_dir: &'a Path,
    pub file_suffix: String,
    pub comment_escape: String,
    pub comment_escape2: String,
    pub exercise_env: bool,
}
