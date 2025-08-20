use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVL");

#[program]
pub mod token_balance_tracker {
    use super::*;

    /// ロガーアカウントを初期化：所有者と監視対象のミントを設定
    pub fn initialize(
        ctx: Context<Initialize>,
        mint: Pubkey,
    ) -> Result<()> {
        let tracker       = &mut ctx.accounts.tracker;
        tracker.owner     = ctx.accounts.authority.key();
        tracker.mint      = mint;
        tracker.history.clear();
        Ok(())
    }

    /// トークンアカウントの残高を履歴に追加  
    /// — token_account は `token::mint`／`token::authority` 属性で検証済み
    pub fn record(
        ctx: Context<Record>,
    ) -> Result<()> {
        let tracker       = &mut ctx.accounts.tracker;
        let token_acc     = &ctx.accounts.token_account;
        let now           = ctx.accounts.clock.unix_timestamp;
        let amount        = token_acc.amount;
        tracker.history.push((now, amount));
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct TokenBalanceTracker {
    pub owner:    Pubkey,            // ロガー所有者
    pub mint:     Pubkey,            // 監視対象のトークンミント
    pub history:  Vec<(i64, u64)>,   // (timestamp, balance) の履歴
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// ロガーアカウント（PDAまたは通常アカウントどちらでも可）
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"tb_tracker", authority.key().as_ref()],
        bump,
        // discriminator(8)+owner(32)+mint(32)+Vec len(4)+max10*(8+8)
        space = 8 + 32 + 32 + 4 + 10 * (8 + 8)
    )]
    pub tracker:       Account<'info, TokenBalanceTracker>,

    /// 初期化を行う権限
    #[account(mut)]
    pub authority:     Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Record<'info> {
    /// 既存のロガー、ownerフィールドとの一致をチェック
    #[account(
        mut,
        has_one = owner
    )]
    pub tracker:       Account<'info, TokenBalanceTracker>,

    /// 監視対象のトークンアカウント  
    /// — 所有者が SPL Token プログラム  
    /// — mint が tracker.mint  
    /// — authority が tracker.owner  
    #[account(
        mut,
        token::mint = tracker.mint,
        token::authority = tracker.owner
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// tracker.owner 署名チェック
    #[account(signer)]
    pub owner:         AccountInfo<'info>,

    /// 現在時刻取得用
    pub clock:         Sysvar<'info, Clock>,
}
