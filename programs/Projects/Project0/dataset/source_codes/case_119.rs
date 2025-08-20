use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf119mvTWf");

#[program]
pub mod assign_data_119 {
    use super::*;

    pub fn assign_data(ctx: Context<Ctx119>) -> Result<()> {
        let old_text = ctx.accounts.item.text.clone();
        let new_text = format!("Case 119 executed by {}", ctx.accounts.actor.key());
        ctx.accounts.item.text = new_text.clone();
        msg!("Case 119: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx119<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, Item119>,
    #[account(signer)]
    pub owner: Signer<'info>,
    pub actor: Signer<'info>,
}

#[account]
pub struct Item119 {
    pub owner: Pubkey,
    pub text: String,
}
