use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculates the space required to store the account data based on the struct definition and its fields. This is useful for ensuring that the account has enough space allocated when it is created.    
pub struct Course {
    pub instructor: Pubkey,
    pub course_id: u64,
    pub bump: u8,
    
    #[max_len(100)]
    pub title: String,

    pub price: u64,
}