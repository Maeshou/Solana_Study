use anchor_lang::prelude::*;

declare_id!("Invn111111111111111111111111111111111111");

#[program]
pub mod inventory_manager {
    /// 新しい品目を追加
    pub fn add_item(
        ctx: Context<AddItem>,
        label: String,
        quantity: u32,
    ) -> Result<()> {
        // ラベル長チェック
        if label.len() > 64 {
            return Err(ErrorCode::LabelTooLong.into());
        }
        // 初期数量チェック：０以上
        // (u32 型なので負にはならない)

        let item = &mut ctx.accounts.item;
        item.owner    = ctx.accounts.user.key();  
        item.label    = label;
        item.quantity = quantity;
        Ok(())
    }

    /// 所持数を増減
    pub fn adjust_quantity(
        ctx: Context<AdjustQuantity>,
        delta: i32,
    ) -> Result<()> {
        let item = &mut ctx.accounts.item;
        let user = ctx.accounts.user.key();

        // 所有者チェック
        if item.owner != user {
            return Err(ErrorCode::Unauthorized.into());
        }
        // 新しい数量計算
        let new_qty = item.quantity as i32 + delta;
        if new_qty < 0 {
            return Err(ErrorCode::InsufficientQuantity.into());
        }
        item.quantity = new_qty as u32;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddItem<'info> {
    /// 同一アカウントを二度初期化できない（Reinit Attack 防止）
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4)]
    pub item:           Account<'info, InventoryItem>,

    /// 操作を行うユーザー（署名必須）
    #[account(mut)]
    pub user:           Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdjustQuantity<'info> {
    /// Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub item:           Account<'info, InventoryItem>,

    /// 実際に署名したユーザー（Signer Authorization）
    pub user:           Signer<'info>,
}

#[account]
pub struct InventoryItem {
    /// この品目を操作できるユーザー
    pub owner:    Pubkey,
    /// 品目名（最大64文字）
    pub label:    String,
    /// 在庫数
    pub quantity: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("ラベルが長すぎます")]
    LabelTooLong,
    #[msg("在庫数が不足しています")]
    InsufficientQuantity,
}
