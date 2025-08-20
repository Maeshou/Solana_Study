// 1) 状態フラグ use_alt による切替：データ/トークンは Account<T>、program だけ AccountInfo
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("AcctOkArbCpi01AAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod acct_ok_arb_cpi_01 {
    use super::*;
    pub fn init(ctx: Context<Init01>, unit: u64, cap: u64, use_alt: bool) -> Result<()> {
        let st = &mut ctx.accounts.state;
        st.admin = ctx.accounts.admin.key();
        st.unit = unit.max(1);
        st.cap = cap.max(st.unit);
        st.sent = 0;
        st.use_alt = use_alt;
        Ok(())
    }
    pub fn run(ctx: Context<Run01>, n: u8) -> Result<()> {
        let st = &mut ctx.accounts.state;
        let mut i = 0u8;
        while i < n {
            let amt = st.unit;
            let next = st.sent.saturating_add(amt);
            if next > st.cap { return Err(E01::Cap.into()); }

            let mut program_ai = ctx.accounts.token_program.to_account_info();
            if st.use_alt { program_ai = ctx.accounts.alt_program.clone(); } // ← 差替ポイント

            token::approve(ctx.accounts.approve_ctx(program_ai.clone()), amt)?;
            token::transfer(ctx.accounts.transfer_ctx(program_ai.clone()), amt)?;
            token::revoke(ctx.accounts.revoke_ctx(program_ai))?;

            st.sent = next;
            if st.sent % (st.unit * 3) == 0 { st.use_alt = !st.use_alt; }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init01<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub state: Account<'info, State01>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Run01<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, State01>,
    pub admin: Signer<'info>,
    #[account(mut)] pub src: Account<'info, TokenAccount>,
    #[account(mut)] pub dst: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub alt_program: AccountInfo<'info>,                    // ← AccountInfo
}
impl<'info> Run01<'info> {
    fn approve_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> {
        CpiContext::new(p, Approve{ to: self.src.to_account_info(), delegate: self.dst.to_account_info(), authority: self.admin.to_account_info() })
    }
    fn transfer_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(p, Transfer{ from: self.src.to_account_info(), to: self.dst.to_account_info(), authority: self.admin.to_account_info() })
    }
    fn revoke_ctx(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> {
        CpiContext::new(p, Revoke{ source: self.src.to_account_info(), authority: self.admin.to_account_info() })
    }
}
#[account] pub struct State01 { pub admin: Pubkey, pub unit: u64, pub cap: u64, pub sent: u64, pub use_alt: bool }
#[error_code] pub enum E01 { #[msg("cap reached")] Cap }
