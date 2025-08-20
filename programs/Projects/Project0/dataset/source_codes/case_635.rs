use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, close_account, MintTo, CloseAccount, TokenAccount, Mint, Token};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf635mvTWf");

#[program]
pub mod launch_procedure_635 {
    use super::*;

    pub fn launch_procedure(ctx: Context<LaunchProcedure635>, amount: u64) -> Result<()> {
        let mt = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.dest.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        mint_to(CpiContext::new(ctx.accounts.token_program.to_account_info(), mt), amount)?;
        let ca = CloseAccount {
            account: ctx.accounts.temp.to_account_info(),
            destination: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        close_account(CpiContext::new(ctx.accounts.token_program.to_account_info(), ca))?;
        let ts = Clock::get()?.unix_timestamp as u64;
        let rec = &mut ctx.accounts.rec;
        rec.counter = rec.counter.checked_add(1).unwrap();
        rec.last_ts = ts;
        msg!("Case 635: minted {} closed count {} at {}", amount, rec.counter, ts);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LaunchProcedure635<'info> {
    #[account(address=token::ID)] pub token_program: Program<'info, Token>,
    pub mint: Account<'info, Mint>,
    #[account(mut)] pub dest: Account<'info, TokenAccount>,
    #[account(mut)] pub temp: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: UncheckedAccount<'info>,
    #[account(mut)] pub rec: Account<'info, Rec635>,
    #[account(signer)] pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Rec635 {
    pub counter: u64,
    pub last_ts: u64,
}
