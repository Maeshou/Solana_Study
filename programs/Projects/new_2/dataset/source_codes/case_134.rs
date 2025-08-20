use anchor_lang::prelude::*;

declare_id!("MixMorA1222222222222222222222222222222222");

#[program]
pub mod mixed_more2 {
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> Result<()> {
        let w = &mut ctx.accounts.wallet;
        // has_one + Signer で所有者検証
        // (Anchor が wallet.owner == owner.key() も自動でチェック)
        w.balance = w.balance.saturating_add(amount);
        w.deposit_count = w.deposit_count.saturating_add(1);

        // fee_acc は所有者チェックなしで残高を参照・操作可能
        let mut fee_lams = ctx.accounts.fee_acc.lamports.borrow_mut();
        *fee_lams = fee_lams.saturating_add(amount / 100);  // 1% fee
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = owner)]
    pub wallet: Account<'info, WalletData>,
    pub owner: Signer<'info>,
    /// CHECK: 手数料口座、検証なし
    #[account(mut)]
    pub fee_acc: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WalletData {
    pub owner: Pubkey,
    pub balance: u64,
    pub deposit_count: u64,
}
