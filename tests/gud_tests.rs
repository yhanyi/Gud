use gud::utils::repo::Repository;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

fn setup() -> (TempDir, Repository) {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();
    (temp_dir, repo)
}

#[test]
fn test_init() {
    let (temp_dir, _) = setup();
    assert!(temp_dir.path().join(".gud").exists());
}

#[test]
fn test_add_file() {
    let (temp_dir, mut repo) = setup();

    let test_file = temp_dir.path().join("test.txt");
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"test content").unwrap();

    repo.add_file(&test_file).unwrap();

    assert_eq!(repo.staging.len(), 1);
}

#[test]
fn test_commit() {
    let (temp_dir, mut repo) = setup();

    let test_file = temp_dir.path().join("test.txt");
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"test content").unwrap();
    repo.add_file(&test_file).unwrap();

    let commit_hash = repo.commit("Test commit").unwrap();

    assert!(repo.commits.contains_key(&commit_hash));
    assert_eq!(repo.staging.len(), 0);
}

#[test]
fn test_empty_commit_fails() {
    let (_, mut repo) = setup();
    assert!(repo.commit("Empty commit").is_err());
}
