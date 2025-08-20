use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, MintTo, Burn};


declare_id!("PDSH07504A7ID");

#[program]
pub mod airdrop_case_075 {
    use super::*;

    pub fn airdrop_action_075(ctx: Context<AirdropCtx075>, amount: u64) -> Result<()> {

        // Transfer and emit event
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer { from: ctx.accounts.pool.to_account_info(), to: ctx.accounts.user.to_account_info(), authority: ctx.accounts.auth.to_account_info() },
                &[&[b"airdrop", &[ctx.bumps["auth"]]]]
            ),
            amount,
        )?;
        emit!(TransferEvent { user: ctx.accounts.user.key(), amount });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AirdropCtx075<'info> {

    #[account(mut, token::mint = mint, token::authority = auth)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub user: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    /// CHECK: PDA derived from static seed, vulnerable to sharing
    #[account(seeds = [b"airdrop"], bump)]
    pub auth: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}



#[event]
pub struct TransferEvent {
    pub user: Pubkey,
    pub amount: u64,
}

