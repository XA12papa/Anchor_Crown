use anchor_lang::prelude::*;

use crate::{
    state::Course,
    error::CrownError,
    constants::MAX_TITLE_LENGTH
};


#[derive(Accounts)]
#[instruction(_course_id : u64)]
pub struct TerminateCourse<'info>{
    #[account(
        mut,
        seeds=[
            b"course",
            instructor.key().as_ref(),
            &_course_id.to_le_bytes(),
        ],
        bump,
        has_one = instructor,
        close = instructor,

    )]
    pub course : Account<'info,Course>,

    #[account(mut)]
    pub instructor: Signer<'info>
}

impl<'info> TerminateCourse<'info> {
    pub fn handler(
        _ctx : Context<'info,TerminateCourse>,
        _course_id : u64,
    )-> Result<()>{
        Ok(())
    }
}

