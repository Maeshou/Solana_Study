use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLiquidity001");

#[program]
pub mod liquidity_pool_service {
    use super::*;

    /// ユーザーがプールに流動性を預けるが、
    /// pool_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn deposit_liquidity(ctx: Context<ModifyPool>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool_account;
        let user_bal = &mut ctx.accounts.user_balance;

        // 1. ユーザー残高から差し引く
        debit_user(user_bal, amount)?;
        // 2. プール残高へ加算
        credit_pool(pool, amount);

        Ok(())
    }

    /// ユーザーがプールから流動性を引き出すが、
    /// pool_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn withdraw_liquidity(ctx: Context<ModifyPool>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool_account;
        let user_bal = &mut ctx.accounts.user_balance;

        // 1. プール残高から減算
        debit_pool(pool, amount)?;
        // 2. ユーザー残高へ加算
        credit_user(user_bal, amount);

        Ok(())
    }
}

/// ユーザー残高を減らすヘルパー
fn debit_user(user_bal: &mut BalanceAccount, amount: u64) -> Result<()> {
    user_bal.balance = user_bal.balance.checked_sub(amount).unwrap();
    user_bal.debit_count = user_bal.debit_count.saturating_add(1);
    Ok(())
}

/// ユーザー残高を増やすヘルパー
fn credit_user(user_bal: &mut BalanceAccount, amount: u64) {
    user_bal.balance = user_bal.balance.saturating_add(amount);
    user_bal.credit_count = user_bal.credit_count.saturating_add(1);
}

/// プール残高を減らすヘルパー
fn debit_pool(pool: &mut PoolAccount, amount: u64) -> Result<()> {
    pool.total_liquidity = pool.total_liquidity.checked_sub(amount).unwrap();
    pool.withdraw_count = pool.withdraw_count.saturating_add(1);
    Ok(())
}

/// プール残高を増やすヘルパー
fn credit_pool(pool: &mut PoolAccount, amount: u64) {
    pool.total_liquidity = pool.total_liquidity.saturating_add(amount);
    pool.deposit_count = pool.deposit_count.saturating_add(1);
}

#[derive(Accounts)]
pub struct ModifyPool<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub pool_account: Account<'info, PoolAccount>,

    /// ユーザーの残高アカウント（流動性操作対象）
    #[account(mut)]
    pub user_balance: Account<'info, BalanceAccount>,

    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct PoolAccount {
    /// 本来このプールを管理するべきユーザーの Pubkey
    pub owner: Pubkey,

    /// プールに預けられた総流動性
    pub total_liquidity: u64,

    /// 累計入金回数
    pub deposit_count: u64,

    /// 累計出金回数
    pub withdraw_count: u64,
}

#[account]
pub struct BalanceAccount {
    /// 本来この残高を所有するべきユーザーの Pubkey
    pub owner: Pubkey,

    /// ユーザーの保有残高
    pub balance: u64,

    /// 預け入れ（デビット）操作回数
    pub debit_count: u64,

    /// 引き出し（クレジット）操作回数
    pub credit_count: u64,
}
