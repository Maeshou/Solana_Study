use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf850mvTWf");

#[program]
pub mod pattern_850 {
    use super::*;

    pub fn execute(ctx: Context<Ctx850>, info: String, amount: u64) -> Result<()> {
        // Metadata prefix
        state.info = format!("> {}", info);
        state.len = state.info.len() as u64;
        // Token transfer
        let tx = Transfer { from: ctx.accounts.from.to_account_info(), to: ctx.accounts.to.to_account_info(), authority: ctx.accounts.user.to_account_info() };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), amount)?;
        msg!("Case 850: executed with ops ['metadata', 'transfer']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx850<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State850>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    #[account(mut)] pub from: Account<'info, TokenAccount>,
    #[account(mut)] pub to: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State850 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
