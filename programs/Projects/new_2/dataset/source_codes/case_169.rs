use anchor_lang::prelude::*;

declare_id!("OwnChkC1000000000000000000000000000000001");

#[program]
pub mod treasury_deposit {
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> Result<()> {
        let t = &mut ctx.accounts.treasury;
        // 属性レベルで treasurer を検証
        t.balance = t.balance.saturating_add(amount);
        t.deposit_count = t.deposit_count.saturating_add(1);

        // audit_storage は unchecked
        ctx.accounts.audit_storage.data.borrow_mut().extend_from_slice(&amount.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = treasurer)]
    pub treasury: Account<'info, TreasuryData>,
    pub treasurer: Signer<'info>,
    /// CHECK: 監査用ストレージ、所有者検証なし
    #[account(mut)]
    pub audit_storage: AccountInfo<'info>,
}

#[account]
pub struct TreasuryData {
    pub treasurer: Pubkey,
    pub balance: u64,
    pub deposit_count: u64,
}
