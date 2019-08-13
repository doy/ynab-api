pub fn api_key() -> std::path::PathBuf {
    directories::ProjectDirs::from("", "", "ynab")
        .unwrap()
        .config_dir()
        .join("api-key")
}
