use anchor_lang::prelude::*;
use anchor_spl::associated_token::create as atc;
use anchor_spl::memo::post;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf663mvTWf");

#[program]
pub mod trigger_sequence_663 {
    use super::*;

    pub fn trigger_sequence(ctx: Context<TriggerSequence663>, memo: String) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.bump = *ctx.bumps.get("state").unwrap();
        atc(ctx.accounts.into());
        post(ctx.accounts.memo_prog.to_account_info(), memo.clone())?;
        msg!("Case 663: bump {} memo '{}'", state.bump, memo);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TriggerSequence663<'info> {
    #[account(address=anchor_spl::memo::ID)] pub memo_prog: Program<'info, Memo>,
    #[account(address=anchor_spl::associated_token::ID)] pub ata_prog: Program<'info, anchor_spl::token::Token>,
    #[account(init,payer=user,seeds=[b"state",user.key().as_ref()],bump,space=8+1)] pub state: Account<'info, State663>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct State663 {
    pub bump: u8,
}
