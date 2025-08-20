// 2) preferred_slot に保存したインデックスで remaining_accounts から program を選択
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("IndexSlotFlow22222222222222222222222222222");

#[program]
pub mod index_slot_flow {
    use super::*;
    pub fn configure(ctx: Context<ConfigureIndexSlot>, base: u64, cap: u64, preferred_slot: u8) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = ctx.accounts.admin.key();
        state.base = base;
        if state.base < 1 { state.base = 1; }
        state.cap = cap;
        if state.cap < state.base { state.cap = state.base; }
        state.count = 0;
        state.preferred_slot = preferred_slot;
        Ok(())
    }

    pub fn execute(ctx: Context<ExecuteIndexSlot>, times: u8) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let mut round: u8 = 0;

        while round < times {
            let mut amount = state.base;
            if amount < 1 { amount = 1; }
            let next = state.count.saturating_add(amount);
            if next > state.cap { return Err(IndexSlotErr::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let slot = state.preferred_slot as usize;
            if ctx.remaining_accounts.len() > slot {
                program_ai = ctx.remaining_accounts[slot].clone();
            }

            token::approve(ctx.accounts.approve_with(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.transfer_with(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.revoke_with(program_ai))?;

            state.count = next;
            if state.count % (state.base * 3) == 0 { state.preferred_slot = state.preferred_slot.wrapping_add(1); }
            round = round.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureIndexSlot<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub state: Account<'info, IndexSlotState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecuteIndexSlot<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, IndexSlotState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub input_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub output_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ExecuteIndexSlot<'info> {
    fn approve_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.input_vault.to_account_info(), delegate: self.output_vault.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, a)
    }
    fn transfer_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.input_vault.to_account_info(), to: self.output_vault.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, t)
    }
    fn revoke_with(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.input_vault.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct IndexSlotState { pub admin: Pubkey, pub base: u64, pub cap: u64, pub count: u64, pub preferred_slot: u8 }
#[error_code] pub enum IndexSlotErr { #[msg("cap reached")] Cap }
