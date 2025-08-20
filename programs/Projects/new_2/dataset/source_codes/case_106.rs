use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("VulnStake7777777777777777777777777777777777");

#[program]
pub mod vuln_stake {
    pub fn distribute(ctx: Context<Dist>) -> Result<()> {
        // pool.authority 未検証
        let pool = &mut ctx.accounts.pool;
        for (&user, &staked) in pool.stakes.iter() {
            let reward = staked / 10;
            **ctx.accounts.user_accounts[0].lamports.borrow_mut() += reward;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Dist<'info> {
    #[account(mut)]
    pub pool: Account<'info, StakePool>,
    /// CHECK: ユーザ一覧を適当に受け取るが権限チェックなし
    #[account(mut)]
    pub user_accounts: Vec<AccountInfo<'info>>,
}

#[account]
pub struct StakePool {
    pub authority: Pubkey,
    pub stakes: BTreeMap<Pubkey, u64>,
}
