
// The path to the Zuko CLI database file
use std::fs;
use std::path::PathBuf;

/// Returns the path to the Zuko CLI database file path.
/// This function constructs the path based on the current executable's directory,
/// assuming the database file is located in a `db` subdirectory.
pub fn get_zuko_cli_db_path() -> PathBuf {
    let exe_path = std::env::current_exe().expect("Cannot get current exe path");
    let canonical_path = fs::canonicalize(&exe_path).unwrap_or(exe_path);
    canonical_path
        .parent()
        .expect("No parent dir")
        .join("db")
        .join("zuko_cli.db")
}


pub fn get_zuko_user_db_path(base_path: &PathBuf) -> PathBuf {
    // Construct the path to the Zuko user database file
    base_path.join("zuko_user.db")
}


