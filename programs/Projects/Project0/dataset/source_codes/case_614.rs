use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf614mvTWf");

#[program]
pub mod sequence_action_614 {
    use super::*;

    pub fn sequence_action(ctx: Context<SequenceAction614>, amount: u64) -> Result<()> {
        let pda1 = Pubkey::find_program_address(&[b"one"], ctx.program_id).0;
        let pda2 = Pubkey::find_program_address(&[b"two"], ctx.program_id).0;
        ctx.accounts.state.a = pda1;
        ctx.accounts.state.b = pda2;
        let tx = Transfer {
            from: ctx.accounts.src.to_account_info(),
            to: ctx.accounts.dest.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        transfer(CpiContext::new(ctx.accounts.token_prog.to_account_info(), tx), amount)?;
        msg!(
            "Case 614: PDA1 {} PDA2 {} sent {}",
            pda1, pda2, amount
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SequenceAction614<'info> {
    #[account(address=token::ID)] pub token_prog: Program<'info, Token>,
    #[account(mut)] pub src: Account<'info, TokenAccount>,
    #[account(mut)] pub dest: Account<'info, TokenAccount>,
    #[account(mut,seeds=[b"state"],bump)] pub state: Account<'info, State614>,
    #[account(signer)] pub user: Signer<'info>,
}

#[account]
pub struct State614 {
    pub a: Pubkey,
    pub b: Pubkey,
}
