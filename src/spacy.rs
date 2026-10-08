// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{config::SpacyConfig, Error, Result};
use std::{path::Path, process::Command};

/// Runs spaCy in a separate Python process, preserving token order.
pub fn lemmatize(config: &SpacyConfig, input: &Path, output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let status = Command::new(&config.python)
        .arg("-m")
        .arg("lemmatizer_wrapper_spacy")
        .arg(&config.model)
        .arg(input)
        .arg(output)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::SpacyFailed(status))
    }
}
