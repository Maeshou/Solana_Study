use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBetSvc003");

#[program]
pub mod betting_service {
    use super::*;

    /// ユーザーがベットを行い、ベットプールにLamportsを預け入れるが、
    /// bet_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        let bet = &mut ctx.accounts.bet_account;
        // 1. 最後にベットした金額を記録
        bet.last_bet_amount = amount;
        // 2. ベット回数をインクリメント
        bet.bet_count = bet.bet_count.checked_add(1).unwrap();

        // 3. ユーザーからプールへ直接Lamportsを移動
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= amount;
        **ctx.accounts.bet_pool.to_account_info().lamports.borrow_mut() += amount;

        Ok(())
    }

    /// ユーザーがベットの勝利報酬を請求するが、
    /// bet_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        let bet = &ctx.accounts.bet_account;
        let reward_acc = &mut ctx.accounts.reward_account;

        // 1. 報酬は最後のベットの2倍とする
        let reward = bet.last_bet_amount.checked_mul(2).unwrap();
        // 2. 報酬口座に加算
        reward_acc.balance = reward_acc.balance.checked_add(reward).unwrap();

        // 3. プールからユーザーへ直接Lamportsを移動
        **ctx.accounts.bet_pool.to_account_info().lamports.borrow_mut() -= reward;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += reward;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して照合すべき
    pub bet_account: Account<'info, BetAccount>,

    /// ベット資金を保管するプールアカウント
    #[account(mut)]
    pub bet_pool: AccountInfo<'info>,

    /// ベットを行うユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して照合すべき
    pub bet_account: Account<'info, BetAccount>,

    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して照合すべき
    pub reward_account: Account<'info, RewardAccount>,

    /// ベット資金を保管するプールアカウント
    #[account(mut)]
    pub bet_pool: AccountInfo<'info>,

    /// 報酬を受け取るユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct BetAccount {
    /// 本来このベット設定を所有するべきユーザーのPubkey
    pub owner: Pubkey,
    /// 最後にベットした金額
    pub last_bet_amount: u64,
    /// ベット回数の累計
    pub bet_count: u64,
}

#[account]
pub struct RewardAccount {
    /// 本来この報酬口座を所有するべきユーザーのPubkey
    pub owner: Pubkey,
    /// 累計報酬残高
    pub balance: u64,
}
