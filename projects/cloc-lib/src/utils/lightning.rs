use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use tokei::{Config, Languages, LanguageType};
use std::{
    fmt,
    io::{self, Write},
    process,
    str::FromStr,
};


use tokei::Language;

pub const FALLBACK_ROW_LEN: usize = 79;
const NO_LANG_HEADER_ROW_LEN: usize = 67;
const NO_LANG_ROW_LEN: usize = 61;
const NO_LANG_ROW_LEN_NO_SPACES: usize = 54;
const IDENT_INACCURATE: &str = "(!)";


pub fn parse_or_exit<T>(s: &str) -> T
    where
        T: FromStr,
        T::Err: fmt::Display,
{
    T::from_str(s).unwrap_or_else(|e| {
        eprintln!("Error:\n{}", e);
        process::exit(1);
    })
}

pub fn print_header<W: Write>(sink: &mut W, row: &str, columns: usize) -> io::Result<()> {
    writeln!(sink, "{}", row)?;
    writeln!(
        sink,
        " {:<6$} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "Language",
        "Files",
        "Lines",
        "Code",
        "Comments",
        "Blanks",
        columns - NO_LANG_HEADER_ROW_LEN
    )?;

    writeln!(sink, "{}", row)
}

pub fn print_results<'a, I, W>(
    sink: &mut W,
    row: &str,
    languages: I,
    list_files: bool,
) -> io::Result<()>
    where
        I: Iterator<Item=(&'a LanguageType, &'a Language)>,
        W: Write,
{
    let path_len = row.len() - NO_LANG_ROW_LEN_NO_SPACES;
    let columns = row.len();
    for (name, language) in languages.filter(isnt_empty) {
        print_language(sink, columns, language, name.name())?;

        if list_files {
            writeln!(sink, "{}", row)?;
            for stat in &language.stats {
                writeln!(sink, "{:1$}", stat, path_len)?;
            }
            writeln!(sink, "{}", row)?;
        }
    }

    Ok(())
}

pub fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}

pub fn print_language<W>(
    sink: &mut W,
    columns: usize,
    language: &Language,
    name: &str,
) -> io::Result<()>
    where
        W: Write,
{
    let mut lang_section_len = columns - NO_LANG_ROW_LEN;
    if language.inaccurate {
        lang_section_len -= IDENT_INACCURATE.len();
    }
    // truncate and replace the last char with a `|` if the name is too long
    if lang_section_len < name.len() {
        write!(sink, " {:.len$}", name, len = lang_section_len - 1)?;
        write!(sink, "|")?;
    } else {
        write!(sink, " {:<len$}", name, len = lang_section_len)?;
    }
    if language.inaccurate {
        write!(sink, "{}", IDENT_INACCURATE)?;
    };
    write!(sink, " ")?;
    writeln!(
        sink,
        "{:>6} {:>12} {:>12} {:>12} {:>12}",
        language.stats.len(),
        language.lines,
        language.code,
        language.comments,
        language.blanks
    )
}

pub fn print_inaccuracy_warning<W>(sink: &mut W) -> io::Result<()>
    where
        W: Write,
{
    writeln!(
        sink,
        "Note: results can be inaccurate for languages marked with '{}'",
        IDENT_INACCURATE
    )
}

#[test]
fn tokei() {
    // The paths to search. Accepts absolute, relative, and glob paths.
    let paths = &["."];
// Exclude any path that contains any of these strings.
    let excluded = &["target"];
// `Config` allows you to configure what is searched and counted.
    let config = Config {
        treat_doc_strings_as_comments: Some(true),
        ..Config::default()
    };
    let mut stdout = io::BufWriter::new(io::stdout());
    let mut languages = Languages::new();
    languages.get_statistics(paths, excluded, &config);
    println!(
        "{:<6$} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9} |",
        "Language",
        "Files",
        "Code",
        "Comments",
        "Blanks",
        "Lines", 16
    );

    let mut max_language = "Language".len();
    let mut max_files = 100000 - 1;
    let mut max_code = 10000 - 1;
    let mut max_comments = 100000000 - 1;
    let mut max_blanks = 1000000 - 1;
    let mut max_lines = max_files;


    for (a, b) in languages.iter() {
        let name = format!("{}", a);
        if name.len() > max_language { max_language = name.len() }
        let files = b.stats.len();
        if files > max_files { max_files = files }
        let code = b.code;
        if code > max_code { max_code = code }
        let comments = b.comments;
        if comments > max_comments { max_comments = comments }
        let blanks = b.blanks;
        if blanks > max_blanks { max_blanks = blanks }
        let lines = b.lines;
        if lines > max_lines { max_lines = lines }
    }
}

