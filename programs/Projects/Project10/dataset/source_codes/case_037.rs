use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH037163FID");

#[program]
pub mod reward_distribution_case_037 {
    use super::*;

    pub fn reward_distribution_action_037(ctx: Context<Reward_distributionCtx037>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.reward_distribution_dst_037.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"reward_distribution", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.reward_distribution_dst_037.to_account_info(), to: ctx.accounts.reward_distribution_src_037.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"reward_distribution", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Reward_distributionCtx037<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub reward_distribution_dst_037: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub reward_distribution_src_037: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"reward_distribution"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



