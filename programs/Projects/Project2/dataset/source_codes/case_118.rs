use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVL");

#[program]
pub mod balance_logger {
    use super::*;

    /// ログアカウントを初期化：bump と初回ログとして 0 を設定
    pub fn initialize_logger(ctx: Context<InitializeLogger>) -> Result<()> {
        let logger = &mut ctx.accounts.logger;
        logger.bump         = *ctx.bumps.get("logger").unwrap();
        logger.last_balance = 0;
        Ok(())
    }

    /// 指定アカウントの残高を読み取り、ログを更新
    pub fn record_balance(ctx: Context<RecordBalance>) -> Result<()> {
        let logger = &mut ctx.accounts.logger;
        // owner = System::id() 制約で、target が SystemProgram 所有であることを保証済み
        let target_info = &ctx.accounts.target;
        logger.last_balance = **target_info.lamports.borrow();
        Ok(())
    }
}

#[account]
pub struct BalanceLogger {
    pub bump:         u8,    // PDA bump
    pub last_balance: u64,   // 直近に記録した lamports 残高
}

#[derive(Accounts)]
pub struct InitializeLogger<'info> {
    /// Logger アカウント (PDA)
    #[account(
        init,
        payer = authority,
        seeds = [b"balance_logger", authority.key().as_ref()],
        bump,
        space = 8 + 1 + 8
    )]
    pub logger:    Account<'info, BalanceLogger>,

    /// 初期化を行う権限
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordBalance<'info> {
    /// 既存の Logger (PDA)
    #[account(
        mut,
        seeds = [b"balance_logger", authority.key().as_ref()],
        bump = logger.bump
    )]
    pub logger:    Account<'info, BalanceLogger>,

    /// 残高を記録する対象アカウント（SystemProgram 所有であることを検証）
    #[account(
        owner = System::id()
    )]
    pub target:    AccountInfo<'info>,

    /// 呼び出し権限
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
