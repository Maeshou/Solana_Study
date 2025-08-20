use anchor_lang::prelude::*;

declare_id!("ExTr111111111111111111111111111111111111");

#[program]
pub mod expense_tracker {
    /// 支出を記録
    pub fn add_expense(
        ctx: Context<AddExpense>,
        timestamp: i64,
        amount: u64,
        category: String,
        description: String,
    ) -> Result<()> {
        // 金額は必ず1以上
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        // カテゴリ長チェック
        if category.len() > 32 {
            return Err(ErrorCode::CategoryTooLong.into());
        }
        // 説明長チェック
        if description.len() > 128 {
            return Err(ErrorCode::DescriptionTooLong.into());
        }

        let exp = &mut ctx.accounts.expense;
        exp.owner       = ctx.accounts.user.key();
        exp.timestamp   = timestamp;
        exp.amount      = amount;
        exp.category    = category;
        exp.description = description;
        Ok(())
    }

    /// 記録済み支出を編集
    pub fn edit_expense(
        ctx: Context<EditExpense>,
        new_timestamp: i64,
        new_amount: u64,
        new_category: String,
        new_description: String,
    ) -> Result<()> {
        // 金額チェック
        if new_amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }
        // カテゴリ長チェック
        if new_category.len() > 32 {
            return Err(ErrorCode::CategoryTooLong.into());
        }
        // 説明長チェック
        if new_description.len() > 128 {
            return Err(ErrorCode::DescriptionTooLong.into());
        }

        let exp = &mut ctx.accounts.expense;
        // 所有者チェック
        if exp.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        exp.timestamp   = new_timestamp;
        exp.amount      = new_amount;
        exp.category    = new_category;
        exp.description = new_description;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddExpense<'info> {
    /// 同じアカウントを二度初期化できない（Reinit Attack 防止）
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 4 + 32 + 4 + 128)]
    pub expense:     Account<'info, Expense>,

    /// 操作を行うユーザー（署名必須）
    #[account(mut)]
    pub user:        Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditExpense<'info> {
    /// Anchor の Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub expense:     Account<'info, Expense>,

    /// 実際に署名したユーザー
    pub user:        Signer<'info>,
}

#[account]
pub struct Expense {
    /// この記録を操作できるユーザー
    pub owner:       Pubkey,
    /// タイムスタンプ (UNIX)
    pub timestamp:   i64,
    /// 金額 (Lamports 単位)
    pub amount:      u64,
    /// 支出カテゴリ（最大32文字）
    pub category:    String,
    /// 詳細説明（最大128文字）
    pub description: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("金額は1以上である必要があります")]
    InvalidAmount,
    #[msg("カテゴリ名が長すぎます")]
    CategoryTooLong,
    #[msg("説明が長すぎます")]
    DescriptionTooLong,
}
