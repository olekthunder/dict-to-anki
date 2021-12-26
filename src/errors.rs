use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("failded to fetch")]
    FetchFailed(#[from] reqwest::Error),
    #[error("error building anki deck")]
    AnkiError(#[from] genanki_rs::Error)
}

pub type Result<T> = std::result::Result<T, AppError>;