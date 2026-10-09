// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use lemmatizer_wrapper::{backend::{lemmatize, LemmatizerBackend}, config::Config, corpus, Result};
use std::path::Path;

fn usage() {
    eprintln!("Usage: lemmatizer_wrapper [spacy] <input.md|input_dir> <output.json|output_dir>");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::load_or_create()?;
    let (backend, input, output) = match args.as_slice() {
        [_, input, output] => (config.backend, input.as_str(), output.as_str()),
        [_, backend, input, output] => (backend.parse::<LemmatizerBackend>()?, input.as_str(), output.as_str()),
        _ => { usage(); std::process::exit(2); }
    };
    let input = Path::new(input);
    let output = Path::new(output);
    if input.is_dir() {
        corpus::process_corpus(&config.spacy, input, output)
    } else {
        lemmatize(&config, backend, input, output)
    }
}
