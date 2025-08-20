use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, close_account, MintTo, CloseAccount, TokenAccount, Mint, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf605mvTWf");

#[program]
pub mod launch_procedure_605 {
    use super::*;

    pub fn launch_procedure(ctx: Context<LaunchProcedure605>, amount: u64) -> Result<()> {
        // ① ミント CPI
        let mt = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        mint_to(CpiContext::new(ctx.accounts.token_program.to_account_info(), mt), amount)?;
        // ② CloseAccount CPI
        let ca = CloseAccount {
            account: ctx.accounts.temp_account.to_account_info(),  
            destination: ctx.accounts.return_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        close_account(CpiContext::new(ctx.accounts.token_program.to_account_info(), ca))?;
        // ③ Clock 取得＋カウンタ更新
        let timestamp = Clock::get()?.unix_timestamp as u64;
        let rec = &mut ctx.accounts.rec;
        rec.counter = rec.counter.checked_add(1).unwrap();
        rec.last_timestamp = timestamp;
        // ④ ログ
        msg!(
            "Case 605: minted {}, closed temp, counter {}, at {}",
            amount,
            rec.counter,
            timestamp
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LaunchProcedure605<'info> {
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub temp_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub return_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub rec: Account<'info, Rec605>,
    #[account(signer)]
    pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Rec605 {
    pub counter: u64,
    pub last_timestamp: u64,
}
