
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SubmitEntryCtxyqlr<'info> {
    #[account(mut)] pub tournament: Account<'info, DataAccount>,
    #[account(mut)] pub participant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_072 {
    use super::*;

    pub fn submit_entry(ctx: Context<SubmitEntryCtxyqlr>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.tournament;
        // custom logic for submit_entry
        assert!(ctx.accounts.tournament.data > 0); acct.data -= amount;
        msg!("Executed submit_entry logic");
        Ok(())
    }
}
