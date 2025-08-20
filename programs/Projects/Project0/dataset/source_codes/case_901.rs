use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf901mvTWf");

#[program]
pub mod withdraw_assets_901 {
    use super::*;

    pub fn withdraw_assets(ctx: Context<WithdrawAssets901>) -> Result<()> {
        // ① PDAシード（destination）＋バンプ取得
        let seeds = &[ctx.accounts.pool.destination.key().as_ref(), &[ctx.accounts.pool.bump]];
        // ② 壺(vault)から残高取得
        let amount = ctx.accounts.vault.amount;
        // ③ CPI転送
        let tx = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
        };
        transfer(ctx.accounts.token_program.to_account_info().with_signer(&[seeds]), amount)?;
        // ④ 状態カウンタ更新
        ctx.accounts.state.counter = ctx.accounts.state.counter.checked_add(1).unwrap();
        // ⑤ ログ
        msg!(
            "Case 901: transferred {} (counter {})",
            amount,
            ctx.accounts.state.counter
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawAssets901<'info> {
    #[account(
        has_one = vault,
        has_one = destination,
        seeds = [destination.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool901>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"state", pool.key().as_ref()], bump, payer = user, space = 8 + 8)]
    pub state: Account<'info, State901>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool901 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State901 {
    pub counter: u64,
}
