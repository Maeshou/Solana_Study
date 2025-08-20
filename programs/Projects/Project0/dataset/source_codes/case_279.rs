use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf279mvTWf");

#[program]
pub mod link_profile_279 {
    use super::*;

    pub fn link_profile(ctx: Context<Ctx279>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 279 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 279: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx279<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item279>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item279 {
    pub matched: Pubkey,
    pub message: String,
}
