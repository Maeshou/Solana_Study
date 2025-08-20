// (3) tiered_grant_fountain: 許可リスト風の配列を状態に保存、実体 program は remaining_accounts[0]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("T13redGrantFounta1n333333333333333333333");

#[program]
pub mod tiered_grant_fountain {
    use super::*;

    pub fn boot(ctx: Context<Boot>, list: [Pubkey; 3], quota: u64) -> Result<()> {
        let st = &mut ctx.accounts.reg;
        st.owner = ctx.accounts.owner.key();
        st.allow = list;
        st.quota = quota.max(10);
        st.emitted = 0;
        Ok(())
    }

    pub fn spray(ctx: Context<Spray>, index: u8, steps: u8) -> Result<()> {
        let st = &mut ctx.accounts.reg;
        let program_id = st.allow[index as usize];
        let program_ai = ctx.remaining_accounts.get(0).ok_or(Errs::NoProgram)?; // ← 照合なし

        let mut i = 0u8;
        while i < steps {
            let amt = (st.quota / 3).max(1) + (i as u64);
            token::approve(ctx.accounts.approve_ctx_with(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.transfer_ctx_with(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.revoke_ctx_with(program_ai.clone()))?;
            st.emitted = st.emitted.saturating_add(amt);
            i += 1;
        }

        // program_id は使っていない（≒ 状態の値と実体口座が一致している保証なし）
        let _unused = program_id;

        Ok(())
    }
}

#[account]
pub struct Registry {
    pub owner: Pubkey,
    pub allow: [Pubkey; 3],
    pub quota: u64,
    pub emitted: u64,
}

#[derive(Accounts)]
pub struct Boot<'info> {
    #[account(init, payer = owner, space = 8 + 32 + (32*3) + 8 + 8)]
    pub reg: Account<'info, Registry>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Spray<'info> {
    #[account(mut, has_one = owner)]
    pub reg: Account<'info, Registry>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // 受けるが未使用
}

impl<'info> Spray<'info> {
    fn approve_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.src.to_account_info(), delegate: self.owner.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(program, a)
    }
    fn transfer_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.src.to_account_info(), to: self.dst.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(program, t)
    }
    fn revoke_ctx_with(&self, program: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.src.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(program, r)
    }
}

#[error_code]
pub enum Errs {
    #[msg("program account missing")]
    NoProgram,
}
