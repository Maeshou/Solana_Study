use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVL");

#[program]
pub mod spl_token_logger {
    use super::*;

    /// ロガーアカウントを初期化：所有者と監視対象の Mint を設定
    pub fn initialize_logger(
        ctx: Context<InitializeLogger>,
        mint: Pubkey,
    ) -> Result<()> {
        let logger = &mut ctx.accounts.logger;
        logger.owner         = ctx.accounts.authority.key();
        logger.mint          = mint;
        logger.last_amount   = 0;
        Ok(())
    }

    /// 指定トークンアカウントのトークン残高を記録
    pub fn record_amount(ctx: Context<RecordAmount>) -> Result<()> {
        let logger       = &mut ctx.accounts.logger;
        let token_acc    = &ctx.accounts.token_account;
        // Anchor の Account<TokenAccount> 型が owner=token::ID, mint, authority を検証
        logger.last_amount = token_acc.amount;
        msg!(
            "Recorded {} tokens for mint {}",
            logger.last_amount,
            logger.mint
        );
        Ok(())
    }
}

#[account]
pub struct TokenLogger {
    pub owner:        Pubkey, // ロガー所有者
    pub mint:         Pubkey, // 監視対象の Mint
    pub last_amount:  u64,    // 直近に記録したトークン残高
}

#[derive(Accounts)]
pub struct InitializeLogger<'info> {
    /// ロガーを一度だけ初期化する通常アカウント
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8
    )]
    pub logger:    Account<'info, TokenLogger>,

    /// 初期化権限を持つ署名者
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordAmount<'info> {
    /// 既存のロガー、owner フィールドと一致する署名者のみ許可
    #[account(
        mut,
        has_one = owner,
    )]
    pub logger:        Account<'info, TokenLogger>,

    /// 監視対象トークンアカウント  
    /// - 所有者が SPL Token プログラム  
    /// - mint が logger.mint  
    /// - authority が logger.owner  
    #[account(
        mut,
        token::mint = logger.mint,
        token::authority = logger.owner
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// logger.owner に一致する署名者
    #[account(signer)]
    pub owner:         AccountInfo<'info>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
}
