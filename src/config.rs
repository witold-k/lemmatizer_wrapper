// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{backend::LemmatizerBackend, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::{Path, PathBuf}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// UTF-8 source suffixes accepted in file and directory mode.
    pub scanendfilter: Vec<String>,
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
            scanendfilter: vec![".md".into()],
            backend: LemmatizerBackend::Spacy,
            spacy: SpacyConfig {
                python: PathBuf::from("python3"),
                model: "en_core_web_sm".into(),
            },
        }
    }
}

impl Config {
    /// Accept only configured source suffixes, ignoring ASCII case.
    pub fn accepts(&self, path: &Path) -> bool {
        path.is_file()
            && path.file_name().and_then(|name| name.to_str()).is_some_and(|name| {
                self.scanendfilter.iter().any(|suffix| {
                    !suffix.is_empty() && name.to_ascii_lowercase().ends_with(&suffix.to_ascii_lowercase())
                })
            })
    }

    /// Validate configured suffixes and return unique scanner extensions.
    pub fn scan_extensions(&self) -> Result<Vec<String>> {
        use crate::error::Error;
        let mut extensions = Vec::new();
        for suffix in &self.scanendfilter {
            let extension = suffix.strip_prefix('.').filter(|ext| {
                !ext.is_empty()
                    && !ext.contains('.')
                    && !ext.contains('/')
                    && !ext.contains('\\')
                    && ext.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            }).ok_or_else(|| Error::InvalidSuffix(suffix.clone()))?;
            let extension = extension.to_ascii_lowercase();
            if !extensions.contains(&extension) {
                extensions.push(extension);
            }
        }
        if extensions.is_empty() {
            return Err(Error::InvalidSuffix("scanendfilter must not be empty".into()));
        }
        Ok(extensions)
    }

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
