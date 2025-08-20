// 1) time_slicer: スロット時間に応じてチャンク長を変化させる
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("TimeSlicer111111111111111111111111111111");

#[program]
pub mod time_slicer {
    use super::*;
    pub fn slice(ctx: Context<Slice>, total: u64) -> Result<()> {
        let st = &mut ctx.accounts.slice_state;
        st.invokes += 1;

        let mut program = ctx.accounts.default_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            st.branch_a += total;
            program = ctx.remaining_accounts[0].clone();
        } else {
            st.branch_b += total;
        }

        let now_slot = Clock::get()?.slot;
        let step = ((now_slot & 7) + 3) as u64;
        let mut rest = total;
        let br = TimeBridge { a: ctx.accounts.buf_a.to_account_info(), b: ctx.accounts.buf_b.to_account_info() };

        let mut round = 0u64;
        while rest > 0 {
            let size = if rest > step + round { step + round } else { rest };
            let cx = br.as_cpi(program.clone());
            br.kick(cx, size + st.invokes)?;
            rest -= size;
            round += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Slice<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8 + 8)]
    pub slice_state: Account<'info, SliceState>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub buf_a: AccountInfo<'info>,
    /// CHECK:
    pub buf_b: AccountInfo<'info>,
    /// CHECK:
    pub default_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct SliceState { pub invokes: u64, pub branch_a: u64, pub branch_b: u64 }
#[derive(Clone)] pub struct TimeBridge<'info> { pub a: AccountInfo<'info>, pub b: AccountInfo<'info> }
impl<'info> TimeBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, TimeBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.a.key, false), AccountMeta::new_readonly(*self.b.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.a.clone(), self.b.clone()] }
    pub fn kick(&self, ctx: CpiContext<'_, '_, '_, 'info, TimeBridge<'info>>, n: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas(), data: n.to_le_bytes().to_vec() };
        invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
