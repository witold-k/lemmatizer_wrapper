// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{config::SpacyConfig, Error, Result};
use serde::Deserialize;
use std::{path::Path, process::Command};

#[derive(Debug, Deserialize)]
pub struct SpacyToken {
    pub text: String,
    pub lemma: String,
    pub start: usize,
    pub end: usize,
    #[serde(default)]
    pub pos: String,
    #[serde(default)]
    pub dep: String,
    #[serde(default)]
    pub head: usize,
    #[serde(default)]
    pub is_space: bool,
}

/// Runs spaCy in a separate Python process, preserving token order.
pub fn lemmatize(config: &SpacyConfig, input: &Path, output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() { std::fs::create_dir_all(parent)?; }
    let module_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/python");
    let status = Command::new(&config.python)
        .env("PYTHONPATH", module_dir)
        .arg("-m")
        .arg("lemmatizer_wrapper_spacy")
        .arg(&config.model)
        .arg(input)
        .arg(output)
        .status()?;
    if status.success() { Ok(()) } else { Err(Error::SpacyFailed(status)) }
}
