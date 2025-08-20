use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf996mvTWf");

#[program]
pub mod action_counter_003 {
    use super::*;

    // 操作回数を 1 増やす
    pub fn increment_count(ctx: Context<Ctx003>) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);
        ctx.accounts.storage.data = ctx.accounts.storage.data.checked_add(1).unwrap();
        msg!("Action count incremented. Total: {}", ctx.accounts.storage.data);
        Ok(())
    }

    // 管理者がリセット（0に戻す）
    pub fn reset_count(ctx: Context<Ctx003>) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);
        ctx.accounts.storage.data = 0;
        msg!("Action count has been reset.");
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
    pub data: u64, // 行動回数
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
}
