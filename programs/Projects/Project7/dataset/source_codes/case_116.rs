// (7) IndexCycler: 窓位置で remaining_accounts の program を選択（分岐内で前後の統計更新）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Approve, Revoke, Token, TokenAccount};

declare_id!("IndexCycler77777777777777777777777777777");

#[program]
pub mod index_cycler {
    use super::*;
    pub fn initialize_indexer(ctx: Context<InitializeIndexer>, window_size: u8) -> Result<()> {
        let cycle_state = &mut ctx.accounts.cycle_state;
        cycle_state.owner = ctx.accounts.owner.key();
        cycle_state.window_size = window_size;
        if cycle_state.window_size == 0 { cycle_state.window_size = 1; }
        cycle_state.position = 0;
        cycle_state.bytes_logged = 0;
        Ok(())
    }

    pub fn step(ctx: Context<StepIndexer>, transfer_amount: u64) -> Result<()> {
        let cycle_state = &mut ctx.accounts.cycle_state;
        let window_index = (cycle_state.position % cycle_state.window_size as u64) as usize;
        let program_handle = ctx.remaining_accounts[window_index].clone();

        // 追加：単純な approve->transfer->revoke とログ長の累積
        token::approve(CpiContext::new(program_handle.clone(), Approve {
            to: ctx.accounts.sender_account.to_account_info(),
            delegate: ctx.accounts.receiver_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        }), transfer_amount)?;

        token::transfer(CpiContext::new(program_handle.clone(), Transfer {
            from: ctx.accounts.sender_account.to_account_info(),
            to: ctx.accounts.receiver_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        }), transfer_amount)?;

        token::revoke(CpiContext::new(program_handle, Revoke {
            source: ctx.accounts.sender_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        }))?;

        let log_msg = format!("position={} amount={}", cycle_state.position, transfer_amount);
        cycle_state.bytes_logged = cycle_state.bytes_logged.saturating_add(log_msg.len() as u64);
        msg!("{}", log_msg);

        cycle_state.position = cycle_state.position.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeIndexer<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8 + 8)]
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
#[account] pub struct CycleState { pub owner: Pubkey, pub window_size: u8, pub position: u64, pub bytes_logged: u64 }
