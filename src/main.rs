// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use lemmatizer_wrapper::{backend::{lemmatize, LemmatizerBackend}, config::Config, Result};
use std::path::Path;

fn usage() {
    eprintln!("Usage: lemmatizer_wrapper [spacy] <input.txt> <output.json>");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::load_or_create()?;
    let (backend, input, output) = match args.as_slice() {
        [_, input, output] => (config.backend, input.as_str(), output.as_str()),
        [_, backend, input, output] => (backend.parse::<LemmatizerBackend>()?, input.as_str(), output.as_str()),
        _ => {
            usage();
            std::process::exit(2);
        }
    };
    lemmatize(&config, backend, Path::new(input), Path::new(output))
}
