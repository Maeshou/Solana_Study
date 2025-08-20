// 3) caravan_toll_gate: 前半で動的CPI（徴収ログ）→中盤でSPL Token transfer（固定ID）→後半で別ロジック
// 分岐→動的CPI→固定IDtransfer→ループ の順
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("CaravanToll11111111111111111111111111111");

#[program]
pub mod caravan_toll_gate {
    use super::*;
    pub fn pass(ctx: Context<Pass>, toll: u64, stamp: u64) -> Result<()> {
        // 入口判定（分岐先行）
        if stamp & 1 == 1 {
            ctx.accounts.road.flags ^= stamp;
        }

        // ──【任意先CPI：動的 program_id】徴収ログ（先に外部へ）
        let mut logger = ctx.accounts.log_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            logger = ctx.remaining_accounts[0].clone(); // ← 差し替え可能
            ctx.accounts.road.routes += 1;
        }
        let lb = TollLogBridge { book: ctx.accounts.log_book.to_account_info(), user: ctx.accounts.traveler_wallet.to_account_info() };
        let cx = lb.as_cpi(logger.clone());
        lb.write(cx, stamp.to_le_bytes().to_vec())?;

        // ──【安全：固定ID】SPL Token transfer（通行料）
        let t = Transfer {
            from: ctx.accounts.traveler_token.to_account_info(),
            to: ctx.accounts.treasury_token.to_account_info(),
            authority: ctx.accounts.traveler_wallet.to_account_info(),
        };
        let tctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        token::transfer(tctx, toll)?;

        // 後半で別処理（ループ）
        for _ in 0..(toll % 3) {
            ctx.accounts.road.hash = ctx.accounts.road.hash.rotate_left(5);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Pass<'info> {
    #[account(mut)]
    pub road: Account<'info, RoadState>,

    // 任意先CPIライン
    /// CHECK:
    pub log_book: AccountInfo<'info>,
    /// CHECK:
    pub log_program: AccountInfo<'info>,

    // 安全ライン
    #[account(mut)]
    pub traveler_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub traveler_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RoadState { pub flags: u64, pub routes: u64, pub hash: u64 }

#[derive(Clone)]
pub struct TollLogBridge<'info> { pub book: AccountInfo<'info>, pub user: AccountInfo<'info> }
impl<'info> TollLogBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, TollLogBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.book.key, false), AccountMeta::new_readonly(*self.user.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.book.clone(), self.user.clone()] }
    pub fn write(&self, cx: CpiContext<'_, '_, '_, 'info, TollLogBridge<'info>>, d: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data: d };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
