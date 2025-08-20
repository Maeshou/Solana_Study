use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVK");

#[program]
pub mod token_balance_logger {
    use super::*;

    /// ロガーアカウントを初期化（bump と最初の残高 0）
    pub fn initialize_logger(ctx: Context<InitializeLogger>) -> Result<()> {
        let logger = &mut ctx.accounts.logger;
        logger.bump         = *ctx.bumps.get("logger").unwrap();
        logger.last_balance = 0;
        Ok(())
    }

    /// 指定トークンアカウントの lamports 残高を記録
    pub fn record_token_balance(ctx: Context<RecordTokenBalance>) -> Result<()> {
        let logger = &mut ctx.accounts.logger;
        // owner = spl_token::id() 制約で、このアカウントが SPL Token プログラム所有であることを保証
        let token_acc = &ctx.accounts.token_account;
        logger.last_balance = **token_acc.lamports.borrow();
        Ok(())
    }
}

#[account]
pub struct TokenBalanceLogger {
    pub bump:         u8,    // PDA bump
    pub last_balance: u64,   // 直近に記録した lamports 残高
}

#[derive(Accounts)]
pub struct InitializeLogger<'info> {
    /// ロガーアカウント (PDA)
    #[account(
        init,
        payer = authority,
        seeds = [b"token_balance_logger", authority.key().as_ref()],
        bump,
        space = 8 + 1 + 8
    )]
    pub logger:    Account<'info, TokenBalanceLogger>,

    /// 初期化を実行する署名者
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordTokenBalance<'info> {
    /// 既存のロガーアカウント
    #[account(
        mut,
        seeds = [b"token_balance_logger", authority.key().as_ref()],
        bump = logger.bump
    )]
    pub logger:        Account<'info, TokenBalanceLogger>,

    /// 記録対象のアカウント（必ず SPL Token プログラム所有であることを検証）
    #[account(owner = token::ID)]
    pub token_account: AccountInfo<'info>,

    /// 呼び出し権限
    #[account(signer)]
    pub authority:     AccountInfo<'info>,

    pub clock:         Sysvar<'info, Clock>,
}
