// 2) ring_buffer_bus: リングバッファ風にオフセット回し、偶数回でメタ順序を入れ替え
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("RingBus111111111111111111111111111111111");

#[program]
pub mod ring_buffer_bus {
    use super::*;
    pub fn drive(ctx: Context<Drive>, load: u64) -> Result<()> {
        let s = &mut ctx.accounts.bus;
        s.turn += 1;

        let mut program = ctx.accounts.route_prog.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            s.path_a += load;
            program = ctx.remaining_accounts[0].clone();
        } else {
            s.path_b += load;
        }

        let offset = (s.turn & 3) as u64 + 1;
        let br = BusBridge { head: ctx.accounts.head_buf.to_account_info(), tail: ctx.accounts.tail_buf.to_account_info() };

        let mut sent = 0u64;
        while sent < load {
            let send = if load - sent > offset { offset } else { load - sent };
            let cx = br.as_cpi(program.clone());
            if (s.turn & 1) > 0 {
                br.push_ht(cx, send + s.turn)?;
            } else {
                br.push_th(cx, send + s.turn)?;
            }
            sent += send;
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Drive<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8 + 8)]
    pub bus: Account<'info, BusState>,
    #[account(mut)] pub payer: Signer<'info>,
    /// CHECK:
    pub head_buf: AccountInfo<'info>,
    /// CHECK:
    pub tail_buf: AccountInfo<'info>,
    /// CHECK:
    pub route_prog: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct BusState { pub turn: u64, pub path_a: u64, pub path_b: u64 }
#[derive(Clone)] pub struct BusBridge<'info> { pub head: AccountInfo<'info>, pub tail: AccountInfo<'info> }
impl<'info> BusBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, BusBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas_ht(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.head.key, false), AccountMeta::new(*self.tail.key, false)] }
    fn metas_th(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.tail.key, false), AccountMeta::new(*self.head.key, false)] }
    fn infos(&self, p: &AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.head.clone(), self.tail.clone()] }
    pub fn push_ht(&self, ctx: CpiContext<'_, '_, '_, 'info, BusBridge<'info>>, v: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas_ht(), data: v.to_le_bytes().to_vec() }; invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
    pub fn push_th(&self, ctx: CpiContext<'_, '_, '_, 'info, BusBridge<'info>>, v: u64) -> Result<()> {
        let ix = Instruction { program_id: *ctx.program.key, accounts: self.metas_th(), data: v.to_le_bytes().to_vec() }; invoke(&ix, &self.infos(&ctx.program))?; Ok(())
    }
}
