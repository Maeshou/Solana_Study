use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf027mvTWf");

#[program]
pub mod renew_key_027 {
    use super::*;

    pub fn renew_key(ctx: Context<Ctx027>) -> Result<()> {
        let previous = ctx.accounts.record.key;
        let new_key = ctx.accounts.new_key.key();
        ctx.accounts.record.key = new_key;
        msg!("Case 027: key updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx027<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record027>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_key: Signer<'info>,
}

#[account]
pub struct Record027 {
    pub manager: Pubkey,
    pub key: Pubkey,
}
