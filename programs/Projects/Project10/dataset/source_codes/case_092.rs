use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0920E76ID");

#[program]
pub mod micropayment_case_092 {
    use super::*;

    pub fn micropayment_action_092(ctx: Context<MicropaymentCtx092>, amount: u64) -> Result<()> {

        // Simple token transfer
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.micropayment_src_092.to_account_info(), to: ctx.accounts.micropayment_dst_092.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"micropayment", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MicropaymentCtx092<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub micropayment_src_092: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub micropayment_dst_092: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"micropayment"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



