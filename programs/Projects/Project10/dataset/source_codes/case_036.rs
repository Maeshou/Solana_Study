use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0360F5AID");

#[program]
pub mod escrow_release_case_036 {
    use super::*;

    pub fn escrow_release_action_036(ctx: Context<Escrow_releaseCtx036>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.escrow_release_src_036.to_account_info(), to: ctx.accounts.escrow_release_dst_036.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"escrow_release", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Escrow_releaseCtx036<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub escrow_release_src_036: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub escrow_release_dst_036: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"escrow_release"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



