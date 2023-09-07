mod config;
mod lexer;
mod util;

use lexer::*;
use std::fs;
use std::process::exit;
use util::utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::parse().unwrap_or_else(|| {
        exit(1);
    });
    let dir = fs::read_dir(config.file_path)?;

    let mut tf_index = TermFreqIndex::new();

    for entry in dir {
        let mut tf = TermFreq::new();
        let path = entry.expect("TODO:").path();
        let content = read_entire_file(&path)?.chars().collect::<Vec<_>>();

        println!("Indexing {}... ", path.display());

        for token in Lexer::new(&content) {
            let token = token
                .into_iter()
                .map(|x| x.to_ascii_lowercase())
                .collect::<String>();

            if let Some(count) = tf.get_mut(&token) {
                *count += 1;
            } else {
                tf.insert(token, 1);
            }
        }

        let mut stats = tf.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();

        tf_index.insert(path, tf);
    }

    println!("Indexing Done...");

    write_tf_to_file(tf_index)?;

    Ok(())
}
