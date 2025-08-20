use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf204mvTWf");

#[program]
pub mod sync_token_204 {
    use super::*;

    pub fn sync_token(ctx: Context<Ctx204>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 204 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 204: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx204<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item204>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item204 {
    pub matched: Pubkey,
    pub message: String,
}
