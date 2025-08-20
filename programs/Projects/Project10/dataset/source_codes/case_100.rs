use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH1002648ID");

#[program]
pub mod burn_case_100 {
    use super::*;

    pub fn burn_action_100(ctx: Context<BurnCtx100>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.burn_dst_100.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"burn", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.burn_dst_100.to_account_info(), to: ctx.accounts.burn_src_100.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"burn", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnCtx100<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub burn_dst_100: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub burn_src_100: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"burn"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



