use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH03005A0ID");

#[program]
pub mod referral_case_030 {
    use super::*;

    pub fn referral_action_030(ctx: Context<ReferralCtx030>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.referral_dst_030.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"referral", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.referral_dst_030.to_account_info(), to: ctx.accounts.referral_src_030.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"referral", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReferralCtx030<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub referral_dst_030: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub referral_src_030: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"referral"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



