use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{approve, burn, Approve, Burn, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf602mvTWf");

#[program]
pub mod dispatch_operation_602 {
    use super::*;

    pub fn dispatch_operation(ctx: Context<DispatchOperation602>, lamports: u64, approve_amt: u64, burn_amt: u64) -> Result<()> {
        // ① システムプログラム送金 CPI
        let sys_tx = system_program::Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
        };
        system_program::transfer(CpiContext::new(ctx.accounts.sys_prog.to_account_info(), sys_tx), lamports)?;
        // ② トークン Approve CPI
        let ap = Approve {
            to: ctx.accounts.account.to_account_info(),
            delegate: ctx.accounts.delegate.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        approve(CpiContext::new(ctx.accounts.token_program.to_account_info(), ap), approve_amt)?;
        // ③ トークン Burn CPI
        let br = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        burn(CpiContext::new(ctx.accounts.token_program.to_account_info(), br), burn_amt)?;
        // ④ メタに合計額を記録
        let meta = &mut ctx.accounts.meta;
        meta.total_lamports = meta.total_lamports.checked_add(lamports).unwrap();
        meta.total_approved = meta.total_approved.checked_add(approve_amt).unwrap();
        meta.total_burned = meta.total_burned.checked_add(burn_amt).unwrap();
        // ⑤ ログ
        msg!(
            "Case 602: {} lamports, {} approved, {} burned | totals: {} / {} / {}",
            lamports,
            approve_amt,
            burn_amt,
            meta.total_lamports,
            meta.total_approved,
            meta.total_burned
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DispatchOperation602<'info> {
    #[account(address = system_program::ID)]
    pub sys_prog: Program<'info, System>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub mint: Account<'info, anchor_spl::token::Mint>,
    #[account(mut)]
    pub account: Account<'info, TokenAccount>,
    pub delegate: UncheckedAccount<'info>,
    #[account(mut)]
    pub meta: Account<'info, Meta602>,
}

#[account]
pub struct Meta602 {
    pub total_lamports: u64,
    pub total_approved: u64,
    pub total_burned: u64,
}
