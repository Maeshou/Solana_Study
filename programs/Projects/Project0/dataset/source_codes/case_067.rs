use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf067mvTWf");

#[program]
pub mod renew_key_067 {
    use super::*;

    pub fn renew_key(ctx: Context<Ctx067>) -> Result<()> {
        let previous = ctx.accounts.record.key;
        let new_key = ctx.accounts.new_key.key();
        ctx.accounts.record.key = new_key;
        msg!("Case 067: key updated from {} to {}", previous, new_key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx067<'info> {
    #[account(mut, has_one = manager)]
    pub record: Account<'info, Record067>,
    #[account(signer)]
    pub manager: Signer<'info>,
    pub new_key: Signer<'info>,
}

#[account]
pub struct Record067 {
    pub manager: Pubkey,
    pub key: Pubkey,
}
