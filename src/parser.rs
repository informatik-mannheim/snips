// Parser

use crate::util::Setting;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const DEFAULTLABEL: &str = "x8gfz4hd"; // just a crazy string.

pub fn parse_write(filepath: &Path, setting: &Setting) -> Result<(), String> {
    // Make vector with lines in text file:
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|e| e.unwrap()).collect();

    // Parse the content of the file:
    let coll = parse(&lines, setting)?;
    write_files(filepath, &coll, setting);
    Ok(())
}

pub fn parse(lines: &Vec<String>, setting: &Setting) -> Result<HashMap<String, Record>, String> {
    let no_lines = lines.len(); // of the the source file

    // Output collector.
    // Key: a label taken from the annotated source code.
    // Values: text buffer to output.
    let mut coll: HashMap<String, Record> = HashMap::new();

    let mut quiet = false; // if true, lines are omitted.
    let mut exercise_quiet = false; // if true, lines are not omitted.
    // TODO
    // This is the default snippet for extracting the whole source code.
    start(DEFAULTLABEL.to_string(), &mut coll);

    // Process line by line...
    for (counter, line) in lines.iter().enumerate() {
        // Show the line and its number.
        // println!("{}. {}", counter + 1, line);

        let _ = match test_token(&line, &setting) {
            // see if this line is a token.
            Some(Token::RegularToken { label, start: true }) => {
                println!("  + {}", label); // begin of a code snippet.
                                           // label is moved into hash map:
                start(label.to_string(), &mut coll);
                if coll.get(&label).unwrap().counter <= 1 && counter > 1 {
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
                println!("  - {}", label); // end of a code snippet.
                end(label.to_string(), &mut coll);
                if counter < no_lines {
                    // Print ... but not at the end of the file.
                    coll.get_mut(&label).unwrap().buffer.push_str("...\n");
                }
                ()
            }
            Some(Token::QuietToken { start: true }) => {
                quiet = true; // start to omit output.
                ()
            }
            Some(Token::QuietToken { start: false }) => {
                quiet = false; // end omitting output.
                ()
            }

            Some(Token::ExerciseToken { start: true }) => {
                println!("  +EXC");
                exercise_quiet = true; // start to omit output in exercise mod.
                ()
            }
            Some(Token::ExerciseToken { start: false }) => {
                println!("  -EXC");
                exercise_quiet = false; // end omitting output in exercise mod.
                ()
            }
            Some(Token::ReplaceToken {
                s: text,
                start: true,
            }) => {
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
                quiet = false; // end marker, output is allowed again.
                ()
            }
            Some(Token::ExerciseReplaceToken {
                s: text,
                start: true,
            }) => {
                println!("  +EXCSUBST");
                if !setting.exercise_solution && !quiet {
                    for r in coll.values_mut() {
                        if r.active {
                            r.buffer.push_str(&text);
                            r.buffer.push('\n');
                        }
                    }
                    quiet = true; // prevents to output next line.
                    ()
                }
            }
            Some(Token::ExerciseReplaceToken { s: _, start: false }) => {
                println!("  -EXCSUBST");
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

fn is_comment_escape(text: &str, setting: &Setting) -> bool {
    if setting.comment_alternative == "" {
        text == setting.comment // only one escape comment
    } else {
        text == setting.comment || text == setting.comment_alternative
    }
}

fn test_token<'a>(line: &'a str, setting: &'a Setting) -> Option<Token> {
    // let line = line.clone();
    let tokens: Vec<&str> = line.trim().split_whitespace().collect();

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
            return Some(Token::ExerciseReplaceToken {
                s: "".to_string(),
                start: false,
            });
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
        // TODO almost redundant to previous section.
        if is_comment_escape(tokens[0], setting) && tokens[1] == "+EXCSUBST" {
            let indent: i32 = tokens[2].parse().unwrap();
            // Truncate first three tokens and fill them with spaces:
            let c: Vec<char> = (1..indent).into_iter().map(|_| ' ').collect();
            let mut spaces = String::from_iter(c);
            // Find rest of line:
            let p = format!("{} {}", tokens[1], tokens[2]);
            let idx = line.find(&p).unwrap() + p.chars().count();
            spaces.push_str(&line[idx..]); // Add rest of line.
            return Some(Token::ExerciseReplaceToken {
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

fn write_files(filepath: &Path, coll: &HashMap<String, Record>, setting: &Setting) {
    for (label, record) in &*coll {
        // println!("\nFile {}", label);
        // println!("{}", record.buffer);

        let filename = filepath.file_stem().unwrap();
        let suffix = filepath.extension().unwrap();

        if label == DEFAULTLABEL {
            let mut file1 = PathBuf::new();
            file1.push(setting.snippet_dest_dir);
            file1.push(filepath.file_name().unwrap());
            write_file(&file1, record);
            // write_file(srcTargetDir + "/" + file.getName);
        } else {
            let mut path = PathBuf::new();
            path.push(setting.snippet_dest_dir);
            let mut ext_filename = String::new();
            ext_filename.push_str(&filename.to_str().unwrap());
            ext_filename.push_str("_");
            ext_filename.push_str(&label);
            ext_filename.push_str(".");
            ext_filename.push_str(suffix.to_str().unwrap());
            path.push(&ext_filename);
            write_file(&path, record);
        }
    }
}

fn write_file(filepath: &Path, record: &Record) {
    std::fs::write(filepath, record.buffer.as_str()).expect("Unable to write file");
}

/// @param active  true if lines are printed.
/// @param counter number of code snippets (until now)
/// @param buffer  buffer to collect the output text.
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

/// `label`: name of the token, `start`: start or end?,
/// `s`: next line is replaced with this text
enum Token {
    RegularToken { label: String, start: bool },     // +/-IN
    QuietToken { start: bool },                      // +/-OUT
    ExerciseToken { start: bool },                   // +/-EXC
    ReplaceToken { s: String, start: bool },         // +/-VAR
    ExerciseReplaceToken { s: String, start: bool }, // +/-EXCSUBST
}

// Unit tests

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::parser::DEFAULTLABEL;
    use indoc::indoc;

    // use super::super::scan;
    use super::super::util::Setting;
    use std::path::Path;

    fn str_to_vec(s: &str) -> Vec<String> {
        let lines = s.split("\n");
        let mut v = Vec::new();
        for z in lines {
            v.push(z.to_string());
        }
        v
    }

    fn config() -> Setting<'static> {
        // Path is relative to project root.
        Setting {
            src_dir: Path::new("tests/testfiles/src"),
            snippet_dest_dir: Path::new("tests/testfiles/snippets"),
            src_dest_dir: Path::new("tests/testfiles/src_dest"),
            file_suffix: ".java",
            comment: "//",
            comment_alternative: "#",
            exercise_solution: false,
        }
    }

    #[test]
    fn test_slide() {
        let s = indoc! {"
        public class Foo {
            public static void main(String[] args) {
                // +IN Slide
                int a = 1;
                // -IN Slide
                System.out.println(\"Value is \" + a);
            }
        }
        "};
        let ok = "...\n        int a = 1;\n...\n";
        let lines = str_to_vec(s);
        let coll = parse(&lines, &config()).unwrap();
        let test = coll.get("Slide").unwrap().buffer.as_str();
        assert_eq!(coll.len(), 2);
        assert_eq!(test, ok);
    }

    #[test]
    fn test_excsubst() {
        let s = indoc! {"
        public class Foo {
          public static void main(String[] args) {
            int a = 1;
            // +EXCSUBST 4 // Your solution:
            // This is the solution:
            System.out.println(\"Value is \" + a);
            // -EXCSUBST
          }
        }
        "};
        let ok = indoc! {"
        public class Foo {
          public static void main(String[] args) {
            int a = 1;
            // Your solution:
          }
        }

        "};
        // test produces an extra line, therefore this extra line.
        let lines = str_to_vec(s);
        let coll = parse(&lines, &config()).unwrap();
        assert_eq!(coll.len(), 1);
        let test = coll.get(DEFAULTLABEL).unwrap().buffer.as_str();
        assert_eq!(test, ok);
    }

    #[test]
    fn test_excsubst_solution() {
        let mut config = config();
        config.exercise_solution = true;
        let s = indoc! {"
        public class Foo {
          public static void main(String[] args) {
            int a = 1;
            // +EXCSUBST 4 // Your solution:
            // This is the solution:
            System.out.println(\"Value is \" + a);
            // -EXCSUBST
          }
        }
        "};
        let ok = indoc! {"
        public class Foo {
          public static void main(String[] args) {
            int a = 1;
            // This is the solution:
            System.out.println(\"Value is \" + a);
          }
        }

        "};
        // test produces an extra line, therefore this extra line.
        let lines = str_to_vec(s);
        let coll = parse(&lines, &config).unwrap();
        assert_eq!(coll.len(), 1);
        let test = coll.get(DEFAULTLABEL).unwrap().buffer.as_str();
        assert_eq!(test, ok);
    }

    #[test]
    fn test_parse() {
    }
}
