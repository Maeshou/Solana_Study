use anchor_lang::prelude::*;

declare_id!("99999999999999999999999999999999");

#[program]
pub mod init_ledger {
    use super::*;

    pub fn start_ledger(
        ctx: Context<StartLedger>,
        owner: Pubkey,
    ) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger;
        ledger.owner = owner;
        ledger.entries = Vec::new();
        ledger.active = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartLedger<'info> {
    #[account(mut)]
    pub ledger: Account<'info, LedgerData>,
    pub initiator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LedgerData {
    pub owner: Pubkey,
    pub entries: Vec<String>,
    pub active: bool,
}
