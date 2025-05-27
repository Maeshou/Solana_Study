
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ExerciseOptionCtxypcl<'info> {
    #[account(mut)] pub option: Account<'info, DataAccount>,
    #[account(mut)] pub holder: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_092 {
    use super::*;

    pub fn exercise_option(ctx: Context<ExerciseOptionCtxypcl>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.option;
        // custom logic for exercise_option
        **ctx.accounts.option.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed exercise_option logic");
        Ok(())
    }
}
