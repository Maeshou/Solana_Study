use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf001mvTWf");

#[program]
pub mod modify_schema_001 {
    use super::*;

    pub fn modify_schema(ctx: Context<Ctx001>) -> Result<()> {
        let old_val = ctx.accounts.target.value;
        let new_val = ctx.accounts.user_input.key().to_bytes()[0] as u64 + old_val;
        ctx.accounts.target.value = new_val;
        let diff = new_val.checked_sub(old_val).unwrap();
        msg!("Case 001: schema changed by {}", diff);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx001<'info> {
    #[account(mut, has_one = admin)]
    pub target: Account<'info, Target001>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub user_input: Signer<'info>,
}

#[account]
pub struct Target001 {
    pub admin: Pubkey,
    pub value: u64,
}
