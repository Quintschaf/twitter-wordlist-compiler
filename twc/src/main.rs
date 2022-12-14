use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use clap::Parser;
use libtwc::TweetCompiler;

#[derive(Debug, clap::Parser)]
struct Args {}

fn main() -> anyhow::Result<()> {
    let _args = Args::parse();
    let language_map = TweetCompiler::from_directory("sources").compile();

    std::fs::create_dir_all("output")?;
    for (language, word_list) in language_map {
        if word_list.is_empty() {
            continue;
        }
        let filename = format!("output/twitter_corpus_{}.txt", language);
        let entries = {
            let mut kvps = Vec::from_iter(word_list);
            kvps.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
            kvps
        };
        println!("[{}] Writing {} entries", language, entries.len());
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(filename)?;
        let mut writer = BufWriter::new(file);
        for (word, count) in entries {
            writeln!(writer, "{} {}", word, count)?;
        }
    }

    Ok(())
}
