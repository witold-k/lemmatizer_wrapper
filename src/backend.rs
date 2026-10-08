// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{config::Config, error::{Error, Result}, spacy};
use serde::{Deserialize, Serialize};
use std::{fmt, path::Path, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LemmatizerBackend {
    Spacy,
}

impl fmt::Display for LemmatizerBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spacy => write!(f, "spacy"),
        }
    }
}

impl FromStr for LemmatizerBackend {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        if value.eq_ignore_ascii_case("spacy") {
            Ok(Self::Spacy)
        } else {
            Err(Error::InvalidBackend(value.to_owned()))
        }
    }
}

pub fn lemmatize(config: &Config, backend: LemmatizerBackend, input: &Path, output: &Path) -> Result<()> {
    match backend {
        LemmatizerBackend::Spacy => spacy::lemmatize(&config.spacy, input, output),
    }
}
