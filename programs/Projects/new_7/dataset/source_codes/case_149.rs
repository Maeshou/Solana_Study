// 1) crest_drop_station: 途中で動的CPI→最後に SPL Token transfer（固定ID）
// 分岐→動的CPI→内部計算→固定IDtransfer→ループ
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("CrestDropSt11111111111111111111111111111");

#[program]
pub mod crest_drop_station {
    use super::*;
    pub fn drop_and_pay(ctx: Context<DropAndPay>, crest: u64, pay: u64) -> Result<()> {
        if crest % 2 == 0 { ctx.accounts.meta.even_count += 1; }

        // ──動的CPI（通知ライン）
        let mut router = ctx.accounts.notify_program.to_account_info();
        if ctx.remaining_accounts.len() > 0 {
            router = ctx.remaining_accounts[0].clone();
            ctx.accounts.meta.paths += 1;
        }
        let nb = NoticeBridge {
            board: ctx.accounts.notice_board.to_account_info(),
            actor: ctx.accounts.receiver_wallet.to_account_info(),
        };
        let cx = nb.as_cpi(router.clone());
        nb.post(cx, crest.to_le_bytes().to_vec())?;

        // 内部計算
        ctx.accounts.meta.hash ^= crest.rotate_left(5);

        // ──固定ID：SPL Token transfer
        let t = Transfer {
            from: ctx.accounts.treasury_token.to_account_info(),
            to: ctx.accounts.receiver_token.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        };
        let tctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), t);
        token::transfer(tctx, pay)?;

        // 仕上げに軽いループ
        for _ in 0..(pay % 3) { ctx.accounts.meta.bump = ctx.accounts.meta.bump.wrapping_add(1); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DropAndPay<'info> {
    #[account(mut)]
    pub meta: Account<'info, DropMeta>,

    // 動的CPIライン
    /// CHECK:
    pub notice_board: AccountInfo<'info>,
    /// CHECK:
    pub receiver_wallet: AccountInfo<'info>,
    /// CHECK:
    pub notify_program: AccountInfo<'info>,

    // 固定ID（SPL Token）
    #[account(mut)]
    pub treasury_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_token: Account<'info, TokenAccount>,
    /// CHECK:
    pub treasury_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct DropMeta { pub even_count: u64, pub paths: u64, pub hash: u64, pub bump: u64 }

#[derive(Clone)]
pub struct NoticeBridge<'info> { pub board: AccountInfo<'info>, pub actor: AccountInfo<'info> }
impl<'info> NoticeBridge<'info> {
    pub fn as_cpi(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, NoticeBridge<'info>> { CpiContext::new(p, self.clone()) }
    fn metas(&self) -> Vec<AccountMeta> { vec![AccountMeta::new(*self.board.key, false), AccountMeta::new_readonly(*self.actor.key, false)] }
    fn infos(&self, p:&AccountInfo<'info>) -> Vec<AccountInfo<'info>> { vec![p.clone(), self.board.clone(), self.actor.clone()] }
    pub fn post(&self, cx: CpiContext<'_, '_, '_, 'info, NoticeBridge<'info>>, data: Vec<u8>) -> Result<()> {
        let ix = Instruction { program_id: *cx.program.key, accounts: self.metas(), data };
        invoke(&ix, &self.infos(&cx.program))?;
        Ok(())
    }
}
