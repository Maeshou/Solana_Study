use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf071mvTWf");

#[program]
pub mod approve_profile_071 {
    use super::*;

    pub fn approve_profile(ctx: Context<Ctx071>) -> Result<()> {
        let old_val = ctx.accounts.target.value;
        let new_val = ctx.accounts.user_input.key().to_bytes()[0] as u64 + old_val;
        ctx.accounts.target.value = new_val;
        let diff = new_val.checked_sub(old_val).unwrap();
        msg!("Case 071: profile changed by {}", diff);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx071<'info> {
    #[account(mut, has_one = admin)]
    pub target: Account<'info, Target071>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub user_input: Signer<'info>,
}

#[account]
pub struct Target071 {
    pub admin: Pubkey,
    pub value: u64,
}
