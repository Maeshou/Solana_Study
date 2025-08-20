use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0521466ID");

#[program]
pub mod burn_case_052 {
    use super::*;

    pub fn burn_action_052(ctx: Context<BurnCtx052>, amount: u64) -> Result<()> {

        // Burn some tokens
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.burn_src_052.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"burn", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        // Update state count
        let mut st = &mut ctx.accounts.state;
        st.count = st.count.checked_add(amount).unwrap_or(st.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnCtx052<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub burn_src_052: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub state: Account<'info, DataState>,

    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"burn"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}


#[account]
pub struct DataState {
    pub count: u64,
}


