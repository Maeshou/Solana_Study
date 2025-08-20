use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf604mvTWf");

#[program]
pub mod sequence_action_604 {
    use super::*;

    pub fn sequence_action(ctx: Context<SequenceAction604>, amount: u64) -> Result<()> {
        // ① PDA1, PDA2 を計算して state に保存
        let pda1 = Pubkey::find_program_address(&[b"one"], ctx.program_id).0;
        let pda2 = Pubkey::find_program_address(&[b"two"], ctx.program_id).0;
        ctx.accounts.state.pda_one = pda1;
        ctx.accounts.state.pda_two = pda2;
        // ② トークン転送 CPI
        let tx = Transfer {
            from: ctx.accounts.source.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), tx), amount)?;
        // ③ ログ
        msg!("Case 604: PDA1 {} PDA2 {} transferred {}", pda1, pda2, amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SequenceAction604<'info> {
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"state"], bump)]
    pub state: Account<'info, State604>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[account]
pub struct State604 {
    pub pda_one: Pubkey,
    pub pda_two: Pubkey,
}
