use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf259mvTWf");

#[program]
pub mod link_profile_259 {
    use super::*;

    pub fn link_profile(ctx: Context<Ctx259>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 259 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 259: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx259<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item259>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item259 {
    pub matched: Pubkey,
    pub message: String,
}
