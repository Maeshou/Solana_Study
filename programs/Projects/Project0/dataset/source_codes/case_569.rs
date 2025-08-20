use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf569mvTWf");

#[program]
pub mod update_setting_569 {
    use super::*;

    pub fn update_setting(ctx: Context<Ctx569>, info_str: String) -> Result<()> {
        let old_text = ctx.accounts.record.text.clone();
        ctx.accounts.record.text = info_str.clone();
        msg!("Case 569: '{}' -> '{}'", old_text, info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx569<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record569>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record569 {
    pub owner: Pubkey,
    pub text: String,
}
