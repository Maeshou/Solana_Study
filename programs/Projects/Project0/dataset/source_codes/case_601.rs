use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, mint_to, Transfer, MintTo, TokenAccount, Mint, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf601mvTWf");

#[program]
pub mod invoke_routine_601 {
    use super::*;

    pub fn invoke_routine(ctx: Context<InvokeRoutine601>, send_amt: u64, mint_amt: u64) -> Result<()> {
        // ① 前回トランスファー量を state から取得
        let state = &mut ctx.accounts.state;
        let prev_send = state.last_sent;
        // ② トークン転送 CPI
        let tx = Transfer {
            from: ctx.accounts.source.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), send_amt)?;
        // ③ トークンミント CPI
        let mc = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.bonus_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        mint_to(CpiContext::new(ctx.accounts.token_program.to_account_info(), mc), mint_amt)?;
        // ④ タイムスタンプ取得
        let clock = Clock::get()?;
        // ⑤ 状態更新
        state.last_sent = send_amt;
        state.minted_total = state.minted_total.checked_add(mint_amt).unwrap();
        state.last_ts = clock.unix_timestamp as u64;
        // ⑥ ログ出力
        msg!(
            "Case 601: sent {} (prev {}), minted {}, total_minted {}, at {}",
            send_amt,
            prev_send,
            mint_amt,
            state.minted_total,
            state.last_ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InvokeRoutine601<'info> {
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub bonus_account: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"state"], bump)]
    pub state: Account<'info, State601>,
    #[account(signer)]
    pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct State601 {
    pub last_sent: u64,
    pub minted_total: u64,
    pub last_ts: u64,
}
