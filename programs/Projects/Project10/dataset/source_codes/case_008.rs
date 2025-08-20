use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0080E41ID");

#[program]
pub mod donation_case_008 {
    use super::*;

    pub fn donation_action_008(ctx: Context<DonationCtx008>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.donation_src_008.to_account_info(), to: ctx.accounts.donation_dst_008.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"donation", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DonationCtx008<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub donation_src_008: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub donation_dst_008: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"donation"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



