use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct BudgetManager(pub u8, pub Vec<(u8, u64)>); // (bump, Vec<(category_id, spent)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVC");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of categories reached")]
    MaxCategoriesReached,
    #[msg("Category not found")]
    CategoryNotFound,
}

#[program]
pub mod budget_manager {
    use super::*;

    const MAX_CATEGORIES: usize = 16;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("manager").unwrap();
        ctx.accounts.manager.0 = b;
        Ok(())
    }

    /// カテゴリ追加：件数制限チェック＋初期ゼロで追加
    pub fn add_category(ctx: Context<Modify>, category_id: u8) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        if list.len() >= MAX_CATEGORIES {
            return err!(ErrorCode::MaxCategoriesReached);
        }
        list.push((category_id, 0));
        Ok(())
    }

    /// 支出記録：該当カテゴリを探索し、金額を加算
    pub fn record_expense(ctx: Context<Modify>, category_id: u8, amount: u64) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == category_id {
                entry.1 = entry.1.wrapping_add(amount);
                found = true;
            }
        }
        if !found {
            return err!(ErrorCode::CategoryNotFound);
        }
        Ok(())
    }

    /// カテゴリ削除：該当カテゴリを一括除去
    pub fn purge_category(ctx: Context<Modify>, category_id: u8) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        list.retain(|&(cid, _)| {
            if cid == category_id {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 総支出報告：すべての金額を合計してログ出力
    pub fn total_spent(ctx: Context<Modify>) -> Result<()> {
        let list = &ctx.accounts.manager.1;
        let mut sum = 0u64;
        for &(_, spent) in list.iter() {
            sum = sum.wrapping_add(spent);
        }
        msg!("Total spent: {}", sum);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"manager", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max16*(1+8)
        space = 8 + 1 + 4 + 16 * (1 + 8)
    )]
    pub manager:   Account<'info, BudgetManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"manager", authority.key().as_ref()],
        bump = manager.0,
    )]
    pub manager:   Account<'info, BudgetManager>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
