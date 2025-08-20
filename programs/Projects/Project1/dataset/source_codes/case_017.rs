use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf998mvTWf");

#[program]
pub mod limited_counter_lock_003 {
    use super::*;

    // 操作カウントを上限付きで進める
    pub fn increment_with_limit(ctx: Context<Ctx003>, max_count: u64) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);

        let current = ctx.accounts.storage.data;
        require!(current < max_count, CustomError::LimitReached);

        ctx.accounts.storage.data = current + 1;
        msg!(
            "Count incremented: {} -> {} (Limit = {})",
            current,
            current + 1,
            max_count
        );
        Ok(())
    }

    // 現在の状態を確認
    pub fn check_count(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Current count: {}", ctx.accounts.storage.data);
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
    pub data: u64, // 現在のカウント数
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Maximum allowed count reached")]
    LimitReached,
}
