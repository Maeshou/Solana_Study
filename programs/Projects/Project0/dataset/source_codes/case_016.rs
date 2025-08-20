use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf016mvTWf");

#[program]
pub mod activate_credential_016 {
    use super::*;

    pub fn activate_credential(ctx: Context<Ctx016>) -> Result<()> {
        let old_val = ctx.accounts.target.value;
        let new_val = ctx.accounts.user_input.key().to_bytes()[0] as u64 + old_val;
        ctx.accounts.target.value = new_val;
        let diff = new_val.checked_sub(old_val).unwrap();
        msg!("Case 016: credential changed by {}", diff);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx016<'info> {
    #[account(mut, has_one = admin)]
    pub target: Account<'info, Target016>,
    #[account(signer)]
    pub admin: Signer<'info>,
    pub user_input: Signer<'info>,
}

#[account]
pub struct Target016 {
    pub admin: Pubkey,
    pub value: u64,
}
