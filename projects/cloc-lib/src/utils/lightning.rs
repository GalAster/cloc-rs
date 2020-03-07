use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use tokei::{Config, Languages, LanguageType};


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

    let mut languages = Languages::new();
    languages.get_statistics(paths, excluded, &config);

    for (a, b) in languages.iter() {
        println!("{} {:?}", a, (b.blanks, b.code, b.comments, b.lines));
    }
}

