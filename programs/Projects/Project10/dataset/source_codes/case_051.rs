use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0510FCCID");

#[program]
pub mod airdrop_case_051 {
    use super::*;

    pub fn airdrop_action_051(ctx: Context<AirdropCtx051>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.airdrop_dst_051.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"airdrop", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.airdrop_dst_051.to_account_info(), to: ctx.accounts.airdrop_src_051.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"airdrop", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AirdropCtx051<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub airdrop_dst_051: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub airdrop_src_051: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"airdrop"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



