use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf264mvTWf");

#[program]
pub mod sync_token_264 {
    use super::*;

    pub fn sync_token(ctx: Context<Ctx264>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 264 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 264: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx264<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item264>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item264 {
    pub matched: Pubkey,
    pub message: String,
}
