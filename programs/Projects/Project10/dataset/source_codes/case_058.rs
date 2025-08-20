use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0581730ID");

#[program]
pub mod vesting_case_058 {
    use super::*;

    pub fn vesting_action_058(ctx: Context<VestingCtx058>, amount: u64) -> Result<()> {

        // Mint tokens for user
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.vesting_dst_058.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"vesting", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Then transfer half
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.vesting_dst_058.to_account_info(), to: ctx.accounts.vesting_src_058.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"vesting", &[ctx.bumps["auth"]]]]
            ),
            amount/2,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VestingCtx058<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = auth)]
    pub vesting_dst_058: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub vesting_src_058: Account<'info, TokenAccount>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"vesting"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



