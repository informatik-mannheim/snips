// Unit tests for parser

#![cfg(test)]
use crate::parser::parse;
use crate::parser::DEFAULTLABEL;
use crate::util::Setting;
use indoc::indoc;
use std::path::PathBuf;

fn str_to_vec(s: &str) -> Vec<&str> {
    s.split("\n").collect()
}

fn config_public() -> Setting {
    // Path is relative to project root.
    Setting {
        src_dir: PathBuf::from("tests/testfiles/src"),
        snippet_dest_dir: PathBuf::from("tests/testfiles/snippets"),
        src_dest_dir: PathBuf::from("tests/testfiles/src_dest"),
        file_suffix: vec![".java".to_string()],
        comment: vec!["//".to_string()],
        exercise_solution: false,
        force_update: true,
        verbosity: 0,
    }
}

fn config_solution() -> Setting {
    // Path is relative to project root.
    Setting {
        src_dir: PathBuf::from("tests/testfiles/src"),
        snippet_dest_dir: PathBuf::from("tests/testfiles/snippets"),
        src_dest_dir: PathBuf::from("tests/testfiles/src_dest"),
        file_suffix: vec![".java".to_string()],
        comment: vec!["//".to_string()],
        exercise_solution: true,
        force_update: true,
        verbosity: 0,
    }
}

#[test]
fn test_slide() {
    let s = indoc! {"
        line 1
        // +IN Slide
          line 3
        // -IN Slide
        line 5
        "};
    let ok = "...\n  line 3\n...\n";
    let lines = str_to_vec(s);
    let coll = parse(&lines, &config_public()).unwrap();
    let test = coll.get("Slide").unwrap().buffer.as_str();
    assert_eq!(coll.len(), 2);
    assert_eq!(test, ok);
}

#[test]
fn excsubst() {
    let s = indoc! {"
        line 1
        // +EXCSUBST 0 // line hint
        line 3
        // -EXCSUBST
        line 5
        "};
    let ok = indoc! {"
        line 1
        // line hint
        line 5

        "};
    // test produces an extra line, therefore this extra line.
    let lines = str_to_vec(s);
    let coll = parse(&lines, &config_public()).unwrap();
    assert_eq!(coll.len(), 1);
    let test = coll.get(DEFAULTLABEL).unwrap().buffer.as_str();
    assert_eq!(test, ok);
}

#[test]
fn excsubst_solution() {
    let s = indoc! {"
        line 1
        // +EXCSUBST 0 // line hint
        line solution
        // -EXCSUBST
        line 5
        "};
    let ok = indoc! {"
        line 1
        line solution
        line 5

        "};
    // test produces an extra line, therefore this extra line.
    let lines = str_to_vec(s);
    let coll = parse(&lines, &config_solution()).unwrap();
    assert_eq!(coll.len(), 1);
    let test = coll.get(DEFAULTLABEL).unwrap().buffer.as_str();
    assert_eq!(test, ok);
}

#[test]
fn unbalanced_out_on() {
    let s = indoc! {"
        line 1
        // +OUT
        line 3
        // +OUT
        line 5
        "};
    let lines = str_to_vec(s);
    let r = parse(&lines, &config_public());
    assert_eq!(r, Err("Line 4: Another +OUT".to_string()));
    // assert_eq!(r, Ok(_);
}

#[test]
fn unbalanced_out_off() {
    let s = indoc! {"
        line 1
        // -OUT
        line 3
        "};
    let lines = str_to_vec(s);
    let r = parse(&lines, &config_public());
    assert_eq!(r, Err("Line 2: -OUT without preceding +OUT".to_string()));
}

#[test]
fn unbalanced_exc_on() {
    let s = indoc! {"
        line 1
        // +EXC
        line 3
        // +EXC
        line 5
        "};
    let lines = str_to_vec(s);
    let r = parse(&lines, &config_public());
    assert_eq!(r, Err("Line 4: Another +EXC".to_string()));
}

#[test]
fn unbalanced_exc_off() {
    let s = indoc! {"
        line 1
        // -EXC
        line 3
        "};
    let lines = str_to_vec(s);
    let r = parse(&lines, &config_public());
    assert_eq!(r, Err("Line 2: -EXC without preceding +EXC".to_string()));
    // assert_eq!(r, Ok(_);
}

#[test]
fn unbalanced_excsubst_on() {
    let s = indoc! {"
        line 1
        // +EXCSUBST 0 hint
        line 3
        // +EXCSUBST 0 hint
        line 5
        "};
    let lines = str_to_vec(s);
    let r = parse(&lines, &config_public());
    assert_eq!(r, Err("Line 4: Another +EXCSUBST".to_string()));
}

#[test]
fn unbalanced_excsubst_off() {
    let s = indoc! {"
        line 1
        // -EXCSUBST
        line 3
        "};
    let lines = str_to_vec(s);
    let r = parse(&lines, &config_public());
    assert_eq!(
        r,
        Err("Line 2: -EXCSUBST without preceding +EXCSUBST".to_string())
    );
}

#[test]
fn indented_label() {
    let s = indoc! {"
            line 1
              // +EXCSUBST 0 // line hint
            line solution
            // -EXCSUBST
            line 5
            "};
    let ok = indoc! {"
            line 1
            // line hint
            line 5
    
            "};
    // test produces an extra line, therefore this extra line.
    let lines = str_to_vec(s);
    let coll = parse(&lines, &config_public()).unwrap();
    assert_eq!(coll.len(), 1);
    let test = coll.get(DEFAULTLABEL).unwrap().buffer.as_str();
    assert_eq!(test, ok);
}

/// The OUT-label cuts off inner IN-labels.
#[test]
fn nested_out_in() {
    let s = indoc! {"
            line 1
            // +OUT
            line 3
            // +IN foo
            line 5
            // -IN foo
            line 7
            // -OUT
            "};
    let ok = indoc! {"
            line 1
    
            "};
    // test produces an extra line, therefore this extra line.
    let lines = str_to_vec(s);
    let coll = parse(&lines, &config_public()).unwrap();
    assert_eq!(coll.len(), 2);
    let test = coll.get(DEFAULTLABEL).unwrap().buffer.as_str();
    assert_eq!(test, ok);
}
