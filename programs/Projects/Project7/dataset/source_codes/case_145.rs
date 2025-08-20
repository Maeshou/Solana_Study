// 1) flipトグルで program を切替（状態フラグをループ内で反転）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("FlipDrip1111111111111111111111111111111111");

#[program]
pub mod flip_drip {
    use super::*;
    pub fn setup(ctx: Context<SetupFlip>, unit: u64, cap: u64) -> Result<()> {
        let st = &mut ctx.accounts.station;
        st.owner = ctx.accounts.owner.key();
        st.unit = if unit < 1 { 1 } else { unit };
        st.cap = if cap < st.unit { st.unit } else { cap };
        st.today = 0;
        st.flip = false;
        Ok(())
    }
    pub fn run(ctx: Context<RunFlip>, loops: u8) -> Result<()> {
        let st = &mut ctx.accounts.station;
        let mut n: u8 = 0;
        while n < loops {
            let mut amt = st.unit;
            if amt < 1 { amt = 1; }
            let sum = st.today.saturating_add(amt);
            if sum > st.cap { return Err(FlipErr::Cap.into()); }
            let program_ai = if st.flip {
                ctx.accounts.alt_prog.to_account_info()
            } else {
                ctx.accounts.token_program.to_account_info()
            };
            token::approve(ctx.accounts.ctx_approve(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.ctx_transfer(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.ctx_revoke(program_ai))?;
            st.today = sum;
            if st.today % (st.unit * 3) == 0 { st.flip = !st.flip; }
            n = n.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupFlip<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub station: Account<'info, FlipState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RunFlip<'info> {
    #[account(mut, has_one = owner)]
    pub station: Account<'info, FlipState>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub from_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_vault: Account<'info, TokenAccount>,
    /// CHECK: 外部プログラム候補
    pub alt_prog: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> RunFlip<'info> {
    fn ctx_approve(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        let a = Approve { to: self.from_vault.to_account_info(), delegate: self.to_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(program_ai, a)
    }
    fn ctx_transfer(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.from_vault.to_account_info(), to: self.to_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(program_ai, t)
    }
    fn ctx_revoke(&self, program_ai: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        let r = Revoke { source: self.from_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(program_ai, r)
    }
}
#[account] pub struct FlipState { pub owner: Pubkey, pub unit: u64, pub cap: u64, pub today: u64, pub flip: bool }
#[error_code] pub enum FlipErr { #[msg("cap reached")] Cap }
