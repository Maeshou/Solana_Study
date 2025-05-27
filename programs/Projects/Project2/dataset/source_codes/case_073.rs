
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct JudgeEntryCtxsstn<'info> {
    #[account(mut)] pub tournament: Account<'info, DataAccount>,
    #[account(mut)] pub judge: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_073 {
    use super::*;

    pub fn judge_entry(ctx: Context<JudgeEntryCtxsstn>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.tournament;
        // custom logic for judge_entry
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed judge_entry logic");
        Ok(())
    }
}
