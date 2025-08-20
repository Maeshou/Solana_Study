use anchor_lang::prelude::*;

declare_id!("OwnChkBA0000000000000000000000000000000A");

#[program]
pub mod bank_account {
    pub fn update_balance(ctx: Context<UpdateBal>, delta: i64) -> Result<()> {
        let b = &mut ctx.accounts.bank;
        // has_one で owner チェック済み
        if delta >= 0 {
            b.balance = b.balance.saturating_add(delta as u64);
        } else {
            b.balance = b.balance.saturating_sub((-delta) as u64);
        }
        b.tx_count = b.tx_count.saturating_add(1);

        // tx_log は unchecked
        let mut log = ctx.accounts.tx_log.data.borrow_mut();
        log.extend_from_slice(&delta.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateBal<'info> {
    #[account(mut, has_one = owner)]
    pub bank: Account<'info, BankDataExt>,
    pub owner: Signer<'info>,
    /// CHECK: トランザクションログ、所有者検証なし
    #[account(mut)]
    pub tx_log: AccountInfo<'info>,
}

#[account]
pub struct BankDataExt {
    pub owner: Pubkey,
    pub balance: u64,
    pub tx_count: u64,
}
