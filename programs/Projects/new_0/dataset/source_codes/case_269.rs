use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf269mvTWf");

#[program]
pub mod link_profile_269 {
    use super::*;

    pub fn link_profile(ctx: Context<Ctx269>) -> Result<()> {
        let old_msg = ctx.accounts.item.message.clone();
        let new_msg = format!("Case 269 by {}", ctx.accounts.actor.key());
        ctx.accounts.item.message = new_msg.clone();
        msg!("Case 269: '{}' → '{}'", old_msg, new_msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx269<'info> {
    #[account(mut, has_one = matched)]
    pub item: Account<'info, Item269>,
    #[account(signer)]
    pub matched: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item269 {
    pub matched: Pubkey,
    pub message: String,
}
