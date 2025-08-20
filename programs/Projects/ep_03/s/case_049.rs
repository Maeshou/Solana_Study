use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSeaRwdSv01");

#[program]
pub mod seasonal_reward {
    use super::*;

    /// シーズン報酬を請求するが、
    /// season_reward.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn claim_season_reward(ctx: Context<ClaimSeasonReward>) -> Result<()> {
        let season_reward = &mut ctx.accounts.season_reward;

        // 1. シーズンIDを設定（所有者チェックなし）
        season_reward.season_id = ctx.accounts.config.season_id;

        // 2. 請求済みフラグを立てる
        season_reward.claimed = true;

        // 3. 報酬プールからユーザーへ Lamports を直接転送
        let amount = ctx.accounts.config.reward_amount;
        **ctx.accounts.reward_vault.to_account_info().lamports.borrow_mut() -= amount;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimSeasonReward<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して所有者一致を検証すべき
    pub season_reward: Account<'info, SeasonReward>,

    /// 報酬を請求するユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 報酬を保管するプールアカウント
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,

    /// シーズン報酬設定アカウント
    pub config: Account<'info, SeasonConfig>,
}

#[account]
pub struct SeasonReward {
    /// 本来この請求アカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 対象シーズンの ID
    pub season_id: u64,
    /// すでに請求済みかどうか
    pub claimed: bool,
}

#[account]
pub struct SeasonConfig {
    /// 対象シーズンの固定 ID
    pub season_id: u64,
    /// 1シーズンあたりの報酬量（Lamports）
    pub reward_amount: u64,
}
