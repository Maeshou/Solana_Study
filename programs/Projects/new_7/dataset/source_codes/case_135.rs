// 2) guild_title_board: 称号掲示の更新と外部通知（ループ→分岐→ループの順）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("GuildTitle111111111111111111111111111111");

#[program]
pub mod guild_title_board {
    use super::*;
    pub fn post(ctx: Context<Post>, fame: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        let mut router = ctx.accounts.notify_prog.to_account_info();

        for _ in 0..(fame % 2 + 1) {
            b.tick ^= fame;
            b.lines += 1;
        }
        if b.lines > 3 {
            b.flags += 1;
            b.history.push((b.lines as u32, (fame & 0xffff) as u32));
        }
        if ctx.remaining_accounts.len() > 0 {
            router = ctx.remaining_accounts[0].clone();
            b.paths += 1;
        }

        let br = TitleBridge { wall: ctx.accounts.wall.to_account_info(), author: ctx.accounts.author.to_account_info() };
        let cx = br.as_cpi(router.clone());
        br.emit(cx, fame.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Post<'info> {
    #[account(mut)]
    pub board: Account<'info, TitleState>,
    /// CHECK:
    pub wall: AccountInfo<'info>,
    /// CHECK:
    pub author: AccountInfo<'info>,
    /// CHECK:
    pub notify_prog: AccountInfo<'info>,
}

#[account]
pub struct TitleState { pub lines: u64, pub flags: u64, pub tick: u64, pub paths: u64, pub history: Vec<(u32,u32)> }

#[derive(Clone)]
pub struct TitleBridge<'info> { pub wall: AccountInfo<'info>, pub author: AccountInfo<'info> }
impl<'info> TitleBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, TitleBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.wall.key, false), AccountMeta::new_readonly(*self.author.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.wall.clone(), self.author.clone()] }
    pub fn emit(&self, cx: CpiContext<'_, '_, '_, 'info, TitleBridge<'info>>, payload: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: payload };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
