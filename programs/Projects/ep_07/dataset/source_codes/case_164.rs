// 3) EpochModuloRouter: 現在スロットの剰余が一致したときだけ外部プログラムを採用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("EpochModulo33333333333333333333333333333");

#[program]
pub mod epoch_modulo_router {
    use super::*;
    pub fn configure(ctx: Context<ConfigureModulo>, base_amount: u64, cap_total: u64, modulo_value: u64, trigger_remainder: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = ctx.accounts.admin.key();
        state.base_amount = base_amount.max(1);
        state.cap_total = cap_total.max(state.base_amount);
        state.modulo_value = modulo_value.max(1);
        state.trigger_remainder = trigger_remainder % state.modulo_value;
        state.total_done = 0;
        Ok(())
    }
    pub fn step(ctx: Context<StepModulo>, cycles: u8) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let current_slot = Clock::get()?.slot;
        let mut i: u8 = 0;

        while i < cycles {
            let next = state.total_done.saturating_add(state.base_amount);
            if next > state.cap_total { return Err(ModErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            let remainder = (current_slot % state.modulo_value) as u64;
            if remainder == state.trigger_remainder { program_account_info = ctx.accounts.alt_path.clone(); }

            token::approve(ctx.accounts.a(program_account_info.clone()), state.base_amount)?;
            token::transfer(ctx.accounts.t(program_account_info.clone()), state.base_amount)?;
            token::revoke(ctx.accounts.r(program_account_info))?;

            state.total_done = next;
            i = i.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct ConfigureModulo<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 8 + 8)]
    pub state: Account<'info, ModuloState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct StepModulo<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, ModuloState>,
    pub admin: Signer<'info>,
    #[account(mut)] pub tank_from: Account<'info, TokenAccount>,
    #[account(mut)] pub tank_to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub alt_path: AccountInfo<'info>,
}
impl<'info> StepModulo<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> { 
        CpiContext::new(p, Approve { to: self.tank_from.to_account_info(), delegate: self.tank_to.to_account_info(), authority: self.admin.to_account_info() })
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> { 
        CpiContext::new(p, Transfer { from: self.tank_from.to_account_info(), to: self.tank_to.to_account_info(), authority: self.admin.to_account_info() })
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> { 
        CpiContext::new(p, Revoke { source: self.tank_from.to_account_info(), authority: self.admin.to_account_info() })
    }
}
#[account] pub struct ModuloState { pub admin: Pubkey, pub base_amount: u64, pub cap_total: u64, pub modulo_value: u64, pub trigger_remainder: u64, pub total_done: u64 }
#[error_code] pub enum ModErr { #[msg("cap reached")] Cap }
