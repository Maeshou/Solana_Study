// 10) relic_cookbook_lab: レシピ進行・偶奇で分岐・外部保存（ループ→分岐→ループ→分岐）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
declare_id!("RelicCook1111111111111111111111111111111");

#[program]
pub mod relic_cookbook_lab {
    use super::*;
    pub fn step(ctx: Context<Step>, amt: u64) -> Result<()> {
        let r = &mut ctx.accounts.recipe;
        let mut store = ctx.accounts.store_prog.to_account_info();

        for _ in 0..(amt % 2 + 1) { r.meter = r.meter.wrapping_add(amt); }
        if r.meter & 1 == 0 { r.flags ^= Clock::get()?.slot; }
        for _ in 0..(amt % 3) { r.logs.push((amt as u32, (r.meter & 0xffff) as u32)); }
        if ctx.remaining_accounts.len() > 0 { store = ctx.remaining_accounts[0].clone(); r.paths += 1; }

        let br = StoreBridge { cabinet: ctx.accounts.cabinet.to_account_info(), author: ctx.accounts.author.to_account_info() };
        let cx = br.as_cpi(store.clone());
        br.save(cx, amt.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Step<'info> {
    #[account(mut)]
    pub recipe: Account<'info, Recipe>,
    /// CHECK:
    pub cabinet: AccountInfo<'info>,
    /// CHECK:
    pub author: AccountInfo<'info>,
    /// CHECK:
    pub store_prog: AccountInfo<'info>,
}
#[account] pub struct Recipe { pub meter: u64, pub flags: u64, pub paths: u64, pub logs: Vec<(u32,u32)> }

#[derive(Clone)]
pub struct StoreBridge<'info> { pub cabinet: AccountInfo<'info>, pub author: AccountInfo<'info> }
impl<'info> StoreBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, StoreBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.cabinet.key, false), AccountMeta::new_readonly(*self.author.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.cabinet.clone(), self.author.clone()] }
    pub fn save(&self, cx: CpiContext<'_, '_, '_, 'info, StoreBridge<'info>>, buf: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: buf };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
