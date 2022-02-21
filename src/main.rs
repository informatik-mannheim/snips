use clap::{Arg, Command};
use std::path::Path;
use snips::scan;
use snips::util::Setting;

fn main() {
    // https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
    let args = Command::new("snips")
        .author("Markus Gumbel")
        .version("0.1.0")
        .about("Collects snippets of text files.")
        .arg(
            Arg::new("src_dir")
                .short('s')
                .default_value(".")
                .help("Directory with source files."),
        )
        .arg(
            Arg::new("snippet_target_dir")
                .short('t')
                .default_value("./snippets")
                .help("Directory where snippet files will be stored."),
        )
        .arg(
            Arg::new("src_target_dir")
                .short('d')
                .default_value("./src_dest")
                .help("Directory where stripped source files will be stored."),
        )
        .arg(Arg::new("file_suffix").short('x').default_value(".txt"))
        .arg(
            Arg::new("comment_escape")
                .short('c')
                .default_value("#")
                .help("Escape comment symbol, e.g. # or //"),
        )
        .arg(
            Arg::new("comment_escape_2")
                .short('b')
                .default_value("//")
                .help("Alternative escape comment symbol"),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .get_matches();
    
    let setting = Setting {
        src_dir: Path::new(args.value_of("src_dir").unwrap_or_default()),
        snippet_target_dir: Path::new(args.value_of("snippet_target_dir").unwrap_or_default()),
        src_target_dir: Path::new(args.value_of("src_target_dir").unwrap_or_default()),
        file_suffix: args.value_of("file_suffix").unwrap_or_default(),
        comment_escape: args.value_of("comment_escape").unwrap_or_default(),
        comment_escape2: args.value_of("comment_escape_2").unwrap_or_default(),
        exercise_env: false,
    };

    scan(setting);
}
