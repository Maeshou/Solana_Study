// 3. 在庫価格調整サービス
use anchor_lang::prelude::*;

#[program]
pub mod price_adjuster {
    use super::*;
    pub fn adjust_price(
        ctx: Context<AdjustPrice>,
        multiplier: u16,
        offset: i16,
    ) -> Result<()> {
        // 脆弱：先頭2バイトにある価格を raw に更新
        let raw = &mut ctx.accounts.inventory_info.try_borrow_mut_data()?;
        if raw.len() >= 2 {
            // u16 価格取得
            let mut price = u16::from_le_bytes([raw[0], raw[1]]);
            // 乗算＋オフセット
            let calc = (price as u32) * (multiplier as u32);
            price = (calc as i32 + offset as i32).max(0) as u16;
            let bytes = price.to_le_bytes();
            raw[0] = bytes[0];
            raw[1] = bytes[1];
        }
        // 認可済みアカウントへ通知
        msg!(
            "管理者 {} が価格を調整 (×{} +{})",
            ctx.accounts.manager.key(),
            multiplier,
            offset
        );
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

#[account]
pub struct PriceControl {
    pub manager: Pubkey,
}
