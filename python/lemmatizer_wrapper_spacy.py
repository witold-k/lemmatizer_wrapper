"""spaCy backend: source-order tokens, lemmata and syntax annotations."""
import json
import sys
from pathlib import Path

import spacy


def main():
    if len(sys.argv) != 4:
        raise SystemExit("Usage: python -m lemmatizer_wrapper_spacy MODEL INPUT OUTPUT")
    model, input_path, output_path = sys.argv[1:]
    nlp = spacy.load(model)
    text = Path(input_path).read_text(encoding="utf-8")
    doc = nlp(text)
    tokens = [
        {
            "text": token.text,
            "lemma": token.lemma_,
            "start": token.idx,
            "end": token.idx + len(token.text),
            "pos": token.pos_,
            "dep": token.dep_,
            "head": token.head.i,
            "is_space": token.is_space,
        }
        for token in doc
    ]
    Path(output_path).write_text(
        json.dumps(tokens, ensure_ascii=False, indent=2), encoding="utf-8"
    )


if __name__ == "__main__":
    main()
