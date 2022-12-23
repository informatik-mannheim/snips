use clap::Parser;
use log::*;
use snips::scan;
use snips::util::Setting;

fn main() {
    // https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
    let setting = Setting::parse();

    let verbose = setting.verbosity as usize;

    stderrlog::new()
        .module(module_path!())
        .quiet(false)
        .verbosity(verbose + 1) // Skip ERROR level
        // .timestamp(ts)
        .show_level(false) // no INFO or DEBUG prefixes
        .init()
        .unwrap();

    if let Err(e) = scan(&setting) {
        error!("Error: snips failed.");
        error!("{}", e);
    }
}
