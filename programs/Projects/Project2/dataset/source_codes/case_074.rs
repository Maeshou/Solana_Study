
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AwardPrizeCtxmmto<'info> {
    #[account(mut)] pub tournament: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_074 {
    use super::*;

    pub fn award_prize(ctx: Context<AwardPrizeCtxmmto>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.tournament;
        // custom logic for award_prize
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed award_prize logic");
        Ok(())
    }
}
