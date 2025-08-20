use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpZxyL45GHJ2kLMNopQRStUvWxY01");

#[program]
pub mod nft_supply_controller {
    use super::*;

    /// 1. 設定関数：ProgramResult を使う
    ///    NFT ドロップあたりのトークン数を設定する
    pub fn configure(
        ctx: Context<Configure>,
        tokens_per_drop: u64,
    ) -> ProgramResult {
        let cfg = &mut ctx.accounts.config;
        cfg.tokens_per_drop = tokens_per_drop;
        Ok(())
    }

    /// 2. 算出関数：トークン数を返す -> Result<u64>
    ///    ドロップ数に応じて必要なトークン量を計算する
    pub fn calculate_tokens(ctx: Context<Calculate>, drop_count: u64) -> Result<u64> {
        let cfg = &ctx.accounts.config;
        let total = cfg
            .tokens_per_drop
            .checked_mul(drop_count)
            .ok_or(ErrorCode::Overflow)?;
        Ok(total)
    }

    /// 3. 実行関数：戻り値を明示せず、unwrap() を多用
    ///    ドロップ数に応じたトークンをミントし、受信者に送る
    pub fn execute_withdrawal(ctx: Context<ExecuteWithdrawal>, drop_count: u64) {
        let cfg = &ctx.accounts.config;
        let amount = cfg.tokens_per_drop
            .checked_mul(drop_count)
            .unwrap(); // Overflow はパニックさせる

        // CPI でミント
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.reward_mint.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            amount,
        ).unwrap(); // Error もパニック
    }

    /// 4. 停止関数：Result<bool> を返す
    ///    ミントを一時停止するフラグを立てる
    pub fn pause(ctx: Context<Pause>) -> Result<bool> {
        let cfg = &mut ctx.accounts.config;
        cfg.paused = true;
        Ok(true)  // 成功時に true を返す
    }
}

#[account]
pub struct Config {
    pub tokens_per_drop: u64,
    pub paused: bool,
}

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(init, payer = initializer, space = 8 + 8 + 1)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Calculate<'info> {
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
pub struct ExecuteWithdrawal<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    /// CHECK: 署名者チェックなしでミント権限を渡す（脆弱）
    pub mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow calculating tokens")]
    Overflow,
}
