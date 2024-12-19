use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub debug: bool,
    pub wrapper: Wrapper,
}

#[derive(Deserialize, Debug)]
pub struct Wrapper {
    pub wrapped_cmd: Option<String>,
    pub wrapped_remove_dup_args: Option<Vec<String>>,
    pub wrapped_prepend_args: Option<Vec<String>>,
    pub wrapped_prepend_if: Option<String>,
}
