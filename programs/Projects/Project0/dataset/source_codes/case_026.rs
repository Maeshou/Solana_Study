use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf026mvTWf");

#[program]
pub mod rotate_token_026 {
    use super::*;

    pub fn rotate_token(ctx: Context<Ctx026>) -> Result<()> {
        let old_val = ctx.accounts.target.value;
        let new_val = ctx.accounts.user_input.key().to_bytes()[0] as u64 + old_val;
        ctx.accounts.target.value = new_val;
        let diff = new_val.checked_sub(old_val).unwrap();
        msg!("Case 026: token changed by {}", diff);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx026<'info> {
    #[account(mut, has_one = admin)]
    pub target: Account<'info, Target026>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub user_input: Signer<'info>,
}

#[account]
pub struct Target026 {
    pub admin: Pubkey,
    pub value: u64,
}
