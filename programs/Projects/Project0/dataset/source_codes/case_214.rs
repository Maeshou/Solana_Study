use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf214mvTWf");

#[program]
pub mod sync_token_214 {
    use super::*;

    pub fn sync_token(ctx: Context<Ctx214>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 214 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 214: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx214<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item214>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item214 {
    pub matched: Pubkey,
    pub message: String,
}
