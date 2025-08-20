use anchor_lang::prelude::*;

declare_id!("NoPushLedger555555555555555555555555555555");

#[program]
pub mod ledger_app {
    use super::*;

    pub fn init_ledger(ctx: Context<InitLedger>) -> Result<()> {
        let l = &mut ctx.accounts.ledger;
        l.count = 0;
        Ok(())
    }

    pub fn add_entry(ctx: Context<AddEntry>, note: String) -> Result<()> {
        // ledger に init がない → 任意台帳を改竄可能
        let _l = &ctx.accounts.ledger;
        // record_account を毎回 init → 再初期化できる
        let r = &mut ctx.accounts.record;
        r.note = note;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLedger<'info> {
    #[account(init, payer = creator, space = 8 + 4)]
    pub ledger: Account<'info, LedgerData>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddEntry<'info> {
    pub ledger: Account<'info, LedgerData>,
    #[account(mut, init, payer = creator, space = 8 + 256)]
    pub record: Account<'info, RecordData>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LedgerData {
    pub count: u32,
}

#[account]
pub struct RecordData {
    pub note: String,
}
