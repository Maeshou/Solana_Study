use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf849mvTWf");

#[program]
pub mod pattern_849 {
    use super::*;

    pub fn execute(ctx: Context<Ctx849>, initial: u64, amount: u64) -> Result<()> {
        // Double init
        state.value = initial.checked_mul(2).unwrap();
        // Token transfer
        let tx = Transfer { from: ctx.accounts.from.to_account_info(), to: ctx.accounts.to.to_account_info(), authority: ctx.accounts.user.to_account_info() };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), amount)?;
        msg!("Case 849: executed with ops ['double_init', 'transfer']");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx849<'info> {
    #[account(init, seeds = [b"state", user.key().as_ref()], bump, payer = user, space = 8 + 1 + 32 + 256)]
    pub state: Account<'info, State849>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(address = token::ID)] pub token_program: Program<'info, Token>,
    #[account(mut)] pub from: Account<'info, TokenAccount>,
    #[account(mut)] pub to: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State849 {
    pub bump: u8,
    pub owner: Pubkey,
    pub storage: [u8; 128],
}
