use anchor_lang::prelude::*;

use crate::{
    constants::MAX_TITLE_LENGTH,
    error::CrownError,
    state::Course,
};

#[derive(Accounts)]
#[instruction(course_id: u64)]// anchor pull the data from the instruction and uses it to validate the  account constraints
pub struct InitializeCourse<'info> {
    #[account(
        init, //Create this account if it doesn't already exist, allocate its required space, fund it appropriately, assign ownership to this program, and initialize its Anchor account discriminator.
        seeds = [
            b"course", // the seed is a byte string that is used to generate the PDA
            instructor.key().as_ref(), // the seed is the instructor's public key
            &course_id.to_le_bytes() // the seed is the course id in little endian format
        ],
        bump, // the bump is a value that is used to generate the PDA
        payer = instructor, // funds the accout deposit amount
        space = 8 + Course::INIT_SPACE



        // [NOTE : the client derives/creates the pda and passes it to the program as an account, the program then checks if the pda is valid and if it is not valid it will throw an error. The client can derive the pda using the same seeds and bump as the program, this is done by using the find_program_address function which takes in the seeds and the program id and returns the pda and the bump. The client can then use this pda to create the account and pass it to the program as an account. The program will then check if the pda is valid and if it is not valid it will throw an error.]
    )]
    pub course: Account<'info, Course>,

    #[account(mut)] // account is expected to be modified 
    pub instructor: Signer<'info>, //This account must have provided a valid transaction signature.

    pub system_program: Program<'info, System>, //The supplied account must actually be the Solana System Program.
}

impl<'info> InitializeCourse<'info> {
    pub fn handler(
        ctx: Context<InitializeCourse>,
        course_id: u64,
        title: String,
        price: u64,
    ) -> Result<()> {
        require!(!title.is_empty(), CrownError::EmptyTitle);

        require!(
            title.len() <= MAX_TITLE_LENGTH,
            CrownError::TitleTooLong
        );

        msg!("bump :{:?}", ctx.bumps.course); // anchor auatomatically calculates the bump value for the PDA and stores it in the context's bumps map, which can be accessed using the account's name as the key. This is useful for ensuring that the correct bump value is used when generating the PDA, as it can vary depending on the seeds used and the current state of the Solana network.

        let course = &mut ctx.accounts.course;


        course.instructor = ctx.accounts.instructor.key();
        course.bump  = ctx.bumps.course;
        course.title = title;
        course.price = price;
        course.course_id = course_id;

        Ok(())
    }
}