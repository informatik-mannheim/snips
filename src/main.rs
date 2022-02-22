use clap::{Arg, Command};
use std::path::Path;
use snips::scan;
use snips::util::Setting;

fn main() {
    // https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
    let args = Command::new("snips")
        .author("(c) 2022 by Markus Gumbel")
        .version("0.1.0")
        .about("Collects snippets of text or source-code files.")
        .arg(
            Arg::new("src_dir")
                .short('s')
                .default_value(".")
                .help("Directory with source files."),
        )
        .arg(
            Arg::new("snippet_dest_dir")
                .short('t')
                .default_value("./snippets")
                .help("Directory where snippet files will be stored."),
        )
        .arg(
            Arg::new("src_dest_dir")
                .short('d')
                .default_value("./src_dest")
                .help("Directory where stripped source files will be stored."),
        )
        .arg(
            Arg::new("file_suffix")
                .short('x')
                .default_value(".txt")
                .help("File suffix of files to process."),
        )
        .arg(
            Arg::new("comment")
                .short('c')
                .default_value("#")
                .help("Escape comment symbol, e.g. # or //"),
        )
        .arg(
            Arg::new("alternative_comment")
                .short('a')
                .default_value("//")
                .help("Alternative escape comment symbol"),
        )
        .arg(
            Arg::new("exercise_solution")
                .short('e')
                .default_value("false")
                .help("Include solutions (EXC and EXCSUBST flags)"),
        )
        .after_help(
            "Extract parts (snippets) of source code or text in general \
                 and copy the stripped files. Useful for source code \
                 presentation or exercise.",
        )
        .get_matches();

    let setting = Setting {
        src_dir: Path::new(args.value_of("src_dir").unwrap_or_default()),
        snippet_dest_dir: Path::new(args.value_of("snippet_dest_dir").unwrap_or_default()),
        src_dest_dir: Path::new(args.value_of("src_dest_dir").unwrap_or_default()),
        file_suffix: args.value_of("file_suffix").unwrap_or_default(),
        comment: args.value_of("comment").unwrap_or_default(),
        comment_alternative: args.value_of("alternative_comment").unwrap_or_default(),
        exercise_solution: args
            .value_of("exercise_solution")
            .unwrap_or_default()
            .parse::<bool>()
            .unwrap(),
    };

    if let Err(e) = scan(setting) {
        println!("snips failed.");
        println!("{}", e);
    }
}
