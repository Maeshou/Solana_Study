use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfCLERP");

#[program]
pub mod penalty_clearing {
    use super::*;

    /// 管理者がユーザーにペナルティを課す機能（前提：既存のペナルティレコードあり）
    pub fn add_penalty(ctx: Context<AddPenalty>, points: u8) -> Result<()> {
        require!(
            ctx.accounts.admin.is_signer,
            ErrorCode::Unauthorized
        );
        let rec = &mut ctx.accounts.record;
        rec.penalty = rec.penalty.wrapping_add(points);
        msg!(
            "Penalty {}pts added to {} (now {}).",
            points,
            rec.user,
            rec.penalty
        );
        Ok(())
    }

    /// ユーザーがペナルティを支払って復活する機能
    pub fn pay_penalty(ctx: Context<PayPenalty>) -> Result<()> {
        require!(
            ctx.accounts.user.is_signer,
            ErrorCode::Unauthorized
        );
        let rec = &mut ctx.accounts.record;
        // ここで実際のトークン送金や lamports 送金 CPI を入れても良い
        // （このサンプルではポイントをリセットするのみ）
        rec.penalty = 0;
        msg!("Penalty cleared for {}. Account reactivated.", rec.user);
        Ok(())
    }
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
    #[account()]
    pub user: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct PayPenalty<'info> {
    /// 既存のペナルティ記録PDA
    #[account(
        mut,
        seeds = [b"penalty", record.user.as_ref()],
        bump,
        has_one = user
    )]
    pub record: Account<'info, PenaltyRecord>,

    /// ペナルティ支払いを行うユーザー
    pub user: Signer<'info>,
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
    #[msg("Unauthorized: signature required")]
    Unauthorized,
}
