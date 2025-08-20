// 3) relic_exchange_kiosk: 遺物交換前の集計と外部転送（分岐→分岐→ループ）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
declare_id!("RelicXchg1111111111111111111111111111111");

#[program]
pub mod relic_exchange_kiosk {
    use super::*;
    pub fn exchange(ctx: Context<Exchange>, count: u64) -> Result<()> {
        let k = &mut ctx.accounts.kiosk;
        let mut pg = ctx.accounts.exchange_prog.to_account_info();

        if count > 7 { k.queue += 2; }
        if ctx.remaining_accounts.len() > 1 {
            pg = ctx.remaining_accounts[1].clone();
            k.sides ^= count;
        }
        for _ in 0..(count % 4) {
            k.batch = k.batch.wrapping_add(1);
        }

        let br = RelicBridge { store: ctx.accounts.store.to_account_info(), user: ctx.accounts.user.to_account_info() };
        let cx = br.as_cpi(pg.clone());
        br.route(cx, count.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub kiosk: Account<'info, Kiosk>,
    /// CHECK:
    pub store: AccountInfo<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// CHECK:
    pub exchange_prog: AccountInfo<'info>,
}

#[account]
pub struct Kiosk { pub queue: u64, pub sides: u64, pub batch: u64 }

#[derive(Clone)]
pub struct RelicBridge<'info> { pub store: AccountInfo<'info>, pub user: AccountInfo<'info> }
impl<'info> RelicBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, RelicBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.store.key, false), AccountMeta::new(*self.user.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.store.clone(), self.user.clone()] }
    pub fn route(&self, cx: CpiContext<'_, '_, '_, 'info, RelicBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
