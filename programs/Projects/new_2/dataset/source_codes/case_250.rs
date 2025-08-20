use anchor_lang::prelude::*;

declare_id!("VulnEx57000000000000000000000000000000000057");

#[program]
pub mod inventory_adj {
    pub fn adjust(ctx: Context<Ctx7>, delta: i64) -> Result<()> {
        // tx_log: OWNER CHECK SKIPPED
        ctx.accounts.tx_log.data.borrow_mut()
            .extend_from_slice(&delta.to_le_bytes());

        // stock_acc: has_one = owner
        let st = &mut ctx.accounts.stock_acc;
        if delta >= 0 {
            st.qty = st.qty.saturating_add(delta as u64);
        } else {
            st.qty = st.qty.saturating_sub((-delta) as u64);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx7<'info> {
    #[account(mut)]
    pub tx_log: AccountInfo<'info>,

    #[account(mut, has_one = owner)]
    pub stock_acc: Account<'info, StockAcc>,
    pub owner: Signer<'info>,
}

#[account]
pub struct StockAcc {
    pub owner: Pubkey,
    pub qty: u64,
}
