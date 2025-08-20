use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf274mvTWf");

#[program]
pub mod sync_token_274 {
    use super::*;

    pub fn sync_token(ctx: Context<Ctx274>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 274 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 274: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx274<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item274>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item274 {
    pub matched: Pubkey,
    pub message: String,
}
