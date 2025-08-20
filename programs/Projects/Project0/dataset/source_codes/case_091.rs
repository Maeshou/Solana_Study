use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf091mvTWf");

#[program]
pub mod approve_profile_091 {
    use super::*;

    pub fn approve_profile(ctx: Context<Ctx091>) -> Result<()> {
        let old_val = ctx.accounts.target.value;
        let new_val = ctx.accounts.user_input.key().to_bytes()[0] as u64 + old_val;
        ctx.accounts.target.value = new_val;
        let diff = new_val.checked_sub(old_val).unwrap();
        msg!("Case 091: profile changed by {}", diff);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx091<'info> {
    #[account(mut, has_one = admin)]
    pub target: Account<'info, Target091>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub user_input: Signer<'info>,
}

#[account]
pub struct Target091 {
    pub admin: Pubkey,
    pub value: u64,
}
