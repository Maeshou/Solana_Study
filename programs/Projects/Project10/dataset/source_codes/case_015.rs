use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0151A9EID");

#[program]
pub mod payroll_case_015 {
    use super::*;

    pub fn payroll_action_015(ctx: Context<PayrollCtx015>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.payroll_src_015.to_account_info(), to: ctx.accounts.payroll_dst_015.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"payroll", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayrollCtx015<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub payroll_src_015: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub payroll_dst_015: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"payroll"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



