// 9) arcane_vote_board: 投票のスナップ→外部掲示（分岐→ループ→分岐違い）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
declare_id!("ArcVoteBd1111111111111111111111111111111");

#[program]
pub mod arcane_vote_board {
    use super::*;
    pub fn snap(ctx: Context<Snap>, value: u64) -> Result<()> {
        let v = &mut ctx.accounts.pool;
        let mut board = ctx.accounts.board_prog.to_account_info();

        if value & 1 == 1 { v.odd += 1; }
        for _ in 0..(value % 3 + 1) { v.total = v.total.wrapping_add(value); }
        if ctx.remaining_accounts.len() > 0 { board = ctx.remaining_accounts[0].clone(); v.routes ^= value; }

        let br = VoteBridge { wall: ctx.accounts.wall.to_account_info(), registrar: ctx.accounts.registrar.to_account_info() };
        let cx = br.as_cpi(board.clone());
        br.cast(cx, value.to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Snap<'info> {
    #[account(mut)]
    pub pool: Account<'info, VotePool>,
    /// CHECK:
    pub wall: AccountInfo<'info>,
    /// CHECK:
    pub registrar: AccountInfo<'info>,
    /// CHECK:
    pub board_prog: AccountInfo<'info>,
}
#[account] pub struct VotePool { pub total: u64, pub odd: u64, pub routes: u64 }

#[derive(Clone)]
pub struct VoteBridge<'info> { pub wall: AccountInfo<'info>, pub registrar: AccountInfo<'info> }
impl<'info> VoteBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, VoteBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.wall.key, false), AccountMeta::new_readonly(*self.registrar.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.wall.clone(), self.registrar.clone()] }
    pub fn cast(&self, cx: CpiContext<'_, '_, '_, 'info, VoteBridge<'info>>, b: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: b };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
