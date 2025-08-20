use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf524mvTWf");

#[program]
pub mod append_state_524 {
    use super::*;

    pub fn append_state(ctx: Context<Ctx524>, info_str: String) -> Result<()> {
        let old_text = ctx.accounts.record.text.clone();
        ctx.accounts.record.text = info_str.clone();
        msg!("Case 524: '{}' -> '{}'", old_text, info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx524<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record524>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record524 {
    pub owner: Pubkey,
    pub text: String,
}
