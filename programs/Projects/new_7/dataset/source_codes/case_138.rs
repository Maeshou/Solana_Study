// 5) paint_contest_board: 色票集計→外部掲示更新（分岐→ループ→ループ）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{AccountMeta, Instruction}, program::invoke};
declare_id!("PaintCnts1111111111111111111111111111111");

#[program]
pub mod paint_contest_board {
    use super::*;
    pub fn submit(ctx: Context<Submit>, hue: u16) -> Result<()> {
        let c = &mut ctx.accounts.canvas;
        let mut pg = ctx.accounts.board_prog.to_account_info();

        if hue as u64 % 2 == 1 { c.odd += 1; c.pixels.push(hue as u32); }
        for _ in 0..(hue as u64 % 3) { c.layers = c.layers.wrapping_add(1); }
        for _ in 0..(hue as u64 % 2) { c.echo ^= hue as u64; }
        if ctx.remaining_accounts.len() > 0 { pg = ctx.remaining_accounts[0].clone(); }

        let br = PaintBridge { gallery: ctx.accounts.gallery.to_account_info(), artist: ctx.accounts.artist.to_account_info() };
        let cx = br.as_cpi(pg.clone());
        br.post(cx, (hue as u64).to_le_bytes().to_vec())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Submit<'info> {
    #[account(mut)]
    pub canvas: Account<'info, Canvas>,
    /// CHECK:
    pub gallery: AccountInfo<'info>,
    /// CHECK:
    pub artist: AccountInfo<'info>,
    /// CHECK:
    pub board_prog: AccountInfo<'info>,
}

#[account]
pub struct Canvas { pub layers: u64, pub odd: u64, pub echo: u64, pub pixels: Vec<u32> }

#[derive(Clone)]
pub struct PaintBridge<'info> { pub gallery: AccountInfo<'info>, pub artist: AccountInfo<'info> }
impl<'info> PaintBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, PaintBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.gallery.key, false), AccountMeta::new_readonly(*self.artist.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.gallery.clone(), self.artist.clone()] }
    pub fn post(&self, cx: CpiContext<'_, '_, '_, 'info, PaintBridge<'info>>, d: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: d };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
