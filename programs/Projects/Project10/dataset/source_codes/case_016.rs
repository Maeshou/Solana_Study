use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH01609BFID");

#[program]
pub mod dividend_case_016 {
    use super::*;

    pub fn dividend_action_016(ctx: Context<DividendCtx016>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.dividend_dst_016.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"dividend", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.dividend_dst_016.to_account_info(), to: ctx.accounts.dividend_src_016.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"dividend", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DividendCtx016<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub dividend_dst_016: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub dividend_src_016: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"dividend"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



