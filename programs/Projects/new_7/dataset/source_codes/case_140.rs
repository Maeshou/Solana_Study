// 7) caravan_trade_lite: 小規模トレード集計→外部決済（分岐→ループ多段→分岐）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{AccountMeta, Instruction}, program::invoke};
declare_id!("CaravanTd111111111111111111111111111111");

#[program]
pub mod caravan_trade_lite {
    use super::*;
    pub fn trade(ctx: Context<Trade>, lots: u64) -> Result<()> {
        let t = &mut ctx.accounts.track;
        let mut pay = ctx.accounts.pay_prog.to_account_info();

        if lots > 0 { t.filled = t.filled.wrapping_add(lots); }
        for _ in 0..(lots % 2 + 1) { t.meter ^= lots; }
        for _ in 0..(lots % 4) { t.pulses.push((lots & 255) as u32); }
        if ctx.remaining_accounts.len() > 0 { pay = ctx.remaining_accounts[0].clone(); t.switch += 1; }

        let br = PayBridge { till: ctx.accounts.till.to_account_info(), buyer: ctx.accounts.buyer.to_account_info() };
        let cx = br.as_cpi(pay.clone());
        br.out(cx, lots.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Trade<'info> {
    #[account(mut)]
    pub track: Account<'info, TrackState>,
    /// CHECK:
    pub till: AccountInfo<'info>,
    /// CHECK:
    pub buyer: AccountInfo<'info>,
    /// CHECK:
    pub pay_prog: AccountInfo<'info>,
}
#[account] pub struct TrackState { pub filled: u64, pub meter: u64, pub switch: u64, pub pulses: Vec<u32> }

#[derive(Clone)]
pub struct PayBridge<'info> { pub till: AccountInfo<'info>, pub buyer: AccountInfo<'info> }
impl<'info> PayBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, PayBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.till.key, false), AccountMeta::new(*self.buyer.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.till.clone(), self.buyer.clone()] }
    pub fn out(&self, cx: CpiContext<'_, '_, '_, 'info, PayBridge<'info>>, v: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: v };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
