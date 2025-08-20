// (7) IndexCycler: 周期位置に応じて remaining_accounts から program を選択
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("IndexCycler77777777777777777777777777777");

#[program]
pub mod index_cycler {
    use super::*;
    pub fn initialize_indexer(ctx: Context<InitializeIndexer>, window_size: u8) -> Result<()> {
        let cycle_state = &mut ctx.accounts.cycle_state;
        cycle_state.owner = ctx.accounts.owner.key();
        cycle_state.window_size = if window_size == 0 { 1 } else { window_size };
        cycle_state.position = 0;
        Ok(())
    }

    pub fn step(ctx: Context<StepIndexer>, transfer_amount: u64) -> Result<()> {
        let cycle_state = &mut ctx.accounts.cycle_state;
        let window_index = (cycle_state.position % cycle_state.window_size as u64) as usize;
        let program_handle = ctx.remaining_accounts[window_index].clone();

        token::transfer(
            CpiContext::new(program_handle, Transfer {
                from: ctx.accounts.sender_account.to_account_info(),
                to: ctx.accounts.receiver_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            }),
            transfer_amount,
        )?;
        cycle_state.position = cycle_state.position.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeIndexer<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8)]
    pub cycle_state: Account<'info, CycleState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StepIndexer<'info> {
    #[account(mut, has_one = owner)]
    pub cycle_state: Account<'info, CycleState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub sender_account: Account<'info, TokenAccount>,
    #[account(mut)] pub receiver_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account] pub struct CycleState { pub owner: Pubkey, pub window_size: u8, pub position: u64 }
