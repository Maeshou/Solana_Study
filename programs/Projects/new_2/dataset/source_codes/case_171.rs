use anchor_lang::prelude::*;

declare_id!("OwnChkC3000000000000000000000000000000003");

#[program]
pub mod reward_claim {
    pub fn claim(
        ctx: Context<ClaimReward>,
        amount: u64,
    ) -> Result<()> {
        let r = &mut ctx.accounts.reward;
        // 属性レベルで recipient を検証
        r.claimed += amount;

        // backup_acc は unchecked
        let mut data = ctx.accounts.backup_acc.data.borrow_mut();
        data.extend_from_slice(&amount.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut, has_one = recipient)]
    pub reward: Account<'info, RewardData>,
    pub recipient: Signer<'info>,
    /// CHECK: バックアップ用アカウント、所有者検証なし
    #[account(mut)]
    pub backup_acc: AccountInfo<'info>,
}

#[account]
pub struct RewardData {
    pub recipient: Pubkey,
    pub claimed: u64,
}
