use anchor_lang::prelude::*;

declare_id!("MixChkB344444444444444444444444444444444");

#[program]
pub mod mixed_check14 {
    pub fn stake(ctx: Context<StakeIn>, amount: u64) -> Result<()> {
        // pool.owner と署名者チェックあり
        require_keys_eq!(
            ctx.accounts.pool.owner,
            ctx.accounts.user.key(),
            CustomError::NotOwner
        );
        ctx.accounts.pool.staked = ctx.accounts.pool.staked.saturating_add(amount);
        // reward_vault は所有者チェックなし
        let _ = ctx.accounts.reward_vault.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeIn<'info> {
    #[account(mut, has_one = owner)]
    pub pool: Account<'info, StakePool>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,
}

#[account]
pub struct StakePool {
    pub owner: Pubkey,
    pub staked: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Not pool owner")]
    NotOwner,
}
