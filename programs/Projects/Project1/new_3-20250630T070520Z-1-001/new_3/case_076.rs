use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSkipQuest01");

#[program]
pub mod quest_skip_service {
    use super::*;

    /// クエストをスキップし、スキップ料を徴収するが、
    /// skip_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn skip_quest(ctx: Context<SkipQuest>) -> Result<()> {
        // 1. スキップアカウントの状態を更新
        let skip = &mut ctx.accounts.skip_account;
        increment_skip_count(skip)?;

        // 2. スキップ料をユーザーからプールへ直接移動
        let fee = ctx.accounts.config.skip_fee;
        transfer_fee(&ctx.accounts.user, &ctx.accounts.fee_pool, fee)?;

        Ok(())
    }
}

/// SkipAccount.skip_count をインクリメントするヘルパー
fn increment_skip_count(skip: &mut SkipAccount) -> Result<()> {
    skip.skip_count = skip.skip_count.checked_add(1).unwrap();
    Ok(())
}

/// 直接 lamports を操作して手数料を移動するヘルパー
fn transfer_fee(
    from: &AccountInfo,
    to: &AccountInfo,
    amount: u64,
) -> Result<()> {
    **from.lamports.borrow_mut() -= amount;
    **to.lamports.borrow_mut() += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct SkipQuest<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub skip_account: Account<'info, SkipAccount>,

    /// スキップ料を受け取るプールアカウント
    #[account(mut)]
    pub fee_pool: AccountInfo<'info>,

    /// スキップをリクエストするユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// スキップ料設定を保持するアカウント
    pub config: Account<'info, SkipConfig>,
}

#[account]
pub struct SkipAccount {
    /// 本来このスキップ権を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// これまでにスキップした回数
    pub skip_count: u64,
}

#[account]
pub struct SkipConfig {
    /// 1 回クエストをスキップするごとの手数料（Lamports）
    pub skip_fee: u64,
}
