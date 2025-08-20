use anchor_lang::prelude::*;

declare_id!("VulnEx100000000000000000000000000000000001");

#[program]
pub mod vuln_example1 {
    pub fn deposit(ctx: Context<Ctx1>, amount: u64) -> Result<()> {
        // audit_log は所有者チェックなし
        ctx.accounts.audit_log.data.borrow_mut().extend_from_slice(&amount.to_le_bytes());
        // treasury は正しく treasurer 検証済み
        let t = &mut ctx.accounts.treasury;
        t.balance = t.balance.saturating_add(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx1<'info> {
    /// CHECK: 監査ログ、所有者検証なし
    #[account(mut)]
    pub audit_log: AccountInfo<'info>,

    #[account(mut, has_one = treasurer)]
    pub treasury: Account<'info, TreasuryData>,
    pub treasurer: Signer<'info>,
}

#[account]
pub struct TreasuryData {
    pub treasurer: Pubkey,
    pub balance: u64,
}
