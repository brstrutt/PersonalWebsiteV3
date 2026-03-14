use crate::app::URL_SUFFIX;

pub fn internal_path(path: &str) -> String {
    format!("{}{}", URL_SUFFIX, path)
}