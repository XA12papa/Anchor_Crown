use anchor_lang::prelude::*;

#[error_code]
pub enum CrownError {
    #[msg("Course title cannot be empty.")]
    EmptyTitle,

    #[msg("Course title is too long.")]
    TitleTooLong,


    #[msg("You are not the course instructor")]
    Unauthorised,
}