use anchor_lang::prelude::*;

declare_id!("Loyal111111111111111111111111111111111111");

#[program]
pub mod loyalty_program {
    /// ロイヤリティアカウントを作成
    pub fn create_account(ctx: Context<CreateAccount>) -> Result<()> {
        let acct = &mut ctx.accounts.loyalty;
        acct.owner  = ctx.accounts.user.key();
        acct.points = 0;
        Ok(())
    }

    /// ポイント獲得
    pub fn earn_points(ctx: Context<ModifyAccount>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.loyalty;
        let signer = ctx.accounts.user.key();

        // 所有者チェック
        if acct.owner != signer {
            return Err(ErrorCode::Unauthorized.into());
        }
        // オーバーフロー検証
        let new_total = acct.points.checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        acct.points = new_total;
        Ok(())
    }

    /// ポイント利用
    pub fn redeem_points(ctx: Context<ModifyAccount>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.loyalty;
        let signer = ctx.accounts.user.key();

        // 所有者チェック
        if acct.owner != signer {
            return Err(ErrorCode::Unauthorized.into());
        }
        // 残高不足チェック
        if acct.points < amount {
            return Err(ErrorCode::InsufficientPoints.into());
        }
        // アンダーフロー検証
        let new_total = acct.points.checked_sub(amount)
            .ok_or(ErrorCode::Underflow)?;
        acct.points = new_total;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    /// Reinit Attack 防止：二度同じアカウントを初期化できない
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub loyalty: Account<'info, LoyaltyAccount>,

    /// 実際に署名したユーザー
    #[account(mut)]
    pub user:    Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyAccount<'info> {
    /// Owner Check & Type Cosplay
    #[account(mut)]
    pub loyalty: Account<'info, LoyaltyAccount>,

    /// 実際に署名したユーザー
    pub user:    Signer<'info>,
}

#[account]
pub struct LoyaltyAccount {
    /// 操作を許可されたユーザー
    pub owner:  Pubkey,
    /// 現在のポイント残高
    pub points: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("オーバーフローしました")]
    Overflow,
    #[msg("アンダーフローしました")]
    Underflow,
    #[msg("ポイントが不足しています")]
    InsufficientPoints,
}
