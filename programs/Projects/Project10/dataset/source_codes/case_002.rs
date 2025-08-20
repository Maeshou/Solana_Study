use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH002048EID");

#[program]
pub mod staking_case_002 {
    use super::*;

    pub fn staking_action_002(ctx: Context<StakingCtx002>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.staking_dst_002.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"staking", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.staking_dst_002.to_account_info(), to: ctx.accounts.staking_src_002.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"staking", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakingCtx002<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub staking_dst_002: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub staking_src_002: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"staking"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



