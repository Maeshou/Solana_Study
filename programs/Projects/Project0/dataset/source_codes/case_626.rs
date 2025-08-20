use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf626mvTWf");

#[program]
pub mod invoke_routine_626 {
    use super::*;

    pub fn invoke_routine(ctx: Context<InvokeRoutine626>, send_amt: u64, mint_amt: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let prev_send = state.last_sent;
        let tx = Transfer {
            from: ctx.accounts.source.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), send_amt)?;
        let mc = anchor_spl::token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.bonus.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        anchor_spl::token::mint_to(CpiContext::new(ctx.accounts.token_program.to_account_info(), mc), mint_amt)?;
        let clock = Clock::get()?;
        state.last_sent = send_amt;
        state.minted_total = state.minted_total.checked_add(mint_amt).unwrap();
        state.last_ts = clock.unix_timestamp as u64;
        msg!(
            "Case 626: sent {} (prev {}), minted {} total {} at {}",
            send_amt, prev_send, mint_amt, state.minted_total, state.last_ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InvokeRoutine626<'info> {
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    #[account(mut)] pub source: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(mut)] pub mint: Account<'info, Mint>,
    #[account(mut)] pub bonus: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"state"], bump)] pub state: Account<'info, State626>,
    #[account(signer)] pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct State626 {
    pub last_sent: u64,
    pub minted_total: u64,
    pub last_ts: u64,
}
