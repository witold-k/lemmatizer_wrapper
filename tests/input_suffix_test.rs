use lemmatizer_wrapper::{config::Config, corpus::mapped_token_output};
use std::path::Path;

#[test]
fn default_suffix_is_markdown() {
    let config = Config::default();
    assert_eq!(config.scanendfilter, vec![".md"]);
    assert_eq!(config.scan_extensions().unwrap(), vec!["md"]);
}

#[test]
fn missing_config_fields_are_rejected() {
    let mut value = serde_json::to_value(Config::default()).unwrap();
    value.as_object_mut().unwrap().remove("scanendfilter");
    let error = serde_json::from_value::<Config>(value).unwrap_err();
    assert!(error.to_string().contains("missing field `scanendfilter`"));
}

#[test]
fn configured_suffixes_are_case_insensitive_and_unique() {
    let mut config = Config::default();
    config.scanendfilter = vec![".md".into(), ".TXT".into(), ".txt".into()];
    assert_eq!(config.scan_extensions().unwrap(), vec!["md", "txt"]);
    let file = std::env::temp_dir().join(format!("lemmatizer-input-{}.TXT", std::process::id()));
    std::fs::write(&file, "text").unwrap();
    assert!(config.accepts(&file));
    std::fs::remove_file(file).unwrap();
}

#[test]
fn malformed_suffixes_are_rejected() {
    for invalid in ["txt", "", ".", ".tar.gz", "../pdf"] {
        let mut config = Config::default();
        config.scanendfilter = vec![invalid.into()];
        assert!(config.scan_extensions().is_err(), "{invalid}");
    }
    let mut config = Config::default();
    config.scanendfilter.clear();
    assert!(config.scan_extensions().is_err());
}

#[test]
fn different_source_types_get_distinct_token_paths() {
    let out = Path::new("/tmp/report.tok");
    let md = mapped_token_output(Path::new("/tmp/report.md"), out, "tok").unwrap();
    let txt = mapped_token_output(Path::new("/tmp/report.txt"), out, "tok").unwrap();
    assert_eq!(md, out);
    assert_eq!(txt, Path::new("/tmp/report.txt.tok"));
    assert_ne!(md, txt);
}
