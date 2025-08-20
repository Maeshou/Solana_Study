use anchor_lang::prelude::*;

declare_id!("SavG11111111111111111111111111111111111");

#[program]
pub mod savings_goals {
    /// 新しい貯蓄目標を作成
    pub fn create_goal(
        ctx: Context<CreateGoal>,
        target: u64,
        description: String,
    ) -> Result<()> {
        // 目標金額は 1 以上
        if target == 0 {
            return Err(ErrorCode::InvalidTarget.into());
        }
        // 説明は最大128文字
        if description.len() > 128 {
            return Err(ErrorCode::DescriptionTooLong.into());
        }

        let goal = &mut ctx.accounts.goal;
        goal.owner       = ctx.accounts.user.key();
        goal.target      = target;
        goal.saved       = 0;
        goal.description = description;
        Ok(())
    }

    /// 貯蓄を追加
    pub fn add_savings(ctx: Context<AddSavings>, amount: u64) -> Result<()> {
        // 金額は 1 以上
        if amount == 0 {
            return Err(ErrorCode::InvalidAmount.into());
        }

        let goal = &mut ctx.accounts.goal;
        // 所有者チェック
        if goal.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        // オーバーフロー検証＋目標超過チェック
        let new_saved = goal
            .saved
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        if new_saved > goal.target {
            return Err(ErrorCode::TargetExceeded.into());
        }

        goal.saved = new_saved;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGoal<'info> {
    /// 同一アカウントを二度初期化できない（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 4 + 128)]
    pub goal:        Account<'info, GoalAccount>,

    /// 貯蓄目標作成者（署名必須）
    #[account(mut)]
    pub user:        Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddSavings<'info> {
    /// 型チェック＆所有者チェック (Owner Check / Type Cosplay)
    #[account(mut)]
    pub goal:        Account<'info, GoalAccount>,

    /// 貯蓄を追加するユーザー（Signer Authorization）
    pub user:        Signer<'info>,
}

#[account]
pub struct GoalAccount {
    /// この目標を操作できるユーザー
    pub owner:       Pubkey,
    /// 目標金額 (Lamports 単位)
    pub target:      u64,
    /// 現在の貯蓄額
    pub saved:       u64,
    /// 目標の説明（最大128文字）
    pub description: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("目標金額は必ず 1 以上である必要があります")]
    InvalidTarget,
    #[msg("追加額は必ず 1 以上である必要があります")]
    InvalidAmount,
    #[msg("貯蓄額が大きすぎてオーバーフローしました")]
    Overflow,
    #[msg("貯蓄額が目標を超過しました")]
    TargetExceeded,
    #[msg("説明が長すぎます")]
    DescriptionTooLong,
}
