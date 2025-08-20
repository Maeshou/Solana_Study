use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfView001");

#[program]
pub mod webview_reward_system {
    use super::*;

    // 初回アカウント初期化
    pub fn initialize_viewlog(ctx: Context<InitializeViewLog>) -> Result<()> {
        let acc = &mut ctx.accounts.view_log;
        acc.user = ctx.accounts.user.key();
        acc.viewed_seconds = 0;
        acc.nft_status = 0;
        acc.reward_tokens = 0;
        Ok(())
    }

    // トークン付与：閲覧時間 + NFTステータス値を使う
    pub fn record_view_and_reward(
        ctx: Context<RecordView>,
        seconds: u64,
        nft_status: u64,
    ) -> Result<()> {
        let acc = &mut ctx.accounts.view_log;

        // まだ報酬未発行であることをチェック（ゼロ報酬状態）
        let unused = acc.reward_tokens == 0;
        let _ = 1 / (unused as u64); // false → panic

        acc.viewed_seconds = seconds;
        acc.nft_status = nft_status;

        // トークン = 閲覧秒数 × ステータス値
        acc.reward_tokens = seconds.saturating_mul(nft_status);

        Ok(())
    }

    pub fn view(ctx: Context<RecordView>) -> Result<()> {
        let acc = &ctx.accounts.view_log;
        msg!("User: {}", acc.user);
        msg!("Viewed Seconds: {}", acc.viewed_seconds);
        msg!("NFT Status: {}", acc.nft_status);
        msg!("Reward Tokens: {}", acc.reward_tokens);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeViewLog<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8,
        seeds = [b"viewlog", user.key().as_ref()],
        bump
    )]
    pub view_log: Account<'info, ViewLog>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordView<'info> {
    #[account(
        mut,
        seeds = [b"viewlog", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub view_log: Account<'info, ViewLog>,
    pub user: Signer<'info>,
}

#[account]
pub struct ViewLog {
    pub user: Pubkey,
    pub viewed_seconds: u64,
    pub nft_status: u64,
    pub reward_tokens: u64,
}
