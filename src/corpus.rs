// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::{config::SpacyConfig, error::{Error, Result}, spacy};
use std::{fs, path::Path};
use token_db::{TokenDb, TokenId};

pub fn save_token_stream(path: &Path, ids: &[TokenId]) -> Result<()> {
    fs::write(path, postcard::to_allocvec(ids)?)?;
    Ok(())
}

pub fn load_token_stream(path: &Path) -> Result<Vec<TokenId>> {
    Ok(postcard::from_bytes(&fs::read(path)?)?)
}

pub fn process_document(config: &SpacyConfig, input: &Path, token_output: &Path) -> Result<()> {
    let db_output = token_output.with_extension("tdb");
    let annotation_output = token_output.with_extension("json");
    if let Some(parent) = token_output.parent() {
        fs::create_dir_all(parent)?;
    }
    // Keep linguistic annotations separate from the compact ordered ID stream.
    spacy::lemmatize(config, input, &annotation_output)?;
    let annotations: Vec<spacy::SpacyToken> =
        serde_json::from_slice(&fs::read(&annotation_output)?)?;
    let mut db = TokenDb::new();
    let ids = annotations.iter()
        .filter(|token| !token.is_space)
        .map(|token| db.insert(&token.lemma).map_err(Error::from))
        .collect::<Result<Vec<_>>>()?;
    save_token_stream(token_output, &ids)?;
    db.save(&db_output)?;
    Ok(())
}

pub fn globalize(token_input: &Path, global: &TokenDb) -> Result<()> {
    let local = TokenDb::load(token_input.with_extension("tdb"))?;
    let mapping = local.iter().map(|(_, entry)| {
        global.id(entry.text()).ok_or_else(|| Error::MissingGlobalToken(entry.text().to_owned()))
    }).collect::<Result<Vec<_>>>()?;
    let ids = load_token_stream(token_input)?.into_iter()
        .map(|id| mapping.get(id.get() as usize).copied().ok_or(Error::InvalidTokenId(id)))
        .collect::<Result<Vec<_>>>()?;
    let stem = token_input.file_stem().and_then(|s| s.to_str())
        .ok_or_else(|| Error::InvalidPath(token_input.to_path_buf()))?;
    save_token_stream(&token_input.with_file_name(format!("{stem}_glob.tok")), &ids)
}

fn stale(input: &Path, output: &Path) -> Result<bool> {
    let modified = fs::metadata(input)?.modified()?;
    match fs::metadata(output) {
        Ok(m) => Ok(modified > m.modified()?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(true),
        Err(e) => Err(e.into()),
    }
}

/// First pass: each Markdown document produces its own token stream, database and annotations.
/// Second pass: merge databases and translate local streams to corpus-global IDs.
pub fn process_corpus(config: &SpacyConfig, input: &Path, output: &Path) -> Result<()> {
    if !input.is_dir() { return Err(Error::InvalidPath(input.to_path_buf())); }
    fs::create_dir_all(output)?;
    let input_str = input.to_str().ok_or_else(|| Error::InvalidPath(input.to_path_buf()))?;
    let output_str = output.to_str().ok_or_else(|| Error::InvalidPath(output.to_path_buf()))?;
    let cfg = config.clone();
    fsscanner::fsscanner_mt::process_dir_map_with_workers(
        input_str, output_str, "md", "tok", 1,
        move |source, dest| {
            let run = || -> Result<()> {
                if stale(source, dest)?
                    || stale(source, &dest.with_extension("tdb"))?
                    || stale(source, &dest.with_extension("json"))? {
                    process_document(&cfg, source, dest)?;
                }
                Ok(())
            };
            run().map_err(|e| fsscanner::Error::Processing(vec![e.to_string()]))
        },
    )?;
    let mut paths = Vec::new();
    fsscanner::fsscanner_base::collect_files_fast(output, "tdb", &mut paths);
    paths.retain(|p| p.file_name().is_some_and(|n| n != "token_db.tdb"));
    paths.sort();
    let global_path = output.join("token_db.tdb");
    let rebuild = !global_path.exists() || paths.iter().try_fold(false, |changed, path| {
        stale(path, &global_path).map(|s| changed || s)
    })?;
    if rebuild {
        let mut global = TokenDb::new();
        for path in &paths { global.merge(&TokenDb::load(path)?)?; }
        global.save(&global_path)?;
    }
    let global = TokenDb::load(&global_path)?;
    for db_path in paths {
        let token_path = db_path.with_extension("tok");
        let stem = token_path.file_stem().and_then(|s| s.to_str())
            .ok_or_else(|| Error::InvalidPath(token_path.clone()))?;
        let global_token = token_path.with_file_name(format!("{stem}_glob.tok"));
        if stale(&global_path, &global_token)? || stale(&token_path, &global_token)? {
            globalize(&token_path, &global)?;
        }
    }
    Ok(())
}
