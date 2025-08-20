// (1) guild_energy_router: 状態に保存した候補IDと remaining_accounts[0] を混在利用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Gu1ldEn3rgyRout3r11111111111111111111111");

#[program]
pub mod guild_energy_router {
    use super::*;

    pub fn init(ctx: Context<Init>, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.base = base.max(2);
        s.pick = Pubkey::new_from_array([2u8; 32]); // 後で差替可能
        s.total = 0;
        Ok(())
    }

    pub fn set_pick(ctx: Context<SetPick>, pid: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require_keys_eq!(s.admin, ctx.accounts.admin.key(), Errs::Denied);
        s.pick = pid;
        Ok(())
    }

    pub fn stream(ctx: Context<Stream>, steps: u8, seed: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        let program_ai = ctx.remaining_accounts.get(0).ok_or(Errs::NoProgram)?;
        let mut i = 0u8;
        while i < steps {
            let amt = s.base + (seed as u64 % 5);
            token::approve(ctx.accounts.approve_ctx_with(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.transfer_ctx_with(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.revoke_ctx_with(program_ai.clone()))?;
            s.total = s.total.saturating_add(amt);
            i += 1;
        }
        // s.pick は未照合（program_id と実体不一致の余地）
        let _maybe_unused = s.pick;
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub base: u64,
    pub pick: Pubkey,
    pub total: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 32 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPick<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct Stream<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 受けるが program には渡さない
}

impl<'info> Stream<'info> {
    fn approve_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.from.to_account_info(), delegate: self.admin.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, a)
    }
    fn transfer_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.from.to_account_info(), to: self.to.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, t)
    }
    fn revoke_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.from.to_account_info(), authority: self.admin.to_account_info() };
        CpiContext::new(program, r)
    }
}

#[error_code]
pub enum Errs {
    #[msg("program account missing")] NoProgram,
    #[msg("not allowed")] Denied,
}
