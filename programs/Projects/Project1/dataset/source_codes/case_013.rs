use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf995mvTWf");

#[program]
pub mod status_tracker_003 {
    use super::*;

    // 状態を次段階に進める（ex: 0→1→2...）
    pub fn advance_status(ctx: Context<Ctx003>) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        let current = ctx.accounts.storage.data;
        require!(current < 5, CustomError::StatusOverflow); // 最大5段階と仮定

        ctx.accounts.storage.data = current + 1;
        msg!("Status advanced from {} to {}", current, current + 1);
        Ok(())
    }

    // 現在のステータスを表示
    pub fn check_status(ctx: Context<Ctx003>) -> Result<()> {
        let status = ctx.accounts.storage.data;
        msg!("Current status: {}", status);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64, // ステータス段階（0〜5）
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Status has already reached maximum")]
    StatusOverflow,
}
