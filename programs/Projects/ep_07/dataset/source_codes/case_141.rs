// 7) 保存した hint_program_key に合致する remaining_accounts を優先採用（緩い突合）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("HintLookup7777777777777777777777777777777");

#[program]
pub mod hint_lookup_selector {
    use super::*;
    pub fn register(ctx: Context<RegisterHint>, unit: u64, cap: u64, hint_program_key: Pubkey) -> Result<()> {
        let h = &mut ctx.accounts.hub;
        h.manager = ctx.accounts.manager.key();
        h.unit = unit;
        if h.unit < 1 { h.unit = 1; }
        h.cap = cap;
        if h.cap < h.unit { h.cap = h.unit; }
        h.total = 0;
        h.hint_program_key = hint_program_key;
        Ok(())
    }

    pub fn execute(ctx: Context<ExecuteHint>, loops: u8) -> Result<()> {
        let h = &mut ctx.accounts.hub;
        let mut i: u8 = 0;

        while i < loops {
            let mut amount = h.unit;
            if amount < 1 { amount = 1; }
            let planned = h.total.saturating_add(amount);
            if planned > h.cap { return Err(HintErr::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            let mut idx: usize = 0;
            while idx < ctx.remaining_accounts.len() {
                let candidate = &ctx.remaining_accounts[idx];
                if candidate.key() == h.hint_program_key { program_ai = candidate.clone(); break; }
                idx = idx.saturating_add(1);
            }

            token::approve(ctx.accounts.ap(program_ai.clone()), amount)?;
            token::transfer(ctx.accounts.tr(program_ai.clone()), amount)?;
            token::revoke(ctx.accounts.rv(program_ai))?;

            h.total = planned;
            if h.total % (h.unit * 5) == 0 { h.hint_program_key = h.hint_program_key; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterHint<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 32)]
    pub hub: Account<'info, HintState>,
    #[account(mut)] pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ExecuteHint<'info> {
    #[account(mut, has_one = manager)]
    pub hub: Account<'info, HintState>,
    pub manager: Signer<'info>,
    #[account(mut)] pub left_store: Account<'info, TokenAccount>,
    #[account(mut)] pub right_store: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ExecuteHint<'info> {
    fn ap(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.left_store.to_account_info(), delegate: self.right_store.to_account_info(), authority: self.manager.to_account_info() };
        CpiContext::new(p, a)
    }
    fn tr(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.left_store.to_account_info(), to: self.right_store.to_account_info(), authority: self.manager.to_account_info() };
        CpiContext::new(p, t)
    }
    fn rv(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.left_store.to_account_info(), authority: self.manager.to_account_info() };
        CpiContext::new(p, r)
    }
}
#[account] pub struct HintState { pub manager: Pubkey, pub unit: u64, pub cap: u64, pub total: u64, pub hint_program_key: Pubkey }
#[error_code] pub enum HintErr { #[msg("cap reached")] Cap }
