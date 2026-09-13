use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("4zY1qLGxTGU2zvPbwtTAk7vFB2jvdNXXZYWua7u5N8rj");


#[program]
pub mod crown {
    use super::*;

    pub fn initialize_course(
        ctx: Context<InitializeCourse>,
        course_id: u64,
        title: String,
        price: u64,
    ) -> Result<()> {
        InitializeCourse::handler(ctx, course_id, title, price)
    }

    pub fn update_course(
        ctx : Context<UpdateCourse>,
        _course_id: u64,
        new_title: String,
        new_price: u64,
    ) -> Result<()> {
        UpdateCourse::handler(ctx, _course_id,new_title, new_price)
    }
}