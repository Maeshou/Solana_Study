use anchor_lang::prelude::*;

declare_id!("OwnChkC9000000000000000000000000000000009");

#[program]
pub mod trade_execution {
    pub fn execute_trade(
        ctx: Context<Execute>,
        amount: u64,
        price: u64,
    ) -> Result<()> {
        let tx = &mut ctx.accounts.trade;
        // 属性レベルで executor を検証
        tx.amount = amount;
        tx.price  = price;
        tx.executed = true;

        // tx_log は unchecked
        let mut log = ctx.accounts.tx_log.data.borrow_mut();
        log.extend_from_slice(&amount.to_le_bytes());
        log.extend_from_slice(&price.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(mut, has_one = executor)]
    pub trade: Account<'info, TradeData>,
    pub executor: Signer<'info>,
    /// CHECK: トランザクションログ、所有者検証なし
    #[account(mut)]
    pub tx_log: AccountInfo<'info>,
}

#[account]
pub struct TradeData {
    pub executor: Pubkey,
    pub amount: u64,
    pub price: u64,
    pub executed: bool,
}
