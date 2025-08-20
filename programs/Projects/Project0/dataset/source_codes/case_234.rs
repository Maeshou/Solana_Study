use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf234mvTWf");

#[program]
pub mod sync_token_234 {
    use super::*;

    pub fn sync_token(ctx: Context<Ctx234>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 234 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 234: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx234<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item234>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item234 {
    pub matched: Pubkey,
    pub message: String,
}
