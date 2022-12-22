// Parser

mod tests; // Unit tests are in other file.

// Issues: none

use crate::util::Setting;
use crate::DEFAULTLABEL;
use log::{debug, trace};
use std::collections::HashMap;

/// Parse a vector of text lines (`lines`) and extract snippets.
/// The environment is specified in `setting`.
/// The snippets are returned in a hash map where the keys
/// are the snippet labels and the processed text file is contained
/// as the `Record` value.
pub fn parse(lines: &Vec<&str>, setting: &Setting) -> Result<HashMap<String, Record>, String> {
    let no_lines = lines.len(); // of the the source file
    let mut mode = Mode {
        out: false,
        exc: false,
        excsubst: false,
        var: false,
    };

    // Output collector.
    // Key: a label taken from the annotated source code.
    // Values: text buffer to output.
    let mut coll: HashMap<String, Record> = HashMap::new();

    let mut quiet = false; // if true, lines are omitted.
    let mut exercise_quiet = false; // if true, lines are not omitted.

    // This is the default snippet for extracting the whole source code.
    start(DEFAULTLABEL.to_string(), &mut coll);

    // Process line by line...
    for (counter, line) in lines.iter().enumerate() {
        let line_no = counter + 1; // counter starts at 0.

        // Show the line and its number.
        trace!("{}. {}", counter + 1, line);

        // Parse the next token:
        let _ = match read_token(&line, &setting) {
            // see if this line is a token.
            Some(Token::RegularToken { label, start: true }) => {
                debug!("  +IN {}", label); // begin of a code snippet.
                                           // label is moved into hash map:
                start(label.to_string(), &mut coll);
                if coll.get(&label).unwrap().counter <= 1 && line_no > 1 {
                    // Print ... but not at the beginning of the file
                    // or when ... was printed at the end of a code snippet.
                    coll.get_mut(&label).unwrap().buffer.push_str("...\n");
                }
                ()
            }
            Some(Token::RegularToken {
                label,
                start: false,
            }) => {
                debug!("  -IN {}", label); // end of a code snippet.
                end(label.to_string(), &mut coll)?;
                if line_no < no_lines {
                    // Print ... but not at the end of the file.
                    coll.get_mut(&label).unwrap().buffer.push_str("...\n");
                }
                ()
            }
            Some(Token::QuietToken { start: true }) => {
                if mode.out {
                    return Err(format!("Line {}: Another +OUT", line_no));
                }
                mode.out = true;
                quiet = true; // start to omit output.
                ()
            }
            Some(Token::QuietToken { start: false }) => {
                if !mode.out {
                    return Err(format!("Line {}: -OUT without preceding +OUT", line_no));
                }
                mode.out = false;
                quiet = false; // end omitting output.
                ()
            }
            Some(Token::ExerciseToken { start: true }) => {
                debug!("  +EXC");
                if mode.exc {
                    return Err(format!("Line {}: Another +EXC", line_no));
                }
                mode.exc = true;
                exercise_quiet = true; // start to omit output in exercise mod.
                ()
            }
            Some(Token::ExerciseToken { start: false }) => {
                debug!("  -EXC");
                if !mode.exc {
                    return Err(format!("Line {}: -EXC without preceding +EXC", line_no));
                }
                mode.exc = false;
                exercise_quiet = false; // end omitting output in exercise mod.
                ()
            }
            Some(Token::ReplaceToken {
                s: text,
                start: true,
            }) => {
                if mode.var {
                    return Err(format!("Line {}: Another +VAR", line_no));
                }
                mode.var = true;
                for r in coll.values_mut() {
                    // all records
                    if r.active {
                        r.buffer.push_str(&text);
                        r.buffer.push('\n');
                    }
                }
                quiet = true; // prevents to output next line.
                ()
            }
            Some(Token::ReplaceToken { s: _, start: false }) => {
                if !mode.var {
                    return Err(format!("Line {}: -VAR without preceding +VAR", line_no));
                }
                mode.var = false;
                quiet = false; // end marker, output is allowed again.
                ()
            }
            Some(Token::ExerciseReplaceToken {
                s: text,
                start: true,
            }) => {
                debug!("  +EXCSUBST");
                if mode.excsubst {
                    return Err(format!("Line {}: Another +EXCSUBST", line_no));
                }
                mode.excsubst = true;
                if !setting.exercise_solution && !quiet {
                    for r in coll.values_mut() {
                        if r.active {
                            r.buffer.push_str(&format!("{}\n", &text));
                        }
                    }
                    quiet = true; // prevents to output next line.
                    ()
                }
            }
            Some(Token::ExerciseReplaceToken { s: _, start: false }) => {
                debug!("  -EXCSUBST");
                if !mode.excsubst {
                    return Err(format!(
                        "Line {}: -EXCSUBST without preceding +EXCSUBST",
                        line_no
                    ));
                }
                mode.excsubst = false;
                quiet = false; // end marker, output is allowed again.
                ()
            }
            _ => {
                if !quiet && (setting.exercise_solution || !exercise_quiet) {
                    // omit lines when in quiet mode.
                    // Store line for every code snippet label...
                    for r in coll.values_mut() {
                        if r.active {
                            r.buffer.push_str(&line);
                            r.buffer.push('\n');
                        }
                    }
                }
                ()
            }
        };
    }
    end(DEFAULTLABEL.to_string(), &mut coll)?; // end default code snippet.
    Ok(coll)
}

/// Read the next token in the text file's `line`.
/// The environment is controlled by `setting`:
fn read_token<'a>(line: &'a str, setting: &'a Setting) -> Option<Token> {
    // Test if `text` is an escape comment according to
    // the settings as specified in `setting`.
    let is_comment_escape = |text: &str| {
        setting.comment.iter().any(|s| text == s)
    };

    // let line = line.clone();
    let tokens: Vec<&str> = line.trim().split_whitespace().collect();

    // Rest of line for EXC* tokens:
    let rest_of_line = || {
        let indent: i32 = tokens[2].parse().unwrap_or_default();
        // Truncate first three tokens and fill them with spaces:
        let mut spaces = if indent == 0 {
            String::new() // empty string
        } else {
            let c: Vec<char> = (1..indent).into_iter().map(|_| ' ').collect();
            String::from_iter(c)
        };
        // Find rest of line:
        let p = format!("{} {}", tokens[1], tokens[2]);
        let idx = line.find(&p).unwrap() + p.chars().count() + 1;
        spaces.push_str(&line[idx..]); // Add rest of line.
        spaces
    };

    if tokens.len() == 2 {
        if is_comment_escape(tokens[0]) && tokens[1] == "+OUT" {
            return Some(Token::QuietToken { start: true });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "-OUT" {
            return Some(Token::QuietToken { start: false });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "+EXC" {
            return Some(Token::ExerciseToken { start: true });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "-EXC" {
            return Some(Token::ExerciseToken { start: false });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "-EXCSUBST" {
            return Some(Token::ExerciseReplaceToken {
                s: "".to_string(),
                start: false,
            });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "-HEADER" {
            return Some(Token::ReplaceToken {
                s: "".to_string(),
                start: false,
            });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "-VAR" {
            return Some(Token::ReplaceToken {
                s: "".to_string(),
                start: false,
            });
        }
    }

    if tokens.len() >= 3 {
        if is_comment_escape(tokens[0]) && tokens[1] == "+IN" {
            return Some(Token::RegularToken {
                label: tokens[2].to_string(),
                start: true,
            });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "-IN" {
            return Some(Token::RegularToken {
                label: tokens[2].to_string(),
                start: false,
            });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "+HEADER" {
            let idx = tokens[0].len() + tokens[1].len(); // Truncate first letters.
            let s = line[idx..].to_string();
            return Some(Token::ReplaceToken { s: s, start: true });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "+VAR" {
            return Some(Token::ReplaceToken {
                s: rest_of_line(),
                start: true,
            });
        }
        if is_comment_escape(tokens[0]) && tokens[1] == "+EXCSUBST" {
            return Some(Token::ExerciseReplaceToken {
                s: rest_of_line(),
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
        Err(format!("End (-) without start (+) for label: {}", label))
    } else {
        coll.get_mut(&label).unwrap().active = false;
        Ok(())
    }
}

/// @param active  true if lines are printed.
/// @param counter number of code snippets (until now)
/// @param buffer  buffer to collect the output text.
#[derive(PartialEq, Debug)]
pub struct Record {
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

struct Mode {
    out: bool,
    var: bool,
    exc: bool,
    excsubst: bool,
}

/// `label`: name of the token, `start`: start or end?,
/// `s`: next line is replaced with this text
enum Token {
    RegularToken { label: String, start: bool },     // +/-IN
    QuietToken { start: bool },                      // +/-OUT
    ExerciseToken { start: bool },                   // +/-EXC
    ReplaceToken { s: String, start: bool },         // +/-VAR
    ExerciseReplaceToken { s: String, start: bool }, // +/-EXCSUBST
}
