use syntect::{
    easy::ScopeRegionIterator,
    parsing::{ParseState, ScopeStack, ScopeStackOp, SyntaxSet},
};

use crate::utils::SELECTOR;
use carbon_dump::SYNTAX_SET;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
    path::Path,
    str::FromStr,
};
use syntect::parsing::SyntaxReference;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Default)]
pub struct ExpertStatistics {
    language: Box<str>,
    files: usize,
    functions: usize,
    types: usize,
    lines: usize,
    chars: usize,
    code_lines: usize,
    comment_lines: usize,
    comment_words: usize,
    comment_chars: usize,
    comment_doc_lines: usize,
    comment_doc_words: usize,
    comment_doc_chars: usize,
}

pub fn print_stats(stats: &ExpertStatistics) {
    println!("File count:                           {:>6}", stats.files);
    println!("Total characters:                     {:>6}", stats.chars);
    println!();
    println!("Function count:                       {:>6}", stats.functions);
    println!("Type count (structs, enums, classes): {:>6}", stats.types);
    println!();
    println!("Code lines (traditional SLOC):        {:>6}", stats.code_lines);
    println!("Total lines (w/ comments & blanks):   {:>6}", stats.lines);
    println!("Comment lines (comment but no code):  {:>6}", stats.comment_lines);
    println!("Blank lines (lines-blank-comment):    {:>6}", stats.lines - stats.code_lines - stats.comment_lines);
    println!();
    println!("Lines with a documentation comment:   {:>6}", stats.comment_doc_lines);
    println!("Total words written in doc comments:  {:>6}", stats.comment_doc_words);
    println!("Total words written in all comments:  {:>6}", stats.comment_words);
    println!("Characters of comment:                {:>6}", stats.comment_chars);
}

pub fn count_line(ops: &[(usize, ScopeStackOp)], line: &str, stack: &mut ScopeStack, stats: &mut ExpertStatistics) {
    stats.lines += 1;
    let mut line_has_comment = false;
    let mut line_has_doc_comment = false;
    let mut line_has_code = false;
    for (s, op) in ScopeRegionIterator::new(&ops, line) {
        stack.apply(op);
        if s.is_empty() {
            // in this case we don't care about blank tokens
            continue;
        }
        if SELECTOR.comment.does_match(stack.as_slice()).is_some() {
            let words =
                s.split_whitespace().filter(|w| w.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '\'')).count();
            if SELECTOR.comment_doc.does_match(stack.as_slice()).is_some() {
                line_has_doc_comment = true;
                stats.comment_doc_words += words;
            }
            stats.comment_chars += s.len();
            stats.comment_words += words;
            line_has_comment = true;
        }
        else if !s.chars().all(|c| c.is_whitespace()) {
            line_has_code = true;
        }
        if SELECTOR.function.does_match(stack.as_slice()).is_some() {
            stats.functions += 1;
        }
        if SELECTOR.types.does_match(stack.as_slice()).is_some() {
            stats.types += 1;
        }
    }
    if line_has_comment && !line_has_code {
        stats.comment_lines += 1;
    }
    if line_has_doc_comment {
        stats.comment_doc_lines += 1;
    }
    if line_has_code {
        stats.code_lines += 1;
    }
}

pub fn count(path: &Path, stats: &mut ExpertStatistics) {
    let syntax = match SYNTAX_SET.find_syntax_for_file(path).unwrap_or(None) {
        Some(syntax) => syntax,
        None => return,
    };
    stats.files += 1;
    let mut state = ParseState::new(syntax);

    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut stack = ScopeStack::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        {
            let ops = state.parse_line(&line, &SYNTAX_SET);
            stats.chars += line.len();
            count_line(&ops, &line, &mut stack, stats);
        }
        line.clear();
    }
}
