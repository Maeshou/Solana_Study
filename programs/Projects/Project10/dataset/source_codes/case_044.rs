use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH044256AID");

#[program]
pub mod micropayment_case_044 {
    use super::*;

    pub fn micropayment_action_044(ctx: Context<MicropaymentCtx044>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.micropayment_dst_044.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"micropayment", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.micropayment_dst_044.to_account_info(), to: ctx.accounts.micropayment_src_044.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"micropayment", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MicropaymentCtx044<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub micropayment_dst_044: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub micropayment_src_044: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"micropayment"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



