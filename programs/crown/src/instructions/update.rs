use anchor_lang::prelude::*;

use crate::{
    state::Course,
    error::CrownError,
    constants::MAX_TITLE_LENGTH
};


#[derive(Accounts)]
#[instruction(_course_id: u64)]
pub struct UpdateCourse<'info>{
    #[account(
        mut,
        has_one = instructor, // anchor checks that if the accout provided has the instructor account or not before handint it to he handler 
        seeds = [
            b"course", // the seed is a byte string that is used to generate the PDA
            instructor.key().as_ref(), // the seed is the instructor's public key
            &_course_id.to_le_bytes() // the seed is the course id in little endian format
        ],
        bump = course.bump, // during initialization we stored course.bump so anchor checks the accout passed 


    )]
    pub course: Account<'info,Course>,

    #[account(mut)]
    pub instructor: Signer<'info>,// instructor need to be signer as any one who knows instructors pubkey could initiate the transaction hence we need to make the instructor signer 

}


impl<'info> UpdateCourse<'info> {
    pub fn handler(
        ctx : Context<'info,UpdateCourse>,
        _course_id : u64,
        new_title: String,
        new_price : u64
    ) -> Result<()>{
        require!(
            !new_title.is_empty(),
            CrownError::EmptyTitle
        );

        require!(
            new_title.len() <= MAX_TITLE_LENGTH,
            CrownError::TitleTooLong
        );
        let course = &mut ctx.accounts.course;

        course.title = new_title;
        course.price = new_price;


        Ok(())
    }
}