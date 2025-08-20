use anchor_lang::prelude::*;

declare_id!("6AbZ8HmK9QcRn3XpYtUv2WeZiAoFxBzLsMnQ1XpQrSt");

#[program]
pub mod rewards_shift {
    use super::*;

    /// from_acc から to_acc へ報酬を移動するが、
    /// 両者が同一アカウントかどうかをチェックしていない！
    pub fn shift_rewards(
        ctx: Context<ShiftRewards>,
        reward_amount: u64,
    ) -> ProgramResult {
        let from_acc = &mut ctx.accounts.from_acc;
        let to_acc   = &mut ctx.accounts.to_acc;

        // ❌ 本来は以下のようにチェックすべき
        // require!(
        //     from_acc.key() != to_acc.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // 分岐やループなしでの単純演算
        from_acc.rewards = from_acc.rewards.checked_sub(reward_amount).unwrap();
        to_acc.rewards   = to_acc.rewards.checked_add(reward_amount).unwrap();

        msg!(
            "{} rewards shifted from {} to {}",
            reward_amount,
            from_acc.user,
            to_acc.user
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ShiftRewards<'info> {
    /// 送信元アカウント（mutable）
    #[account(mut)]
    pub from_acc: Account<'info, RewardAccount>,

    /// 送信先アカウント（mutable）
    #[account(mut)]
    pub to_acc:   Account<'info, RewardAccount>,

    /// 管理者としての署名者
    #[account(signer)]
    pub admin:    Signer<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RewardAccount {
    /// アカウント所有者
    pub user:    Pubkey,
    /// 保有報酬量
    pub rewards: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Duplicate mutable accounts used.")]
    DuplicateMutableAccount,
}
