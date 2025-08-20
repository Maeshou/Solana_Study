use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfPNLTOKEN");

#[program]
pub mod penalty_earn {
    use super::*;

    /// ユーザープロファイルを初期化。初期レートは100/100 = 100%。
    pub fn register_user(
        ctx: Context<RegisterUser>,
    ) -> Result<()> {
        let prof = &mut ctx.accounts.profile;
        prof.user       = ctx.accounts.user.key();
        prof.admin      = ctx.accounts.admin.key();
        prof.rate_num   = 100;
        prof.rate_den   = 100;
        prof.balance    = 0;
        msg!("Registered {} with 100% earning rate", prof.user);
        Ok(())
    }

    /// 管理者がユーザーにペナルティを課し、earning rate の分子を減少させる。
    pub fn penalize_user(
        ctx: Context<PenalizeUser>,
        penalty: u64,
    ) -> Result<()> {
        // has_one + signer で admin チェック済
        let prof = &mut ctx.accounts.profile;
        // レートの分子を下限 1 以上に制限
        prof.rate_num = prof.rate_num.saturating_sub(penalty).max(1);
        msg!("Penalized {}: new rate {}/{}", prof.user, prof.rate_num, prof.rate_den);
        Ok(())
    }

    /// ユーザーが基本報酬を獲得。ペナルティに応じて実際の付与量を調整する。
    pub fn earn(
        ctx: Context<Earn>,
        base_amount: u64,
    ) -> Result<()> {
        let prof = &mut ctx.accounts.profile;
        // adjusted = base_amount * rate_num / rate_den
        let adjusted = base_amount
            .checked_mul(prof.rate_num).unwrap()
            .checked_div(prof.rate_den).unwrap();
        prof.balance = prof.balance.checked_add(adjusted).unwrap();
        msg!("{} earned {} (base {})", prof.user, adjusted, base_amount);
        Ok(())
    }
}

#[account]
pub struct Profile {
    pub user:      Pubkey,
    pub admin:     Pubkey,
    pub rate_num:  u64,
    pub rate_den:  u64,
    pub balance:   u64,
    pub bump:      u8,
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32*2 + 8*3 + 1,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile:       Account<'info, Profile>,
    #[account(mut)]
    pub user:          Signer<'info>,
    /// プロファイル登録の権限を持つ管理者
    pub admin:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PenalizeUser<'info> {
    #[account(
        mut,
        seeds = [b"profile", profile.user.as_ref()],
        bump = profile.bump,
        has_one = admin @ ErrorCode::Unauthorized
    )]
    pub profile:  Account<'info, Profile>,
    #[account(mut, signer)]
    pub admin:    AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Earn<'info> {
    #[account(
        mut,
        seeds = [b"profile", profile.user.as_ref()],
        bump = profile.bump,
        has_one = user @ ErrorCode::Unauthorized
    )]
    pub profile:  Account<'info, Profile>,
    #[account(signer)]
    pub user:     AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
