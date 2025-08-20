use anchor_lang::prelude::*;

declare_id!("FlipLedger22222222222222222222222222222222");

#[program]
pub mod ledger_flag {
    use super::*;

    pub fn flip(ctx: Context<Flip>) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        rec.flag = !rec.flag;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Flip<'info> {
    #[account(mut, has_one = authority)]
    pub rec: Account<'info, FlipData>,
    pub authority: Signer<'info>,
}

#[account]
pub struct FlipData {
    pub authority: Pubkey,
    pub flag: bool,
}
