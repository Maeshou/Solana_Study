use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH04312CBID");

#[program]
pub mod tipping_case_043 {
    use super::*;

    pub fn tipping_action_043(ctx: Context<TippingCtx043>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.tipping_src_043.to_account_info(), to: ctx.accounts.tipping_dst_043.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"tipping", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TippingCtx043<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub tipping_src_043: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub tipping_dst_043: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"tipping"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



