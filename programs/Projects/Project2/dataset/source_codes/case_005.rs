// =============================================================================
// 5. Staking Pool with Multi-level Owner Checks
// =============================================================================
#[program]
pub mod secure_staking {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, reward_rate: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.admin = ctx.accounts.admin.key();
        pool.reward_rate = reward_rate;
        pool.total_staked = 0;
        pool.bump = *ctx.bumps.get("pool").unwrap();
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let user_stake = &mut ctx.accounts.user_stake;
        
        pool.total_staked += amount;
        user_stake.amount += amount;
        user_stake.last_update = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let user_stake = &mut ctx.accounts.user_stake;
        
        require!(user_stake.amount >= amount, StakingError::InsufficientStake);
        
        pool.total_staked -= amount;
        user_stake.amount -= amount;
        
        Ok(())
    }
}

#[account]
pub struct StakingPool {
    pub admin: Pubkey,
    pub reward_rate: u64,
    pub total_staked: u64,
    pub bump: u8,
}

#[account]
pub struct UserStake {
    pub user: Pubkey,
    pub pool: Pubkey,
    pub amount: u64,
    pub last_update: i64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 8 + 8 + 1,
        seeds = [b"pool", admin.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.admin.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 32 + 8 + 8 + 1,
        seeds = [b"stake", pool.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.admin.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    #[account(
        mut,
        seeds = [b"stake", pool.key().as_ref(), user.key().as_ref()],
        bump = user_stake.bump,
        constraint = user_stake.user == user.key()
    )]
    pub user_stake: Account<'info, UserStake>,
    
    pub user: Signer<'info>,
}

#[error_code]
pub enum StakingError {
    #[msg("Insufficient stake amount")]
    InsufficientStake,
}