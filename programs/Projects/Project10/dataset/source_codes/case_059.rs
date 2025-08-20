use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH0590481ID");

#[program]
pub mod escrow_deposit_case_059 {
    use super::*;

    pub fn escrow_deposit_action_059(ctx: Context<Escrow_depositCtx059>, amount: u64) -> Result<()> {

        // Burn some tokens
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn { mint: ctx.accounts.mint.to_account_info(), to: ctx.accounts.escrow_deposit_src_059.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"escrow_deposit", &[ctx.bumps["auth"]]]]
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
pub struct Escrow_depositCtx059<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub escrow_deposit_src_059: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub state: Account<'info, DataState>,

    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"escrow_deposit"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}


#[account]
pub struct DataState {
    pub count: u64,
}


