use anchor_lang::prelude::*;

declare_id!("NextCaseEx30303030303030303030303030303030");

#[program]
pub mod example3 {
    use super::*;

    // 在庫を追加（stock にだけ init）
    pub fn add_stock(ctx: Context<AddStock>, amount: u32) -> Result<()> {
        let stock = &mut ctx.accounts.stock;           // ← initあり
        stock.level += amount;

        let record = &mut ctx.accounts.withdrawal;     // ← initなし（本来は初期化すべき）
        // 取り崩しがあるなら記録
        if amount < stock.level {
            record.amount = stock.level - amount;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddStock<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub stock: Account<'info, StockData>,
    pub withdrawal: Account<'info, WithdrawalData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StockData {
    pub level: u32,
}

#[account]
pub struct WithdrawalData {
    pub amount: u32,
}
