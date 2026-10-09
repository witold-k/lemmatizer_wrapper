// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::{fmt, io, path::PathBuf, process::ExitStatus};
use token_db::TokenId;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    Postcard(postcard::Error),
    TokenDb(token_db::Error),
    Scanner(fsscanner::Error),
    InvalidBackend(String),
    InvalidPath(PathBuf),
    InvalidTokenId(TokenId),
    MissingGlobalToken(String),
    SpacyFailed(ExitStatus),
}
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::Postcard(e) => write!(f, "postcard error: {e}"),
            Self::TokenDb(e) => write!(f, "token database error: {e}"),
            Self::Scanner(e) => write!(f, "filesystem scanner error: {e}"),
            Self::InvalidBackend(s) => write!(f, "unsupported backend: {s}"),
            Self::InvalidPath(p) => write!(f, "invalid path: {}", p.display()),
            Self::InvalidTokenId(id) => write!(f, "invalid token ID: {}", id.get()),
            Self::MissingGlobalToken(s) => write!(f, "missing global token: {s}"),
            Self::SpacyFailed(s) => write!(f, "spaCy process failed: {s}"),
        }
    }
}
impl std::error::Error for Error {}
impl From<io::Error> for Error { fn from(e: io::Error) -> Self { Self::Io(e) } }
impl From<serde_json::Error> for Error { fn from(e: serde_json::Error) -> Self { Self::Json(e) } }
impl From<postcard::Error> for Error { fn from(e: postcard::Error) -> Self { Self::Postcard(e) } }
impl From<token_db::Error> for Error { fn from(e: token_db::Error) -> Self { Self::TokenDb(e) } }
impl From<fsscanner::Error> for Error { fn from(e: fsscanner::Error) -> Self { Self::Scanner(e) } }
