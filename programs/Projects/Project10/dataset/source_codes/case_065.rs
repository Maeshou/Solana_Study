use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0650DF3ID");

#[program]
pub mod commission_case_065 {
    use super::*;

    pub fn commission_action_065(ctx: Context<CommissionCtx065>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.commission_dst_065.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"commission", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.commission_dst_065.to_account_info(), to: ctx.accounts.commission_src_065.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"commission", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CommissionCtx065<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub commission_dst_065: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub commission_src_065: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"commission"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



