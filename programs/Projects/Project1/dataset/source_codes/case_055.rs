use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfLott01");

#[program]
pub mod nft_weighted_lottery_entry {
    use super::*;

    pub fn enter_lottery(ctx: Context<LotteryCtx>) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        let user = ctx.accounts.user.key();

        // NFT保有口数を抽選重みとする（口数 = amount）
        let token_account = &ctx.accounts.user_token_account;
        let amount = token_account.amount;

        // 再登録防止
        require!(!entry.registered, LotteryError::AlreadyEntered);

        // 記録
        entry.user = user;
        entry.weight = amount;
        entry.registered = true;

        Ok(())
    }

    pub fn show(ctx: Context<LotteryCtx>) -> Result<()> {
        let e = &ctx.accounts.entry;
        msg!("User: {}", e.user);
        msg!("Weight: {}", e.weight);
        msg!("Registered: {}", e.registered);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LotteryCtx<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 1,
        seeds = [b"lottery", user.key().as_ref()],
        bump
    )]
    pub entry: Account<'info, LotteryEntry>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == target_mint.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub target_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LotteryEntry {
    pub user: Pubkey,
    pub weight: u64,       // NFTの保有数＝抽選口数
    pub registered: bool,  // 1人1回まで
}

#[error_code]
pub enum LotteryError {
    #[msg("You have already entered the lottery.")]
    AlreadyEntered,
}
