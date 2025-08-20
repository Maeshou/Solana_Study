use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH07824B5ID");

#[program]
pub mod referral_case_078 {
    use super::*;

    pub fn referral_action_078(ctx: Context<ReferralCtx078>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.referral_src_078.to_account_info(), to: ctx.accounts.referral_dst_078.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"referral", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReferralCtx078<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub referral_src_078: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub referral_dst_078: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"referral"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



