use crate::utils::{
    expert::{count, print_stats},
    ExpertStatistics,
};
use walkdir::{DirEntry, WalkDir};


pub mod utils;

fn is_ignored(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with(".") && s.len() > 1).unwrap_or(false)
}

#[test]
fn main() {
    let mut stats = ExpertStatistics::default();
    for entry in WalkDir::new(".").into_iter().filter_entry(|e| !is_ignored(e)) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            count(entry.path(), &mut stats);
        }
    }
    println!("{:#?}", stats);
    print_stats(&stats);
}
