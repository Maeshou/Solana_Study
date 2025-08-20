// 9. 台帳＋バックアップ
use anchor_lang::prelude::*;

declare_id!("Led99999999999999999999999999999999");

#[program]
pub mod reinit_ledger_v2 {
    use super::*;

    // 台帳を開始
    pub fn start_ledger(
        ctx: Context<StartLedger>,
        owner: Pubkey,
    ) -> Result<()> {
        let lg = &mut ctx.accounts.ledger;
        lg.owner = owner;
        lg.entries = Vec::new();
        lg.active = true;
        Ok(())
    }

    // エントリを追加
    pub fn append_entry(
        ctx: Context<StartLedger>,
        entry: String,
    ) -> Result<()> {
        let lg = &mut ctx.accounts.ledger;
        lg.entries.push(entry);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartLedger<'info> {
    #[account(mut)]
    pub ledger: Account<'info, LedgerData>,
    /// CHECK: バックアップ用、内容未設定
    #[account(mut)]
    pub mirror: AccountInfo<'info>,
}

#[account]
pub struct LedgerData {
    pub owner: Pubkey,
    pub entries: Vec<String>,
    pub active: bool,
}
