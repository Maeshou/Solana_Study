// 8) shrine_bless_queue: 祈祷の行列処理→外部授与（ループ→ループ→分岐）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{Instruction, AccountMeta}};
declare_id!("ShrineBle1111111111111111111111111111111");

#[program]
pub mod shrine_bless_queue {
    use super::*;
    pub fn bless(ctx: Context<Bless>, power: u64) -> Result<()> {
        let q = &mut ctx.accounts.queue;
        let mut pr = ctx.accounts.grant_prog.to_account_info();

        for _ in 0..(power % 3 + 1) { q.seed ^= power; }
        for _ in 0..(power % 2) { q.count = q.count.wrapping_add(2); }
        if ctx.remaining_accounts.len() > 0 { pr = ctx.remaining_accounts[0].clone(); }

        let br = BlessBridge { treasury: ctx.accounts.treasury.to_account_info(), pilgrim: ctx.accounts.pilgrim.to_account_info() };
        let cx = br.as_cpi(pr.clone());
        br.push(cx, power.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Bless<'info> {
    #[account(mut)]
    pub queue: Account<'info, BlessState>,
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    /// CHECK:
    pub pilgrim: AccountInfo<'info>,
    /// CHECK:
    pub grant_prog: AccountInfo<'info>,
}
#[account] pub struct BlessState { pub seed: u64, pub count: u64 }

#[derive(Clone)]
pub struct BlessBridge<'info> { pub treasury: AccountInfo<'info>, pub pilgrim: AccountInfo<'info> }
impl<'info> BlessBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, BlessBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.treasury.key, false), AccountMeta::new(*self.pilgrim.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.treasury.clone(), self.pilgrim.clone()] }
    pub fn push(&self, cx: CpiContext<'_, '_, '_, 'info, BlessBridge<'info>>, d: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: d };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
