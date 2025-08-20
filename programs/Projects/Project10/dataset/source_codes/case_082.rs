use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH08204CAID");

#[program]
pub mod vesting_case_082 {
    use super::*;

    pub fn vesting_action_082(ctx: Context<VestingCtx082>, amount: u64) -> Result<()> {

        // Transfer and emit event
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.pool.to_account_info(), to: ctx.accounts.user.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"vesting", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        emit!(TransferEvent { user: ctx.accounts.user.key(), amount });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VestingCtx082<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub user: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"vesting"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



#[event]
pub struct TransferEvent {
    pub user: Pubkey,
    pub amount: u64,
}

