use es_manifest;

//current user name
const USER_NAME: &str = "\\u";

//shell name
const SHELL_NAME: &str = "\\s";

//version of the shell, you are using
const SHELL_VERSION: &str = "\\v";

//current working directory
const CURRENT_DIRECTORY: &str = "\\w";

//current working directory, full path
const CURRENT_F_DIRECTORY: &str = "\\W";

pub fn parse(string: &str) -> String {
    string
        .replace(CURRENT_F_DIRECTORY, &get_current_dir_path(true))
        .replace(CURRENT_DIRECTORY, &get_current_dir_path(false))
        .replace(USER_NAME, &get_user())
        .replace(SHELL_NAME, &es_manifest::name())
        .replace(SHELL_VERSION, &es_manifest::version())
}

fn get_user() -> String {
    std::env::var("USER").unwrap_or_default()
}

fn get_current_dir_path(is_full_path: bool) -> String {
    match std::env::current_dir() {
        Ok(path) => {
            if is_full_path == false {
                path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            } else {
                path.to_string_lossy().to_string()
            }
        }
        Err(_) => "/".to_owned(),
    }
}
