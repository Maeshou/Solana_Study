use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf904mvTWf");

#[program]
pub mod extract_credits_904 {
    use super::*;

    pub fn extract_credits(ctx: Context<ExtractCredits904>) -> Result<()> {
        let seeds = &[ctx.accounts.pool.destination.key().as_ref(), &[ctx.accounts.pool.bump]];
        let amount = ctx.accounts.vault.amount;
        transfer(ctx.accounts.token_program.to_account_info().with_signer(&[seeds]), amount)?;
        // レント免除バランス取得
        let rent_bal = ctx.accounts.rent.minimum_balance(0);
        ctx.accounts.state.rent_exempt = rent_bal;
        msg!(
            "Case 904: transferred {} (rent_exempt={})",
            amount,
            rent_bal
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtractCredits904<'info> {
    #[account(has_one=vault, has_one=destination, seeds=[destination.key().as_ref()], bump=pool.bump)]
    pub pool: Account<'info, Pool904>,
    #[account(mut)] pub vault: Account<'info, TokenAccount>,
    #[account(mut)] pub destination: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"state", pool.key().as_ref()], bump, payer=user, space=8+8)]
    pub state: Account<'info, State904>,
    #[account(signer)] pub user: Signer<'info>,
    #[account(address=token::ID)] pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Pool904 {
    pub vault: Pubkey,
    pub destination: Pubkey,
    pub bump: u8,
}

#[account]
pub struct State904 {
    pub rent_exempt: u64,
}
