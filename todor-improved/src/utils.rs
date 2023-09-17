use dirs::home_dir;

pub const TODOR_DB_FILENAME: &str = "todor.txt";

pub fn get_db_file_path() -> std::path::PathBuf {
    home_dir()
        .map(|path| path.join(TODOR_DB_FILENAME))
        .unwrap_or_default()
}

pub fn db_exists() -> bool {
    let dir = get_db_file_path();
    std::fs::metadata(dir).is_ok()
}

pub fn create_db_file() -> std::io::Result<()> {
    let dir = get_db_file_path();
    std::fs::File::create(dir)?;
    Ok(())
}

pub fn check_db_file() -> std::io::Result<()> {
    if !db_exists() {
        create_db_file()?;
    }
    Ok(())
}   
