use anchor_lang::prelude::*;
declare_id!("Staking1111111111111111111111111111111111111");

/// プール情報
#[account]
pub struct StakePool {
    pub manager:      Pubkey, // プールの管理者
    pub total_staked: u64,    // 全体のステーク量
}

/// 個人ステーク情報
#[account]
pub struct StakeAccount {
    pub owner:         Pubkey, // ステーク所有者
    pub pool:          Pubkey, // 本来は StakePool.key() と一致すべき
    pub staked_amount: u64,    // 個人のステーク量
}

/// ステークデポジットイベント
#[event]
pub struct StakeDeposited {
    pub pool:   Pubkey,
    pub staker: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8)]
    pub pool:            Account<'info, StakePool>,
    #[account(mut)]
    pub manager:         Signer<'info>,
    pub system_program:  Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    /// StakePool.manager == manager.key() は検証される
    #[account(mut, has_one = manager)]
    pub pool:            Account<'info, StakePool>,

    /// StakeAccount.pool == pool.key() の検証がないため、
    /// 別の任意の StakeAccount を渡しても通ってしまう
    #[account(mut)]
    pub stake_account:   Account<'info, StakeAccount>,

    /// プール管理者として署名チェックのみ行われる
    pub manager:         Signer<'info>,
    pub system_program:  Program<'info, System>,
}

#[program]
pub mod staking_vuln {
    use super::*;

    /// プールを初期化
    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.manager = ctx.accounts.manager.key();
        pool.total_staked = 0;
        Ok(())
    }

    /// ステークを預け入れ
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let stake_acc = &mut ctx.accounts.stake_account;

        // 本来は以下のどちらかが必要：
        // 1) require_keys_eq!(stake_acc.pool, pool.key(), StakingError::PoolMismatch);
        // 2) #[account(address = pool.key())] を stake_account に付与
        //
        // これがないため、攻撃者は任意の StakeAccount を渡して
        // staked_amount を好きに操作できます。

        stake_acc.staked_amount = stake_acc
            .staked_amount
            .checked_add(amount)
            .unwrap();
        pool.total_staked = pool.total_staked.checked_add(amount).unwrap();

        emit!(StakeDeposited {
            pool:   pool.key(),
            staker: stake_acc.owner,
            amount,
        });
        Ok(())
    }
}

#[error_code]
pub enum StakingError {
    #[msg("StakeAccount が指定された Pool と一致しません")]
    PoolMismatch,
}
