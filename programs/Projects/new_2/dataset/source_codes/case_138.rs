use anchor_lang::prelude::*;

declare_id!("MixMorA5666666666666666666666666666666666");

#[program]
pub mod mixed_more6 {
    pub fn start_stake(ctx: Context<Start>, amount: u64) -> Result<()> {
        let s = &mut ctx.accounts.stake;
        // has_one + Signer で staker チェック
        s.amount = s.amount.saturating_add(amount);
        s.starts = s.starts.saturating_add(1);

        // stake_log は未検証
        let mut bs = ctx.accounts.stake_log.data.borrow_mut();
        bs.extend_from_slice(&amount.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(mut, has_one = staker)]
    pub stake: Account<'info, StakeRecord>,
    pub staker: Signer<'info>,
    /// CHECK: ログ用アカウント
    #[account(mut)]
    pub stake_log: AccountInfo<'info>,
}

#[account]
pub struct StakeRecord {
    pub staker: Pubkey,
    pub amount: u64,
    pub starts: u64,
}
