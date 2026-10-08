"""Minimal spaCy backend: JSON tokens with text, lemma, and character offsets."""
import json
import sys
from pathlib import Path

import spacy


def main():
    if len(sys.argv) != 4:
        raise SystemExit("Usage: python -m lemmatizer_wrapper_spacy MODEL INPUT OUTPUT")
    model, input_path, output_path = sys.argv[1:]
    nlp = spacy.load(model)
    doc = nlp(Path(input_path).read_text(encoding="utf-8"))
    tokens = [
        {"text": token.text, "lemma": token.lemma_, "start": token.idx, "end": token.idx + len(token.text)}
        for token in doc
    ]
    Path(output_path).write_text(json.dumps(tokens, ensure_ascii=False, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
