use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0790BAEID");

#[program]
pub mod royalty_case_079 {
    use super::*;

    pub fn royalty_action_079(ctx: Context<RoyaltyCtx079>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.royalty_dst_079.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"royalty", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.royalty_dst_079.to_account_info(), to: ctx.accounts.royalty_src_079.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"royalty", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RoyaltyCtx079<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub royalty_dst_079: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub royalty_src_079: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"royalty"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



