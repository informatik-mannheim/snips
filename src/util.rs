use clap::Parser;
use std::path::PathBuf;

/// The settings for a snippet run.
#[derive(Parser)]
#[command(name = "snips")]
#[command(version = "0.2.4")]
#[command(author = "Markus Gumbel")]
#[command(about = "Extract parts (snippets) of source code or text in general 
and copy the stripped files. Useful for source code \
presentation or exercises.", long_about = None)]
pub struct Setting {
    /// Directory with source files.
    #[arg(short = 's', long, value_name = "directory")]
    pub src_dir: PathBuf,

    /// Directory where snippet files will be stored.
    #[arg(
        short = 't',
        long,
        value_name = "directory",
        default_value = "./snippets"
    )]
    pub snippet_dest_dir: PathBuf,

    /// Directory where stripped source files will be stored.
    #[arg(
        short = 'd',
        long,
        value_name = "directory",
        default_value = "./src_dest"
    )]
    pub src_dest_dir: PathBuf,

    /// One or more file suffixes of files to process.
    #[arg(short = 'x', long, value_name = "suffix", default_value = ".txt")]
    pub file_suffix: Vec<String>,

    /// One or more escape comment symbols, e.g. # or //.
    #[arg(short = 'c', long, value_name = "comment", default_value = "#")]
    pub comment: Vec<String>,

    /// Include solutions (EXC and EXCSUBST flags).
    #[arg(short = 'e', long)]
    pub exercise_solution: bool,

    /// Force (re-)generation of all snippets.
    #[arg(short = 'f', long)]
    pub force_update: bool,

    /// Copy also all other files to src_dest_dir.
    #[arg(short = 'o', long)]
    pub copy_other_files: bool,

    /// Add this flag multiple times to increase message verbosity.
    #[arg(short= 'v', long, action = clap::ArgAction::Count)]
    pub verbosity: u8,
}
