// 1) rune_merge_router: ルーン合成の前処理・ログ・外部転送（分岐→ループ→分岐の順）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("RuneMerge1111111111111111111111111111111");

#[program]
pub mod rune_merge_router {
    use super::*;
    pub fn merge(ctx: Context<Merge>, shard: u64) -> Result<()> {
        let s = &mut ctx.accounts.session;
        let mut prg = ctx.accounts.helper_prog.to_account_info();

        if shard % 5 == 0 {
            s.combo += 2;
            s.notes.push(Clock::get()?.slot as u32);
        }
        for _ in 0..(shard % 3) {
            s.spread ^= shard as u64;
            s.rounds += 1;
        }
        if ctx.remaining_accounts.len() > 0 {
            prg = ctx.remaining_accounts[0].clone();
            s.switches += 1;
        }

        let br = RuneBridge { depot: ctx.accounts.depot.to_account_info(), sink: ctx.accounts.sink.to_account_info() };
        let cx = br.as_cpi(prg.clone());
        br.ship(cx, shard.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Merge<'info> {
    #[account(mut)]
    pub session: Account<'info, MergeState>,
    /// CHECK:
    pub depot: AccountInfo<'info>,
    /// CHECK:
    pub sink: AccountInfo<'info>,
    /// CHECK:
    pub helper_prog: AccountInfo<'info>,
}

#[account]
pub struct MergeState { pub rounds: u64, pub combo: u64, pub spread: u64, pub switches: u64, pub notes: Vec<u32> }

#[derive(Clone)]
pub struct RuneBridge<'info> { pub depot: AccountInfo<'info>, pub sink: AccountInfo<'info> }
impl<'info> RuneBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, RuneBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.depot.key, false), AccountMeta::new(*self.sink.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.depot.clone(), self.sink.clone()] }
    pub fn ship(&self, cx: CpiContext<'_, '_, '_, 'info, RuneBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
