use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgAchvSvc01");

#[program]
pub mod achievement_service {
    use super::*;

    /// 新しい実績をアンロックするが、
    /// achievement_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn unlock_achievement(ctx: Context<ModifyAchievement>, achievement_id: u8) -> Result<()> {
        let acct = &mut ctx.accounts.achievement_account;
        mark_unlocked(acct, achievement_id);
        Ok(())
    }

    /// アンロックした実績の報酬を請求するが、
    /// achievement_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        let acct = &mut ctx.accounts.achievement_account;
        let vault = &mut ctx.accounts.reward_vault.to_account_info();
        let user  = &mut ctx.accounts.user.to_account_info();
        let amount = ctx.accounts.config.reward_amount;

        transfer_reward(vault, user, amount)?;
        record_claim(acct);
        Ok(())
    }
}

/// 実績をアンロックし、アンロック回数を更新するヘルパー
fn mark_unlocked(acct: &mut AchievementAccount, id: u8) {
    acct.last_unlocked = id;
    acct.unlock_count = acct.unlock_count.saturating_add(1);
}

/// Lamports を vault から user へ直接移動するヘルパー
fn transfer_reward(from: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<()> {
    **from.lamports.borrow_mut() = from.lamports()
        .checked_sub(amount)
        .unwrap();
    **to.lamports.borrow_mut() = to.lamports()
        .checked_add(amount)
        .unwrap();
    Ok(())
}

/// 報酬請求回数を更新するヘルパー
fn record_claim(acct: &mut AchievementAccount) {
    acct.claim_count = acct.claim_count.saturating_add(1);
}

#[derive(Accounts)]
pub struct ModifyAchievement<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] で所有者照合を行うべき
    pub achievement_account: Account<'info, AchievementAccount>,

    /// 実績をアンロックするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] で所有者照合を行うべき
    pub achievement_account: Account<'info, AchievementAccount>,

    /// 報酬を保管するプールアカウント
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,

    /// 報酬を受け取るユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 報酬量設定を保持するアカウント
    pub config: Account<'info, AchievementConfig>,
}

#[account]
pub struct AchievementAccount {
    /// 本来この実績を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 最後にアンロックされた実績の ID
    pub last_unlocked: u8,
    /// アンロック操作の累計回数
    pub unlock_count: u64,
    /// 報酬請求操作の累計回数
    pub claim_count: u64,
}

#[account]
pub struct AchievementConfig {
    /// 1 実績あたりの報酬 Lamports
    pub reward_amount: u64,
}
