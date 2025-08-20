use anchor_lang::prelude::*;

declare_id!("Ex3000000000000000000000000000000000003");

#[program]
pub mod example3 {
    use super::*;

    // 在庫を追加し、累積追加量を記録
    pub fn add_stock(ctx: Context<AddStock>, amount: u32) -> Result<()> {
        let stock = &mut ctx.accounts.stock;      // ← initあり
        stock.level += amount;
        stock.total_added += amount;

        // しきい値超過でフラグ
        if stock.level > stock.alert_threshold {
            stock.alert = true;
        } else {
            stock.alert = false;
        }
        Ok(())
    }

    // 在庫を消費して実際に消費した量と不足数を記録
    pub fn consume_stock(ctx: Context<ConsumeStock>, want: u32) -> Result<()> {
        let st = &mut ctx.accounts.stock;         // ← initなし：既存参照のみ
        let mut consumed = 0;
        let mut i = 0;
        while i < want {
            if st.level == 0 {
                break;
            }
            st.level -= 1;
            consumed += 1;
            i += 1;
        }
        st.last_consumed = consumed;
        st.shortage = want - consumed;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddStock<'info> {
    #[account(init, payer = mgr, space = 8 + 4*4 + 1)]
    pub stock: Account<'info, StockData>,
    #[account(mut)] pub mgr: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConsumeStock<'info> {
    pub stock: Account<'info, StockData>,
    #[account(mut)] pub mgr: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StockData {
    pub level: u32,
    pub total_added: u32,
    pub alert_threshold: u32,
    pub alert: bool,
    pub last_consumed: u32,
    pub shortage: u32,
}
