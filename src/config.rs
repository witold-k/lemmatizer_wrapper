// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{backend::LemmatizerBackend, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend: LemmatizerBackend,
    pub spacy: SpacyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacyConfig {
    pub python: PathBuf,
    pub model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            backend: LemmatizerBackend::Spacy,
            spacy: SpacyConfig {
                python: PathBuf::from("python3"),
                model: "en_core_web_sm".into(),
            },
        }
    }
}

impl Config {
    pub fn default_path() -> PathBuf {
        if let Some(dir) = env::var_os("XDG_CONFIG_HOME") {
            return PathBuf::from(dir).join("lemmatizer_wrapper/config.json");
        }
        if let Some(home) = env::var_os("HOME") {
            return PathBuf::from(home).join(".config/lemmatizer_wrapper/config.json");
        }
        PathBuf::from("config.json")
    }

    pub fn load_or_create() -> Result<Self> {
        let path = Self::default_path();
        if !path.exists() {
            let config = Self::default();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, serde_json::to_string_pretty(&config)?)?;
            return Ok(config);
        }
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
}
