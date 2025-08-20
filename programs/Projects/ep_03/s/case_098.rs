use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgCrowdFund01");

#[program]
pub mod crowdfunding_service {
    use super::*;

    /// 支援者がキャンペーンに資金を提供するが、
    /// campaign_account.owner と ctx.accounts.user.key() の照合検証がない
    pub fn contribute(ctx: Context<ManageCampaign>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign_account;
        let user = &ctx.accounts.user;
        // 1. キャンペーン総支援額と支援回数を記録
        record_contribution(campaign, amount);
        // 2. ユーザーからキャンペーンのプールへ Lamports を直接移動
        transfer_lamports(&user.to_account_info(), &ctx.accounts.pool, amount)?;
        Ok(())
    }

    /// キャンペーンオーナーが資金を引き出すが、
    /// campaign_account.owner と ctx.accounts.user.key() の照合検証がない
    pub fn withdraw(ctx: Context<ManageCampaign>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign_account;
        let user = &ctx.accounts.user;
        // 1. 総引き出し額と回数を記録
        record_withdrawal(campaign, amount);
        // 2. プールからユーザーへ Lamports を直接移動
        transfer_lamports(&ctx.accounts.pool, &user.to_account_info(), amount)?;
        Ok(())
    }
}

/// キャンペーンへの支援を記録するヘルパー関数
fn record_contribution(c: &mut CampaignAccount, amount: u64) {
    c.total_raised = c.total_raised.saturating_add(amount);
    c.contribution_count = c.contribution_count.saturating_add(1);
}

/// キャンペーンからの引き出しを記録するヘルパー関数
fn record_withdrawal(c: &mut CampaignAccount, amount: u64) {
    c.total_withdrawn = c.total_withdrawn.saturating_add(amount);
    c.withdraw_count = c.withdraw_count.saturating_add(1);
}

/// 直接 Lamports を移動するヘルパー関数
fn transfer_lamports(
    from: &AccountInfo, 
    to: &AccountInfo, 
    amount: u64
) -> Result<()> {
    **from.lamports.borrow_mut() = from
        .lamports()
        .checked_sub(amount)
        .unwrap();
    **to.lamports.borrow_mut() = to
        .lamports()
        .checked_add(amount)
        .unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct ManageCampaign<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub campaign_account: Account<'info, CampaignAccount>,

    /// ラムポートを保管するキャンペーンプール
    #[account(mut)]
    pub pool: AccountInfo<'info>,

    /// 操作をリクエストするユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct CampaignAccount {
    /// 本来このキャンペーンを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// これまでに集めた総支援額
    pub total_raised: u64,
    /// 支援トランザクション数
    pub contribution_count: u64,
    /// これまでに引き出した総額
    pub total_withdrawn: u64,
    /// 引き出し回数
    pub withdraw_count: u64,
}
