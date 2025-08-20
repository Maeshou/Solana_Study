use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfPENLT");

#[program]
pub mod penalty_manager {
    use super::*;

    /// ユーザーのペナルティカウントを初期化
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.user    = ctx.accounts.user.key();
        record.penalty = 0;
        msg!("Initialized penalty record for {}", record.user);
        Ok(())
    }

    /// 管理者がペナルティを加算
    pub fn add_penalty(ctx: Context<AddPenalty>, points: u8) -> Result<()> {
        require!(
            ctx.accounts.admin.is_signer,
            ErrorCode::Unauthorized
        );
        let record = &mut ctx.accounts.record;
        // wrapping_add でオーバーフローしないように
        record.penalty = record.penalty.wrapping_add(points);
        msg!(
            "Added {} penalty points to {} (total now {})",
            points,
            record.user,
            record.penalty
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    /// ユーザーペナルティ記録PDA
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 1,
        seeds = [b"penalty", user.key().as_ref()],
        bump
    )]
    pub record: Account<'info, PenaltyRecord>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// 対象ユーザー
    pub user: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPenalty<'info> {
    /// 既存のペナルティ記録PDA
    #[account(
        mut,
        seeds = [b"penalty", record.user.as_ref()],
        bump,
        has_one = user
    )]
    pub record: Account<'info, PenaltyRecord>,

    /// 管理者署名必須
    pub admin: Signer<'info>,

    /// ペナルティ対象ユーザー
    pub user: UncheckedAccount<'info>,
}

#[account]
pub struct PenaltyRecord {
    /// ユーザーPubkey
    pub user:    Pubkey,
    /// 現在のペナルティポイント
    pub penalty: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: admin signature required")]
    Unauthorized,
}
