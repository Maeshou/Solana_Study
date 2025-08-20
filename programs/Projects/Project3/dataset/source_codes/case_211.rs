use anchor_lang::prelude::*;
declare_id!("StakeSafe11111111111111111111111111111111");

/// ステークプール情報
#[account]
pub struct StakePool {
    pub manager:    Pubkey,  // プール管理者
    pub total_stake: u64,    // 総ステーク量
}

/// ユーザーのステークアカウント
#[account]
pub struct StakeAccount {
    pub owner:      Pubkey,  // ステークしたユーザー
    pub pool:       Pubkey,  // StakePool.key()
    pub amount:     u64,     // ステーク量
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8)]
    pub pool:        Account<'info, StakePool>,
    #[account(mut)]
    pub manager:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    /// StakePool.manager == manager.key() を検証
    #[account(mut, has_one = manager)]
    pub pool:        Account<'info, StakePool>,

    /// StakeAccount.pool == pool.key()、StakeAccount.owner == staker.key() を検証
    #[account(
        init,
        payer = staker,
        space = 8 + 32 + 32 + 8,
        has_one = pool,
        has_one = owner
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub manager:     Signer<'info>,
    #[account(mut)]
    pub staker:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    /// StakeAccount.pool == pool.key()、StakeAccount.owner == staker.key() を検証
    #[account(mut, has_one = pool, has_one = owner)]
    pub stake_account: Account<'info, StakeAccount>,

    /// プールアカウント
    #[account(mut)]
    pub pool:        Account<'info, StakePool>,

    #[account(mut)]
    pub staker:      Signer<'info>,
}

#[program]
pub mod staking_safe {
    use super::*;

    /// プールを作成
    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.manager     = ctx.accounts.manager.key();
        p.total_stake = 0;
        Ok(())
    }

    /// ステークを行う
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let p  = &mut ctx.accounts.pool;
        let sa = &mut ctx.accounts.stake_account;

        // 明示的にフィールドをセット
        sa.owner = ctx.accounts.staker.key();
        sa.pool  = ctx.accounts.pool.key();
        sa.amount = amount;

        // 二重チェック
        require_keys_eq!(sa.pool, p.key(), StakeError::PoolMismatch);
        require_keys_eq!(sa.owner, ctx.accounts.staker.key(), StakeError::OwnerMismatch);

        // 総ステーク量を加算（オーバーフロー検出）
        p.total_stake = p
            .total_stake
            .checked_add(amount)
            .ok_or(StakeError::Overflow)?;
        Ok(())
    }

    /// ステークを解除
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let p  = &mut ctx.accounts.pool;
        let sa = &mut ctx.accounts.stake_account;
        let amt = sa.amount;

        // 二重チェック
        require_keys_eq!(sa.pool, p.key(), StakeError::PoolMismatch);
        require_keys_eq!(sa.owner, ctx.accounts.staker.key(), StakeError::OwnerMismatch);

        // 総ステーク量をデクリメント（オーバーフロー防止）
        p.total_stake = p
            .total_stake
            .checked_sub(amt)
            .ok_or(StakeError::Underflow)?;

        // アカウントを閉じることで lamports を返還
        Ok(())
    }
}

#[error_code]
pub enum StakeError {
    #[msg("StakeAccount.pool が StakePool に一致しません")]
    PoolMismatch,
    #[msg("StakeAccount.owner が署名者に一致しません")]
    OwnerMismatch,
    #[msg("ステーク量の加算でオーバーフローが発生しました")]
    Overflow,
    #[msg("ステーク量の減算でアンダーフローが発生しました")]
    Underflow,
}
