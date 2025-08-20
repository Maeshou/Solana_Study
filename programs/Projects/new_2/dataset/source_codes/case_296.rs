// 3. 在庫価格管理モジュール
use anchor_lang::prelude::*;

#[program]
pub mod price_manager {
    use super::*;
    // 価格を乗算＋オフセットで更新
    pub fn adjust_price(ctx: Context<AdjustPrice>, mul: u16, off: i16) -> Result<()> {
        let raw = &mut ctx.accounts.inventory_info.try_borrow_mut_data()?;
        if raw.len() >= 2 {
            let mut p = u16::from_le_bytes([raw[0], raw[1]]);
            let calc = (p as u32) * (mul as u32);
            p = (calc as i32 + off as i32).max(0) as u16;
            let b = p.to_le_bytes();
            raw[0] = b[0]; raw[1] = b[1];
        }
        msg!("管理者 {} が価格調整 (×{} +{})", ctx.accounts.manager.key(), mul, off);
        Ok(())
    }
    // 価格を過去値に復元（単純に先頭2バイトを固定値に）
    pub fn restore_price(ctx: Context<RestorePrice>) -> Result<()> {
        let raw = &mut ctx.accounts.inventory_info.try_borrow_mut_data()?;
        if raw.len() >= 2 {
            raw[0] = 0x10; raw[1] = 0x00;
        }
        msg!("管理者 {} が価格復元", ctx.accounts.manager.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdjustPrice<'info> {
    /// CHECK: 在庫情報（検証なし）
    pub inventory_info: AccountInfo<'info>,
    #[account(mut, has_one = manager)]
    pub price_ctrl: Account<'info, PriceControl>,
    pub manager: Signer<'info>,
}

#[derive(Accounts)]
pub struct RestorePrice<'info> {
    /// CHECK: 在庫情報（検証なし）
    pub inventory_info: AccountInfo<'info>,
    #[account(has_one = manager)]
    pub price_ctrl: Account<'info, PriceControl>,
    pub manager: Signer<'info>,
}

#[account]
pub struct PriceControl {
    pub manager: Pubkey,
}
