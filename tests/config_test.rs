use lemmatizer_wrapper::{backend::LemmatizerBackend, config::Config};

#[test]
fn default_backend_is_spacy() {
    let config = Config::default();
    assert_eq!(config.backend, LemmatizerBackend::Spacy);
    assert_eq!(config.spacy.model, "en_core_web_sm");
}

#[test]
fn backend_parsing() {
    assert_eq!("spacy".parse::<LemmatizerBackend>().unwrap(), LemmatizerBackend::Spacy);
    assert_eq!("SPACY".parse::<LemmatizerBackend>().unwrap(), LemmatizerBackend::Spacy);
    assert!("unknown".parse::<LemmatizerBackend>().is_err());
}

#[test]
fn backend_serialization() {
    let config = Config::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"backend\":\"spacy\""));
    let restored: Config = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.backend, LemmatizerBackend::Spacy);
}
