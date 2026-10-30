use git2::Repository;
use tempfile::TempDir;

#[test]
fn test_git_repository_init() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    assert!(!repo.is_bare());
    assert!(repo.is_empty().unwrap());
}

#[test]
fn test_git_status_empty() {
    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    let statuses = repo.statuses(None).unwrap();
    assert!(statuses.is_empty());
}

#[test]
fn test_git_add_file() {
    use std::fs;

    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    // Crear un archivo
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "test content").unwrap();

    // Verificar que hay cambios sin trackear
    let statuses = repo.statuses(None).unwrap();
    assert!(!statuses.is_empty());
}

#[test]
fn test_git_signature() {
    use git2::Signature;

    let sig = Signature::now("test", "test@example.com").unwrap();
    assert_eq!(sig.name().unwrap(), "test");
    assert_eq!(sig.email().unwrap(), "test@example.com");
}

#[test]
fn test_git_commit_creation() {
    use git2::{IndexAddOption, Signature};
    use std::fs;

    let temp_dir = TempDir::new().unwrap();
    let repo = Repository::init(temp_dir.path()).unwrap();

    // Crear archivo inicial
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "initial content").unwrap();

    // Add y commit
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();

    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let sig = Signature::now("test", "test@example.com").unwrap();

    let commit_id = repo
        .commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
        .unwrap();

    assert!(repo.find_commit(commit_id).is_ok());
}

#[test]
fn test_init_repo_with_remote_creates_main_branch() {
    use std::fs;

    let temp_dir = TempDir::new().unwrap();

    // Crear un archivo para que el commit inicial tenga contenido
    let file_path = temp_dir.path().join("secrets.yml");
    fs::write(&file_path, "hello: world\n").unwrap();

    let dir_str = temp_dir.path().to_str().unwrap();

    // Init con remote — el push fallará (no hay remote real) pero la función
    // devuelve Ok de todas formas
    crypta::git::init_repo_with_remote(dir_str, "git@github.com:test/secrets.git").unwrap();

    // Abrir el repo y verificar la rama activa
    let repo = Repository::open(temp_dir.path()).unwrap();

    // La rama activa debe ser "main", no "master"
    assert_eq!(repo.head().unwrap().shorthand().unwrap(), "main");

    // La rama local "main" debe existir
    assert!(repo.find_branch("main", git2::BranchType::Local).is_ok());

    // La rama local "master" NO debe existir
    assert!(repo.find_branch("master", git2::BranchType::Local).is_err());
}
