// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::{fmt, io, process::ExitStatus};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    InvalidBackend(String),
    SpacyFailed(ExitStatus),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Json(error) => write!(f, "JSON error: {error}"),
            Self::InvalidBackend(name) => write!(f, "unsupported backend: {name} (only spacy is supported)"),
            Self::SpacyFailed(status) => write!(f, "spaCy process failed: {status}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::InvalidBackend(_) | Self::SpacyFailed(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self { Self::Io(error) }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self { Self::Json(error) }
}
