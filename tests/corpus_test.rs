use lemmatizer_wrapper::corpus::{save_token_stream, load_token_stream, globalize};
use token_db::TokenDb;

#[test]
fn local_ids_are_remapped_to_global_ids_without_losing_order() {
    let root = std::env::temp_dir().join(format!("lemmer-test-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let path = root.join("sample.tok");
    let mut local = TokenDb::new();
    let a = local.insert("laufen").unwrap();
    let b = local.insert("Programm").unwrap();
    local.insert("laufen").unwrap();
    local.save(&path.with_extension("tdb")).unwrap();
    save_token_stream(&path, &[a,b,a]).unwrap();
    let mut global = TokenDb::new();
    global.insert("andere").unwrap();
    global.merge(&local).unwrap();
    globalize(&path, &global).unwrap();
    let result = load_token_stream(&root.join("sample_glob.tok")).unwrap();
    assert_eq!(result, vec![global.id("laufen").unwrap(),global.id("Programm").unwrap(),global.id("laufen").unwrap()]);
    std::fs::remove_dir_all(root).unwrap();
}
