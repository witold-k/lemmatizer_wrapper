# lemmatizer_wrapper

A small Rust orchestrator for a local lemmatization backend, following the
configuration and external-backend pattern of
[pdf_to_text_wrapper](https://github.com/witold-k/pdf_to_text_wrapper).

Only **spaCy** is supported at present. Backend selection follows the same
pattern as pdf_to_text_wrapper: an enum, a default in the configuration, and
an optional CLI override.

## Install

```sh
python3 -m venv .venv
.venv/bin/pip install spacy
.venv/bin/python -m spacy download en_core_web_sm
```

The first run creates `~/.config/lemmatizer_wrapper/config.json`
(or under `$XDG_CONFIG_HOME`). Set `spacy.python` to the absolute path
of your virtual environment's Python executable. The configuration defaults
to `"backend": "spacy"` and `"scanendfilter": [".md"]`.
All configuration fields are required when loading an existing file; add
`"scanendfilter": [".md"]` to configurations created by older versions.

To make the Python backend module importable, set:

```sh
export PYTHONPATH="$PWD/python"
```

## Run

Use the configured default backend:

```sh
cargo run -- input.md output.json
```

Or select spaCy explicitly:

```sh
cargo run -- spacy input.md output.json
```

The output is a JSON array of tokens in source order. Each item has
`text`, `lemma`, `start`, and `end` (Python character offsets).
This preserves the original token spelling alongside its lemma.
It does not yet generate token pairs, triples, or corpus matrices.

The command accepts either a single source file or a directory. Directory mode
recursively scans the UTF-8 file extensions in `scanendfilter` (for example,
`[".md", ".txt"]`), produces per-document `.tok`, `.tdb`, and annotation
`.json` files, then builds the corpus-global token database and remapped
token streams. It checks modification times to avoid unnecessary work.

```sh
cargo run -- docs/ corpus-output/
cargo run -- article.md article.json
```

Configured extensions are input **text** formats: spaCy reads their contents
as UTF-8. PDF and DOCX binaries are not supported directly; convert those
files to text first using `pdf_to_text_wrapper`.

When multiple input formats have the same basename, outputs stay distinct:
`article.md` maps to `article.tok`, while `article.txt` maps to
`article.txt.tok`. Existing Markdown output names are preserved.
