use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0931961ID");

#[program]
pub mod membership_redemption_case_093 {
    use super::*;

    pub fn membership_redemption_action_093(ctx: Context<Membership_redemptionCtx093>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.membership_redemption_dst_093.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"membership_redemption", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.membership_redemption_dst_093.to_account_info(), to: ctx.accounts.membership_redemption_src_093.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"membership_redemption", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Membership_redemptionCtx093<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub membership_redemption_dst_093: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub membership_redemption_src_093: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"membership_redemption"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



