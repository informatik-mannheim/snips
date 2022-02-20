// Parser

use crate::util::Setting;
use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const DEFAULTLABEL: &str = "x8gfz4hd"; // just a crazy string.

pub fn parse(filepath: &Path, setting: &Setting) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    // Output collector.
    // Key: a label taken from the annotated source code.
    // Values: text buffer to output.
    let mut coll: HashMap<String, Record> = HashMap::new();

    let mut _quiet = false; // if true, lines are omitted.
    let mut _exercise_quiet = false; // if true, lines are not omitted.
    let mut counter = 0;
    // line counter
    // let no_lines = reader.lines().size_hint().0; // of the the source file

    // This is the default snippet extracting the whole source code.
    start(DEFAULTLABEL.to_string(), &mut coll);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.

        // Show the line and its number.
        println!("{}. {}", index + 1, line);

        counter += 1;
        let _x = match test_token(&line, &setting) {
            // see if this line is a token.
            Some(Token::RegularToken { label, start: true }) => {
                println!(" + {}", label); // begin of a code snippet.
                                          // label is moved into hash map:
                start(label.to_string(), &mut coll);
                if coll.get(&label).unwrap().counter <= 1 && counter > 1 {
                    // Print ... but not at the beginning of the file
                    // or when ... was printed at the end of a code snippet.
                    coll.get_mut(&label).unwrap().buffer.push_str("...\n");
                }
                ()
            }
            _ => (),
        };
    }

    end(DEFAULTLABEL.to_string(), &mut coll); // end default code snippet.
    write_files();
}

fn is_comment_escape(text: &str, setting: &Setting) -> bool {
    if setting.comment_escape2 == "" {
        text == setting.comment_escape // only one escape comment
    } else {
        text == setting.comment_escape || text == setting.comment_escape2
    }
}

fn test_token<'a>(line: &'a str, setting: &'a Setting) -> Option<Token> {
    // let line = line.clone();
    let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.len() == 2 {
        if is_comment_escape(tokens[0], setting) && tokens[1] == "+OUT" {
            return Some(Token::QuietToken { start: true });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "-OUT" {
            return Some(Token::QuietToken { start: false });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "+EXC" {
            return Some(Token::ExerciseToken { start: true });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "-EXC" {
            return Some(Token::ExerciseToken { start: false });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "-EXCSUBST" {
            return Some(Token::ExerciseToken { start: false });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "-HEADER" {
            return Some(Token::ReplaceToken {
                s: "".to_string(),
                start: false,
            });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "-VAR" {
            return Some(Token::ReplaceToken {
                s: "".to_string(),
                start: false,
            });
        }
    }

    if tokens.len() >= 3 {
        if is_comment_escape(tokens[0], setting) && tokens[1] == "+IN" {
            return Some(Token::RegularToken {
                label: tokens[2].to_string(),
                start: true,
            });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "-IN" {
            return Some(Token::RegularToken {
                label: tokens[2].to_string(),
                start: false,
            });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "+HEADER" {
            let idx = tokens[0].len() + tokens[1].len(); // Truncate first letters.
            let s = line[idx..].to_string();
            return Some(Token::ReplaceToken { s: s, start: true });
        }
        if is_comment_escape(tokens[0], setting) && tokens[1] == "+VAR" {
            let indent: i32 = tokens[2].parse().unwrap();
            // Truncate first three tokens and fill them with spaces:
            let c: Vec<char> = (1..indent).into_iter().map(|_| ' ').collect();
            let mut spaces = String::from_iter(c);
            let idx = tokens[0].len() + tokens[1].len() + tokens[2].len();
            let t = &line[idx..];
            spaces.push_str(t);
            return Some(Token::ReplaceToken {
                s: spaces,
                start: true,
            });
        }
    }
    None
}

fn start(label: String, coll: &mut HashMap<String, Record>) {
    if !coll.contains_key(&label) {
        coll.insert(label, Record::new(true));
    } else {
        coll.get_mut(&label).unwrap().active = true;
        coll.get_mut(&label).unwrap().counter += 1;
    }
}

fn end(label: String, coll: &mut HashMap<String, Record>) -> Result<(), String> {
    if !coll.contains_key(&label) {
        Result::Err("End without start.".to_string())
    } else {
        coll.get_mut(&label).unwrap().active = false;
        Result::Ok(())
    }
}

fn write_files() {}

/// @param active  true if lines are printed.
/// @param counter number of code snippets (until now)
/// @param buffer  buffer to collect the output text.
struct Record {
    pub active: bool,
    pub counter: i32,
    pub buffer: String,
}

impl Record {
    fn new(active: bool) -> Self {
        Record {
            active: active,
            counter: 0,
            buffer: String::new(),
        }
    }
}

/// `label`: name of the token, `start`: start or end?,
/// `s`: next line is replaced with this text
enum Token {
    RegularToken { label: String, start: bool },     // +/-IN
    QuietToken { start: bool },                      // +/-OUT
    ExerciseToken { start: bool },                   // +/-EXC
    ReplaceToken { s: String, start: bool },         //
    ExerciseReplaceToken { s: String, start: bool }, // +/-EXCSUBST
}

// Unit tests

#[cfg(test)]
mod tests {

    use crate::parser::parse;

    // use super::super::scan;
    use super::super::util::Setting;
    use std::path::Path;

    fn config() -> Setting<'static> {
        // Path is relative to project root.
        Setting {
            src_dir: Path::new("tests/testfiles/src"),
            snippet_target_dir: Path::new("tests/testfiles/snippets"),
            src_target_dir: Path::new("tests/testfiles/src_dest"),
            file_suffix: ".java",
            comment_escape: "//",
            comment_escape2: "#",
            exercise_env: false,
        }
    }

    #[test]
    fn it_works() {
        use std::path::{Path, PathBuf};

        let setting = config();
        let mut path = PathBuf::new();
        path.push(setting.src_dir);
        path.push("Testfile_IN_Slide.java");
        parse(&path.as_path(), &setting);
        assert_eq!(true, true);
    }
}
