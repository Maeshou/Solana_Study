use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0091BC8ID");

#[program]
pub mod lottery_case_009 {
    use super::*;

    pub fn lottery_action_009(ctx: Context<LotteryCtx009>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.lottery_dst_009.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"lottery", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.lottery_dst_009.to_account_info(), to: ctx.accounts.lottery_src_009.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"lottery", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LotteryCtx009<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub lottery_dst_009: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub lottery_src_009: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"lottery"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



