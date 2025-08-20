use anchor_lang::prelude::*;

declare_id!("OwnChkE5000000000000000000000000000000006");

#[program]
pub mod payment {
    pub fn process_payment(
        ctx: Context<Payment>,
        amount: u64,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.account;
        // 属性レベルで payer を検証
        acct.balance = acct.balance.saturating_sub(amount);
        acct.tx_count = acct.tx_count.saturating_add(1);

        // record_cache は unchecked
        let mut cache = ctx.accounts.record_cache.data.borrow_mut();
        cache.extend_from_slice(&amount.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut, has_one = payer)]
    pub account: Account<'info, PaymentAccount>,
    pub payer: Signer<'info>,
    /// CHECK: 記録キャッシュ、所有者検証なし
    #[account(mut)]
    pub record_cache: AccountInfo<'info>,
}

#[account]
pub struct PaymentAccount {
    pub payer: Pubkey,
    pub balance: u64,
    pub tx_count: u64,
}
