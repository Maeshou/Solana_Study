use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpJ1K2L3M4N5O6P7Q8R9S0T1U2V3W4");

#[program]
pub mod points_manager {
    use super::*;

    /// ポイントアカウントを初期化
    pub fn initialize(
        ctx: Context<InitializePoints>,
        bump: u8,
    ) -> ProgramResult {
        let account = &mut ctx.accounts.points_account;
        account.owner = *ctx.accounts.user.key;
        account.bump = bump;
        account.points = 0;
        Ok(())
    }

    /// ポイントを付与
    pub fn award(
        ctx: Context<ModifyPoints>,
        amount: u64,
    ) -> ProgramResult {
        let account = &mut ctx.accounts.points_account;
        account.points = account.points.checked_add(amount).ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    /// ポイントを使用
    pub fn redeem(
        ctx: Context<ModifyPoints>,
        amount: u64,
    ) -> ProgramResult {
        let account = &mut ctx.accounts.points_account;
        account.points = account.points.checked_sub(amount).ok_or(ErrorCode::InsufficientPoints)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializePoints<'info> {
    #[account(
        init,
        seeds = [b"points", user.key().as_ref()],
        bump = bump,
        payer = user,
        space = 8 + 32 + 1 + 8,
    )]
    pub points_account: Account<'info, PointsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ModifyPoints<'info> {
    #[account(
        mut,
        seeds = [b"points", points_account.owner.as_ref()],
        bump = points_account.bump,
        has_one = owner,
    )]
    pub points_account: Account<'info, PointsAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct PointsAccount {
    pub owner: Pubkey,
    pub bump: u8,
    pub points: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Arithmetic overflow.")]
    Overflow,
    #[msg("Insufficient points.")]
    InsufficientPoints,
}
