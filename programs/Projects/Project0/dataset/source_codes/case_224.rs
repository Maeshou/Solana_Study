use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf224mvTWf");

#[program]
pub mod sync_token_224 {
    use super::*;

    pub fn sync_token(ctx: Context<Ctx224>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 224 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 224: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx224<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item224>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item224 {
    pub matched: Pubkey,
    pub message: String,
}
