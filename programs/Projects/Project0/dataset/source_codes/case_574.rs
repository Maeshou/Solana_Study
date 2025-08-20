use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf574mvTWf");

#[program]
pub mod append_state_574 {
    use super::*;

    pub fn append_state(ctx: Context<Ctx574>, info_str: String) -> Result<()> {
        let old_text = ctx.accounts.record.text.clone();
        ctx.accounts.record.text = info_str.clone();
        msg!("Case 574: '{}' -> '{}'", old_text, info_str);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx574<'info> {
    #[account(mut, has_one = owner)]
    pub record: Account<'info, Record574>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Record574 {
    pub owner: Pubkey,
    pub text: String,
}
