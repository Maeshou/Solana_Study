use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf539mvTWf");

#[program]
pub mod update_setting_539 {
    use super::*;

    pub fn update_setting(ctx: Context<Ctx539>, info_str: String) -> Result<()> {
        let old_text = ctx.accounts.record.text.clone();
        ctx.accounts.record.text = info_str.clone();
        msg!("Case 539: '{}' -> '{}'", old_text, info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx539<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record539>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record539 {
    pub owner: Pubkey,
    pub text: String,
}
